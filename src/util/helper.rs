use std::cmp::Ordering;

use chrono::{DateTime, Utc, Local, NaiveDateTime, TimeZone};
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
    fn get_date(&self) -> Result<&DateTime<Utc>, CryptoError>;
    fn get_decimal(&self, name: &str) -> Result<&Decimal, CryptoError>;
    fn to_short_id(&self) -> String;
}

impl Transaction for Line {
    fn get_type(&self) -> Result<String, CryptoError> {
        match self.get("type") {
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
        match self.get("date") {
            Some(field) => Ok(field.get().to_datetime()?.with_timezone(&Local).format(&format).to_string()),
            None => return Err(CryptoError::Custom(format!("Missing field {} for line [{}]", "date", self.get_id().to_string())))
        }
    }
    
    fn get_decimal(&self, name: &str) -> Result<&Decimal, CryptoError> {
        match self.get(name) {
            Some(field) => Ok(field.get().to_decimal()?),
            None => return Err(CryptoError::Custom(format!("Missing field {} for line [{}]", name, self.get_id().to_string())))
        }
    }
    
    fn get_date(&self) -> Result<&DateTime<Utc>, CryptoError> {
        match self.get("date") {
            Some(field) => Ok(field.get().to_datetime()?),
            None => return Err(CryptoError::Custom(format!("Missing field {} for line [{}]", "date", self.get_id().to_string())))
        }
    }
    
    fn to_short_id(&self) -> String {
        self.get_id().to_string()[..8].to_owned()
    }
    
}

pub fn sort_by_date(lines: &mut Vec<&Line>) {
    lines.sort_by(|l1, l2| {
        let mut order = Ordering::Less;
        let date1 = l1.get("date").unwrap();
        let date2 = l2.get("date").unwrap();
        
        if date1.get().to_datetime().unwrap() > date2.get().to_datetime().unwrap() {
            order = Ordering::Greater;
        }
        
        order
    });
}

pub fn convert_to_date(date: &str, config: &Config) -> Result<DateTime<Utc>, CryptoError> {
    convert_to_date_format(date, &config.date_format()?)
}

pub fn convert_to_date_format(date: &str, format: &str) -> Result<DateTime<Utc>, CryptoError> {
    match NaiveDateTime::parse_from_str(date, &format) {
        Ok(dt) => {
            let local_date = match Local.from_local_datetime(&dt) {
                chrono::LocalResult::None => return Err(CryptoError::Custom("Invalid date [".to_owned() + date + "]")),
                chrono::LocalResult::Single(local_date) => local_date,
                chrono::LocalResult::Ambiguous(d1, d2) => {
                    let msg ="Could not decide between [".to_owned() + &d1.to_string() + "]" + " and [" + &d2.to_string() + "]";
                    return Err(CryptoError::Custom(msg));
                }
            };
            
            Ok(local_date.with_timezone(&Utc))
        },
        Err(_) => Err(CryptoError::Custom("Error parsing the date [".to_owned() + date + "]"))
    }
}