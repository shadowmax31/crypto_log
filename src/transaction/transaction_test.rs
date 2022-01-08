use std::ops::Add;

use chrono::{DateTime, Duration, Local, Datelike};
use rental_rod::db::{Db, table::Table};
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::{report::{cost_basis::{CostBasis}, capital_gain::CapitalGain}, util::{config::Config, helper::sort_by_date}};

use super::Transaction;

trait DateToLocal {
    fn to_local_str(&self, config: &Config) -> String;
    fn inc(self) -> Self;
}

impl DateToLocal for DateTime<Local> {
    fn to_local_str(&self, config: &Config) -> String {
        self.format(&config.date_format().unwrap()).to_string()
    }
    
    fn inc(self) -> Self {
        let min = Duration::minutes(1);
        self.add(min)
    }
}

#[test]
fn test_buy() {
    let mut db = init_db("/tmp/transaction_test_buy", true);
    let config = Config::new().unwrap();
    
    let date = Local::now();
    
    let transaction = Transaction::new(&mut db, &config);
    transaction
    .buy(
        &date.to_local_str(&config),
        "1",
        "btc",
        "10000",
        "Description",
    )
    .unwrap();
    
    // Validate the cost basis
    let cost = CostBasis::new(&config);
    let value = cost
    .calculate(&transaction.table("btc").unwrap(), None, false)
    .unwrap()
    .unwrap();
    assert_eq!(value.to_string(), "10000");
    
    let date = date.inc();
    transaction
    .buy(
        &date.to_local_str(&config),
        "0.5",
        "btc",
        "20000",
        "Description",
    )
    .unwrap();
    
    // Validate the cost basis
    let value = cost
    .calculate(&transaction.table("btc").unwrap(), None, false)
    .unwrap()
    .unwrap();
    assert_eq!(value.to_string(), "20000");
    
    // Test add the same date twice
    transaction
    .buy(
        &date.to_local_str(&config),
        "0.2",
        "btc",
        "20000",
        "Description",
    )
    .unwrap();
    let value = cost
    .calculate(&transaction.table("btc").unwrap(), None, false)
    .unwrap()
    .unwrap();
    // The cost basis should not have changed
    assert_eq!(value.to_string(), "20000");
}

#[test]
fn test_buy_sell() {
    let path = "/tmp/transaction_test_buy_sell";
    let mut db = init_db(path, true);
    let config = Config::new().unwrap();
    
    let transaction = Transaction::new(&mut db, &config);
    let date = init_buy(&transaction, &config, Local::now());
    
    let cost = CostBasis::new(&config);
    let initial_cost_basis = cost.calculate(&transaction.table("btc").unwrap(), None, false).unwrap().unwrap();
    
    // Validate cost basis
    let cost_eth = cost.calculate(&transaction.table("eth").unwrap(), None, false).unwrap().unwrap();
    assert_eq!(cost_eth.to_string(), "2500");
    
    // Test the transaction ordering by date (adds a transaction at the end of the list)
    let later = date.add(Duration::hours(2));
    transaction.buy(&later.to_local_str(&config), "1", "btc", "1000", "Description").unwrap();
    let cost_with_later_transaction = cost.calculate(&transaction.table("btc").unwrap(), None, false).unwrap().unwrap();
    assert_eq!(cost_with_later_transaction.to_string(), "12400");
    
    // When you sell, the cost basis should not change
    let date = date.inc();
    let sell_id = transaction.sell(&date.to_local_str(&config), "0.25", "btc", "12000", "Description").unwrap().unwrap();
    
    let new_cost_basis = cost.calculate(&transaction.table("btc").unwrap(), Some(&sell_id), false).unwrap().unwrap(); // The transaction is not the last one
    assert_eq!(initial_cost_basis, new_cost_basis);
    assert_eq!(new_cost_basis.to_string(), "20000");
    // END - Sell test
    
    // Test the capital gain (with ordering by date). The buy transaction in 2 hours is ignored
    // Because the taxable event (sell) happened before
    let db = init_db(path, false);
    let gain = return_capital_gain(&db, &config);
    assert_eq!(gain.to_string(), "7000.00");
    
    // Test sell the same date twice
    transaction.sell(&date.to_local_str(&config), "0.11", "btc", "12000", "Description").unwrap();
    let gain = return_capital_gain(&db, &config);
    // The capital gain should not have changed
    assert_eq!(gain.to_string(), "7000.00");
    
    // Test earlier buy
    let earlier = date.add(Duration::hours(-1));
    transaction.buy(&earlier.to_local_str(&config), "1", "btc", "1000", "Description").unwrap();
    
    let gain = return_capital_gain(&db, &config);
    assert_eq!(gain.to_string(), "8900.00");
    // END - Test earlier buy
    
    // Test buy after sell
    let date = date.inc();
    let id = transaction.buy(&date.to_local_str(&config), "0.30", "btc", "15000", "Description").unwrap().unwrap();
    let tmp_table = &mut transaction.table("btc").unwrap();
    let next_id = get_next_line_id(tmp_table, &id);
    let new_cost_basis = cost.calculate(&transaction.table("btc").unwrap(), next_id, false).unwrap().unwrap(); // The transaction is not the last one
    assert_eq!(new_cost_basis.to_string(), "16823.5294");
    
    // Second sell test to check the capital gain with multiple tax events
    let date = date.inc();
    let sell_id = transaction.sell(&date.to_local_str(&config), "0.52", "btc", "1000", "Description").unwrap().unwrap();
    let new_cost_basis = cost.calculate(&transaction.table("btc").unwrap(), Some(&sell_id), false).unwrap().unwrap(); // The transaction is not the last one
    assert_eq!(new_cost_basis.to_string(), "16823.5294");
    
    // Check the capital gain with multiple tax events / sell
    let gain = return_capital_gain(&db, &config);
    assert_eq!(gain.to_string(), "1151.7647");
    
    // Check the value of cost basis (includes the later/last transaction)
    let later_cost = cost.calculate(&transaction.table("btc").unwrap(), None, false).unwrap().unwrap();
    assert_eq!(later_cost.to_string(), "11601.2425");
}

