use std::str::FromStr;

use csv::StringRecord;
use rust_decimal::Decimal;

use crate::util::{error::CryptoError, color::{RED, ENDC, YELLOW}};

use super::{csv_helper::Throw, Generator};

pub fn generate(generator: &Generator, row: &StringRecord) -> Result<bool, CryptoError> {
        let mut found = true;

        let date = generator.get_naive_datetime(row.get_throw(0)?, "%Y-%m-%d %H:%M:%S")?;
        let date = &generator.convert_from_utc_to_local(date)?;
        let amount = row.get_throw(3)?;
        let ticker = row.get_throw(2)?;
        let mut price = row.get_throw(7)?;
        if price == "" {
            price = "0";
        }

        let to_ticker = row.get_throw(4)?;
        let to_amount = row.get_throw(5)?;

        let table = generator.transaction.table(ticker)?;

        let transaction_type = row.get_throw(9)?;
        if transaction_type == "reimbursement" || transaction_type == "referral_card_cashback" || transaction_type == "crypto_earn_interest_paid" {
            generator.transaction_string("buy", date, amount, ticker, "0")?;
        }
        else if transaction_type == "viban_purchase" {
            generator.transaction_string("buy", date, to_amount, to_ticker, price)?;
            generator.comment("Check this transaction ^", None);
        }
        else if transaction_type == "card_top_up" || transaction_type == "crypto_transfer" || (transaction_type == "crypto_viban_exchange" && to_ticker == "CAD") {
            generator.transaction_string("sell", date, &Decimal::from_str(amount)?.abs().to_string(), ticker, &Decimal::from_str(price)?.abs().to_string())?;
        }
        else if transaction_type == "crypto_exchange" {
            generator.transaction_exchange_string(date, &Decimal::from_str(amount)?.abs().to_string(), ticker, to_amount, to_ticker, &Decimal::from_str(price)?.abs().to_string())?;
        }
        else if transaction_type == "dust_conversion_credited" || transaction_type == "dust_conversion_debited" {
            if !generator.transaction.transaction_exists(date, &table, true)? {
                generator.echo("", Some(row));
                generator.echo(&format!("{}^ Please insert this row manually, this is not supported yet{}", RED, ENDC), None);
                generator.echo(&format!("{}Use this date: {}{}", YELLOW, date, ENDC), None);
                generator.echo("", None);
            }
            else {
                generator.comment("The transaction exists", Some(row));
            }
        }
        else {
            found = false;
        }

    Ok(found)
}