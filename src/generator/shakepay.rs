use csv::StringRecord;

use crate::util::{error::CryptoError};

use super::{csv_helper::Throw, Generator};


pub fn generate(generator: &Generator, row: &StringRecord) -> Result<bool, CryptoError> {
    let mut found = true;
    
    let date = generator.get_naive_datetime(row.get_throw(1)?, "%Y-%m-%dT%H:%M:%S+00")?;
    let date = generator.convert_from_utc_to_local(date)?;
    let amount = row.get_throw(4)?;
    let ticker = row.get_throw(5)?;
    let mut price = row.get_throw(2)?;
    if price == "" {
        price = "0";
    }
    
    if row.get_throw(0)? == "purchase/sale" {
        if row.get_throw(3)? == "CAD" {
            generator.transaction_string("buy", &date, amount, ticker, price)?;
        }
        else if row.get_throw(5)? == "CAD" {
            let sold_ticker_amount = row.get_throw(2)?;
            let sold_ticker = row.get_throw(3)?;
            let sold_for = row.get_throw(4)?;

            generator.transaction_string("sell", &date, sold_ticker_amount, sold_ticker, sold_for)?;
        }
        else {
            generator.comment("This transaction type is not supported. Please add an issue on github.", Some(row)); 
        }
    }
    else if row.get_throw(0)? == "peer transfer" {
        generator.transaction_string("buy", &date, amount, ticker, price)?;
    }
    else if row.get_throw(0)? == "shakingsats" {
        generator.transaction_string("buy", &date, amount, ticker, "0")?;
    }
    else {
        found = false;
    }
    
    Ok(found)
}