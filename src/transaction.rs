use std::ascii::AsciiExt;
use std::str::FromStr;

use chrono::{DateTime, Utc, NaiveDateTime, TimeZone, Local};
use rental_rod::db::table::Table;
use rental_rod::db::{Db, line::Line, field_type::Type};
use rust_decimal::Decimal;

use crate::util::config::Config;
use crate::util::error::CryptoError;

use self::transaction_type::TransactionType;

mod transaction_type;

pub struct Transaction<'a> {
    db: &'a mut Db,
    config: &'a Config
}

impl<'a> Transaction<'a> {
    
    pub fn new(db: &'a mut Db, config: &'a Config) -> Transaction<'a> {
        Transaction { db, config }
    }
    
    pub fn buy(&self, date: &str, amount: &str, ticker: &str, price: &str, description: &str) -> Result<(), CryptoError> {
        let mut table = self.db.table(ticker)?;
        if !self.transaction_exists(date, &mut table, false)? {
            let line = self.create_transaction(date, amount, price, description, TransactionType::Buy)?;
            table.insert(line);
            
            self.db.write(&mut table)?;
        }
        
        Ok(())
    }
    
    pub fn sell(&self, date: &str, amount: &str, ticker: &str, price: &str, description: &str) -> Result<(), CryptoError> {
        let mut table = self.db.table(ticker)?;
        if !self.transaction_exists(date, &mut table, false)? {
            let line = self.create_transaction(date, amount, price, description, TransactionType::Sell)?;
            table.insert(line);
            
            self.db.write(&mut table)?;
        }
        
        Ok(())
    }
    
    pub fn exchange(&self, date: &str, from_amount: &str, from_ticker: &str, to_amount: &str, to_ticker: &str, at_price: &str, description: Option<&str>) -> Result<(), CryptoError> {
        let description = match description {
            Some(d) => d.to_owned() + " / ",
            None => "".to_owned()
        };
    
        self.sell(date, from_amount, from_ticker, at_price, &(description.to_owned() + "Exchanged for " + to_ticker))?;
        self.buy(date, to_amount, to_ticker, at_price, &(description.to_owned() + "Bought with " + from_ticker))?;

        Ok(())
    }
    
    pub fn transaction_exists(&self, date: &str, table: &mut Table, silent: bool) -> Result<bool, CryptoError> {
        let date = self.convert_to_date(date)?;
        let lines = table.get_lines();
        
        let mut exists = false;
        for line in lines {
            if let Some(field) = line.get("date") {
                match field.get() {
                    Type::DateTime(field) => {
                        exists = &date == field;

                        if exists {
                            break;
                        }
                    }
                    _ => ()
                }
            }
        }
        
        
        if exists && !silent {
            println!("The transaction already exists ({}: {}", table.get_name(), date);
        }
        
        Ok(exists)
    }
    
    pub fn create_transaction(&self, date: &str, amount: &str, price: &str, description: &str, t_type: TransactionType) -> Result<Line, CryptoError> {
        let mut line = Line::new();
        
        line.add("date", Type::from_datetime(self.convert_to_date(date)?))?;
        line.add("amount", Type::from_decimal(Decimal::from_str(amount)?))?;
        line.add("price", Type::from_decimal(Decimal::from_str(price)?))?;
        line.add("description", Type::from_str(description))?;
        line.add("type", Type::from_str(&t_type.value()))?;
        
        Ok(line)
    }
    
    pub fn convert_to_date(&self, date: &str) -> Result<DateTime<Utc>, CryptoError> {
        let format = self.config.date_format()?;
        match NaiveDateTime::parse_from_str(date, &format) {
            Ok(dt) => {
                let local = Local::now().timezone();
                let local_date = match local.from_local_datetime(&dt) {
                    chrono::LocalResult::None => return Err(CryptoError::Custom("Invalid date [".to_owned() + date + "]")),
                    chrono::LocalResult::Single(local_date) => local_date,
                    chrono::LocalResult::Ambiguous(d1, d2) => {
                        let msg ="Could not decide between [".to_owned() + &d1.to_string() + "]" + " and [" + &d2.to_string() + "]";
                        return Err(CryptoError::Custom(msg));
                    }
                };
                
                Ok(local_date.with_timezone(&Utc))
            },
            Err(a) => {
                let i = 0;
                Err(CryptoError::Custom("Error parsing the date [".to_owned() + date + "]"))
            }
        }
    }
    
}