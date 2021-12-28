use rental_rod::db::Db;
use uuid::Uuid;

use crate::{config::Config, transaction::Transaction, error::CryptoError};

pub struct Crypto<'a> {
    db: &'a mut Db,
    config: &'a Config
}

impl<'a> Crypto<'a> {
    /**
    This tool is for tracking your crypto transactions. 
    It tracks the cost basis for any crypto and the capital gain
    */
    pub fn new(db: &'a mut Db, config: &'a Config) -> Crypto<'a> {
        Crypto { db, config }
    }
    
    /**
    Lists all the crypto ticker in the database
    */
    pub fn list(&self) {
        // print(listTickers(self.db))
    }
    
    /**
    Mostly used for developpement. It allows to delete a document
    :param table: Table to use to delete the document
    :param docId: Id of the document to delete
    */
    pub fn delete(&self, table: &str, id: Uuid) {
        // tbl = self.db.table(table)
        // tbl.remove(doc_ids=[docId])
    }
    
    /**
    Adds info to the cost basis for the specified crypto ticker
    :param date: The date and time at which the transaction was made
    :param amount: How much of the crypto did you buy?
    :param ticker: Name of the crypto ticker
    :param price: Price you paid for the crypto (including fees)
    :param description: The description of the transaction
    */
    pub fn buy(&mut self, date: &str, amount: &str, ticker: &str, price: &str, description: &str) -> Result<(), CryptoError> {
        let ticker = ticker.to_uppercase();
        let transaction = Transaction::new(&mut self.db, self.config);

        transaction.buy(date, amount, &ticker, price, description)
    }
    
    
    /**
    Tracks details of selling a crypto
    :param date: The date and time at which the transaction was made
    :param amount: How much of the crypto did you sell?
    :param ticker: Name of the crypto ticker
    :param forPrice: The price you sold the crypto for
    :param description: The description of the transaction
    */
    pub fn sell(&self, date: &str, amount: &str, ticker: &str, for_price: &str, description: &str) {
        // ticker = ticker.upper()
        // transaction = Transaction(self.db)
        // transaction.sell(date, amount, ticker, forPrice, description)
    }
    
    /**
    Tracks the value of an exchange (crypto for crypto)
    :param date: The date and time at which the transaction was made
    :param fromAmount: Amount to remove from fromTicker
    :param fromTicker: crypto that is exchanged for toTicker
    :param toAmount: amount of toTicker you bought with fromTicker
    :param toTicker: the new crypto that was bought
    :param toPrice: the current price (in fiat currency) for the amount of crypto (toTicker) that was bought
    :param description: The description of the transaction
    */
    pub fn exchange(&self, date: &str, from_amount: &str, from_ticker: &str, to_amount: &str, to_ticker: &str, to_price: &str, description: &str) {
        // fromTicker = fromTicker.upper()
        // toTicker = toTicker.upper()
        
        // transaction = Transaction(self.db)
        // transaction.exchange(date, fromAmount, fromTicker, toAmount, toTicker, toPrice, description)
    }
    
    /**
    Import a file from any supported exchange (the app supports ShakePay, Crypto.com and Newton)
    :param path: The path to the file to import
    */
    pub fn generate(&self, path: &str) {
        // gen = Generate(self.db, path)
        // gen.gen()
    }
    
    // Reports
    
    /**
    Gets the amount of crypto for the ticker
    :param ticker: Crypto ticker
    */
    pub fn amount(&self, ticker: &str) {
        // ticker = ticker.upper()
        
        // report = Report(self.db)
        // report.amount(ticker)
    }
    
    /**
    Returns the cost basis of the specified ticker
    :param ticker: Crypto ticker
    :param details: Use this parameter if you want more details on the report
    */
    pub fn cost(&self, ticker: &str, details: bool) {
        // ticker = ticker.upper()
        
        // report = Report(self.db)
        // report.costBasis(ticker, details)
    }
    
    /**
    Returns info about your capital gain or loss
    :param year: Choose the related year for the capital gain (current year by detault)
    :param details: Use this parameter if you want more details on the report
    */
    pub fn cg(&self, year: Option<i64>, details: bool) {
        // report = Report(self.db)
        // report.capitalGain(year, details)
    }
    
    /**
    Exports the history of transaction in the current database
    :param all: Returns all the interactions in the history
    :param withIds: Adds the id of the related document in the report
    */
    pub fn export(&self, all: bool, with_ids: bool) {
        // report = Report(self.db)
        // report.history(self.history, all, withIds)
    }
}