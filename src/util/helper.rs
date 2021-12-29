use rental_rod::db::{Db, line::Line};
use rust_decimal::Decimal;

use super::{error::CryptoError, config::Config};

const CAPITAL_GAIN: &str = "CAPITAL_GAIN";
const SYSTEM_TABLES: [&str; 1]= [CAPITAL_GAIN];

pub fn list_tickers(db: &Db) -> Result<Vec<String>, CryptoError>{
    let tables = db.tables()?;
    
    let mut tables_without_system = vec![];
    for table in tables {
        if !SYSTEM_TABLES.contains(&&table[..]) {
            tables_without_system.push(table);
        }
    }
    
    Ok(tables_without_system)
}

pub trait Transaction {
    fn get_type(&self) -> Result<String, CryptoError>;
    fn get_amount(&self) -> Result<&Decimal, CryptoError>;
    fn get_price(&self) -> Result<&Decimal, CryptoError>;
    fn get_date_str(&self, config: &Config) -> Result<String, CryptoError>;
    fn get_decimal(&self, name: &str) -> Result<&Decimal, CryptoError>;
}

impl Transaction for Line {
    fn get_type(&self) -> Result<String, CryptoError> {
        match self.get_read("type") {
            Some(field) => Ok(field.get().to_str()?),
            None => Err(CryptoError::Custom(format!("Missing field type for line [{}]", self.get_id().to_string())))
        }
    }
    
    fn get_amount(&self) -> Result<&Decimal, CryptoError> {
        self.get_decimal("amount")
    }
    
    fn get_price(&self) -> Result<&Decimal, CryptoError> {
        self.get_decimal("price")
    }

    fn get_date_str(&self, config: &Config) -> Result<String, CryptoError> {
        let format = config.date_format()?;
        match self.get_read("date") {
            Some(field) => Ok(field.get().to_datetime()?.format(&format).to_string()),
            None => return Err(CryptoError::Custom(format!("Missing field {} for line [{}]", "date", self.get_id().to_string())))
        }
    }

    fn get_decimal(&self, name: &str) -> Result<&Decimal, CryptoError> {
        match self.get_read(name) {
            Some(field) => Ok(field.get().to_decimal()?),
            None => return Err(CryptoError::Custom(format!("Missing field {} for line [{}]", name, self.get_id().to_string())))
        }
    }
    
}