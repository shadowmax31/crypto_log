use csv::StringRecord;

use crate::util::error::CryptoError;

use super::{csv_helper::Throw, Generator};

pub fn generate(generator: &Generator, row: &StringRecord) -> Result<bool, CryptoError> {
    let mut found = true;
    
    if row.get_throw(1)? == "TRADE" {
        let date = generator.get_naive_datetime(row.get_throw(0)?, "%m/%d/%Y %H:%M:%S")?;
        let date = generator.convert_date_to_str(date)?;

        let amount = row.get_throw(2)?;
        let ticker = row.get_throw(3)?;
        let mut price = row.get_throw(4)?;
        if price == "" {
            price = "0"
        }
        
        if row.get_throw(5)? == "CAD" {
            generator.transaction_string("buy", &date, amount, ticker, price)?;
        }
        else {
            generator.comment("This transaction type is not supported. Please add an issue on github.", Some(&row));
        }
    }
    else {
        found = false;
    }
    
    Ok(found)
}
