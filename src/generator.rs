use chrono::{Utc, TimeZone, NaiveDateTime, Local};
use csv::StringRecord;
use rental_rod::db::Db;

use crate::{util::{config::Config, error::CryptoError}, transaction::Transaction};

use self::csv_helper::Throw;

mod csv_helper;
mod shakepay;
mod crypto_com;
mod newton;

enum GeneratorType {
    ShakePay,
    Newton,
    CryptoCom
}

pub struct Generator<'a> {
    config: &'a Config,
    transaction: Transaction<'a>,
    path: &'a str
}

impl<'a> Generator<'a> {
    pub fn new(db: &'a mut Db, config: &'a Config, path: &'a str) -> Generator<'a> {
        let transaction = Transaction::new(db, config);
        Generator { config, transaction, path }
    }
    
    pub fn gen(&self) -> Result<(), CryptoError> {
        // del = ,
        let mut csv = csv::ReaderBuilder::new().flexible(true).from_path(self.path)?;
        
        let generator_type = self.get_generator_type(csv.headers()?)?;
        
        let mut not_used: Vec<StringRecord> = vec![];
        println!("#!/bin/bash");
        for row in csv.records() {
            let row = row?;
            let used = match generator_type {
                GeneratorType::ShakePay => shakepay::generate(&self, &row)?,
                GeneratorType::Newton => newton::generate(&self, &row)?,
                GeneratorType::CryptoCom => crypto_com::generate(&self, &row)?,
            };
            
            if !used {
                not_used.push(row);
            }
        }
        
        if not_used.len() > 0 {
            println!();
            self.comment("List of transactions that were not used", None);
        }
        
        for row in not_used {
            self.comment("", Some(&row));
        }
        
        Ok(())
    }
    
    
    // Picks the right class to import the CSV
    fn get_generator_type(&self, row: &StringRecord) -> Result<GeneratorType, String>  {
        let generator_type;
        if row.get_throw(0)? == "Transaction Type" || row.get_throw(1)? == "Date" || row.get_throw(2)? == "Amount Debited" || row.get_throw(3)? == "Debit Currency" {
            generator_type = GeneratorType::ShakePay;
        }
        else if row.get_throw(0)? == "Timestamp (UTC)" || row.get_throw(1)? == "Transaction Description" || row.get_throw(2)? == "Currency" || row.get_throw(3)? == "Amount" {
            generator_type = GeneratorType::CryptoCom;
        }
        else if row.get_throw(0)? == "Date" || row.get_throw(1)? == "Type" || row.get_throw(2)? == "Received Quantity" || row.get_throw(3)? == "Received Currency" {
            generator_type = GeneratorType::Newton;
        }
        else {
            return Err(String::from("File not supported"));
        }
        
        Ok(generator_type)
    }
    
    // Generate the transaction (buy or sell) command
    pub fn transaction_string(&self, transaction_type: &str, date: &str, amount: &str, ticker: &str, price: &str) -> Result<(), CryptoError> {
        let mut msg = "";
        let table = self.transaction.table(ticker)?;
        
        if self.transaction.transaction_exists(date, &table, true)? {
            msg = "# The transaction exists -- ";
        }
        
        println!("{}crypto {} \"{}\" {} {} {} \"{}\"", 
        msg, transaction_type, date, amount, ticker, price, self.path);
        
        Ok(())
    }
    
    // Generate the exchange transaction command
    pub fn transaction_exchange_string(&self, date: &str, amount: &str, ticker: &str, to_amount: &str, to_ticker: &str, for_price: &str) -> Result<(), CryptoError> {
        let mut msg = "";
        let mut table = self.transaction.table(ticker)?;
        if self.transaction.transaction_exists(date, &mut table, true)? {
            msg = "# The transaction exists -- "
        }
        
        println!("{}crypto exchange \"{}\" {} {} {} {} {} \"{}\"", 
        msg, date, amount, ticker, to_amount, to_ticker, for_price, self.path);
        
        Ok(())
    }
    
    
    pub fn echo(&self, msg: &str, row: Option<&StringRecord>) {
        let msg = self.add_row_to_msg(msg, row);
        
        println!("echo \"{}\"", msg);
    }
    
    pub fn comment(&self, msg: &str, row: Option<&StringRecord>) {
        let msg = self.add_row_to_msg(msg, row);
        
        println!("# {}", msg);
    }
    
    pub fn add_row_to_msg(&self, msg: &str, row: Option<&StringRecord>) -> String {
        let  mut concat = msg.to_owned();
        if let Some(row) = row {
            if concat != "" {
                concat.push_str(": ");
            }
            
            
            concat.push_str("[");
            concat.push_str(&Self::write_row(row));
            concat.push_str("]");
        }
        
        concat
    }
    
    fn write_row(row: &StringRecord) -> String {
        let mut s_row = String::from("");
        for cell in row {
            if s_row.len() > 0 {
                s_row.push_str(", ");
            }
            s_row.push_str("'");
            s_row.push_str(cell);
            s_row.push_str("'");
        }
        
        s_row
    }
    
    
    
    
    // pub fn convert_date(&self, sDate: &str, sFormat: &str, pTimezone=timezone.utc):
    pub fn get_naive_datetime(&self, s_date: &str, format: &str) -> Result<NaiveDateTime, String> {
        match NaiveDateTime::parse_from_str(s_date, format) {
            Ok(date) => Ok(date),
            Err(_) => return Err(format!("Error parsing the date {} with {}", s_date, format))
        }
    }
    
    pub fn convert_from_utc_to_local(&self, date: NaiveDateTime) -> Result<String, CryptoError> {
        let format = &self.config.date_format()?;
        let date_utc = Utc.from_utc_datetime(&date);
        
        let s_date = date_utc.with_timezone(&Local).format(format).to_string();
        Ok(s_date)
    }

    pub fn convert_date_to_str(&self, date: NaiveDateTime) -> Result<String, CryptoError> {
        Ok(date.format(&self.config.date_format()?).to_string())
    }
    
}