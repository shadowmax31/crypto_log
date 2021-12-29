use rental_rod::db::Db;

use crate::{util::{color::{GREEN, ENDC}, error::CryptoError}, report::amount::get_amount};

mod amount;

pub struct Report<'a> {
    db: &'a Db
}

impl<'a> Report<'a> {
    pub fn new(db: &Db) -> Report {
        Report { db }
    }

    pub fn amount(&self, ticker: &str) -> Result<(), CryptoError> {
        let amount = get_amount(&mut self.db.table(ticker)?)?;

        println!("You have {}{}{} {}", GREEN, amount, ENDC, ticker);
        Ok(())
    }
}