#[test]
fn test_exchange() {
    let path = "/tmp/transaction_test_exchange";

    let mut db = init_db(path, true);
    let config = Config::new().unwrap();

    let cost = CostBasis::new(&config);

    let transaction = Transaction::new(&mut db, &config);
    let date = init_buy(&transaction, &config, Local::now());

    let date = date.inc();
    transaction.exchange(&date.to_local_str(&config), "0.25", "btc", "1", "eth", "1000", Some("Description")).unwrap();

    // Validate the different cost basis after an exchange of crypto
    let cost_btc = cost.calculate(&transaction.table("btc").unwrap(), None, false).unwrap().unwrap();
    let cost_eth = cost.calculate(&transaction.table("eth").unwrap(), None, false).unwrap().unwrap();
    assert_eq!(cost_btc.to_string(), "20000");
    assert_eq!(cost_eth.to_string(), "2000");

    let gain = return_capital_gain(&db, &config);

    // Validate the capital gain
    assert_eq!(gain.to_string(), "-4000.00");
}

fn init_buy(transaction: &Transaction, config: &Config, date: DateTime<Local>) -> DateTime<Local> {
    let date = date.inc();
    transaction
    .buy(
        &date.to_local_str(config),
        "1",
        "btc",
        "10000",
        "Description",
    )
    .unwrap();
    
    let date = date.inc();
    transaction
    .buy(
        &date.to_local_str(config),
        "0.5",
        "btc",
        "20000",
        "Description",
    )
    .unwrap();
    
    let date = date.inc();
    transaction
    .buy(
        &date.to_local_str(config),
        "2",
        "eth",
        "5000",
        "Description",
    )
    .unwrap();
    
    date
}

fn return_capital_gain(db: &Db, config: &Config) -> Decimal {
    let capital_gain = CapitalGain::new(&db, &config);
    let gain = capital_gain.gain(Local::now().year(), false).unwrap();
    
    gain
}

fn init_db(path: &str, fresh: bool) -> Db {
    if std::path::Path::new(path).exists() && fresh {
        std::fs::remove_dir_all(path).unwrap();
    }
    
    Db::new(path).unwrap()
}

fn get_next_line_id<'a>(tbl: &'a mut Table, current: &Uuid) -> Option<&'a Uuid> {
    let mut id = None;
    let mut lines = tbl.get_lines();
    sort_by_date(&mut lines);

    let mut next = false;
    for line in lines {
        if next {
            id = Some(line.get_id());
            break;
        }

        if line.get_id() == current {
            next = true;
        }
    }

    id
}