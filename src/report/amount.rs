use rental_rod::db::table::Table;
use rust_decimal::Decimal;

use crate::{util::{error::CryptoError, helper::Transaction}, transaction::transaction_type::TransactionType};

pub fn get_amount(table: &mut Table) -> Result<Decimal, CryptoError> {
    let mut total = Decimal::ZERO;

    for line in table.get_lines() {
        let t_type = line.get_type()?;
        let amount = line.get_amount()?;

        if t_type == TransactionType::Buy.value() {
            total = total + amount;
        }
        else if t_type == TransactionType::Sell.value() {
            total = total - amount;
        }
        else {
            return Err(CryptoError::Custom(format!("Type [{}] is not supported", &t_type)));
        }
    }

    Ok(total)
}