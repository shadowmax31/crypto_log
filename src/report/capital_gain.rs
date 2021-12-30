use chrono::Datelike;
use rental_rod::db::{Db, line::Line, table::Table};
use rust_decimal::Decimal;

use crate::{util::{config::Config, helper::{list_tickers, sort_by_date, Transaction}, error::CryptoError, color::{BOLD, YELLOW, ENDC, GREEN}}, transaction::transaction_type::TransactionType};

use super::cost_basis::CostBasis;

pub struct CapitalGain<'a> {
    db: &'a Db,
    config: &'a Config
}

impl<'a> CapitalGain<'a> {
    pub fn new(db: &'a Db, config: &'a Config) -> CapitalGain<'a> {
        CapitalGain { db, config }
    }


    pub fn gain(&self, year: i32, details: bool) -> Result<Decimal, CryptoError> {
        let mut gain = Decimal::ZERO;
        for ticker in list_tickers(self.db)? {
            // Loop through all tickers
            let table = self.db.table(&ticker)?;
            let mut lines = table.find(|l| {
                match l.get_date() {
                    Ok(d) => d.year() == year,
                    Err(_) => false,
                }
            });

            sort_by_date(&mut lines);

            for line in lines {
                // A sell transaction is the only taxable event
                if line.get_type()? == TransactionType::Sell.value() {
                    let (tmp, cost_basis) = self.calculate(&table, &line)?;
                    gain += tmp;

                    if details {
                        let s_date = line.get_date_str(self.config)?;
                        let mut msg = s_date + ": ";
                        msg.push_str(&format!("{}{}_{}{}", GREEN, ticker, line.to_short_id(), ENDC));
                        msg.push_str(&format!(" - Capital Gain {}$", tmp));
                        msg.push_str(&format!(" (Value sold / Cost basis: {}$ / {}$)", line.get_price()?, cost_basis));

                        println!("{}", msg);
                    }
                }
            }

        }

        Ok(gain)
    }
        
                    

    pub fn calculate(&self, table: &Table, taxable_event: &Line) -> Result<(Decimal, Decimal), CryptoError> {
        let cost = CostBasis::new(&self.config);
        let cost_basis = cost.calculate(table, Some(taxable_event.get_id()), false)?;

        let capital_gain_and_cost_basis = match cost_basis {
            Some(cost_basis) => {
                let cost_basis_for_transaction = cost_basis * taxable_event.get_amount()?;
                let capital_gain = taxable_event.get_price()? - cost_basis_for_transaction;

                (capital_gain.round_dp(4), cost_basis_for_transaction.round_dp(4))
            },
            None => return Err(CryptoError::Custom(format!("{}{}The capital gain cannot be calculated accurately{}", BOLD, YELLOW, ENDC)))
        };
            

        Ok(capital_gain_and_cost_basis)
    }
}