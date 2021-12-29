use rental_rod::db::table::Table;
use rust_decimal::Decimal;

use crate::{transaction::transaction_type::TransactionType, util::error::CryptoError};

pub fn get_amount(table: &mut Table) -> Result<Decimal, CryptoError> {
    let mut total = Decimal::ZERO;

    for line in table.get_lines() {
        let t_type = match line.get("type") {
            Some(field) => field.get().to_str()?,
            None => return Err(CryptoError::Custom(format!("Missing field type for line [{}]", line.get_id().to_string())))
        };

        let amount = match line.get("amount") {
            Some(field) => field.get().to_decimal()?,
            None => return Err(CryptoError::Custom(format!("Missing field amount for line [{}]", line.get_id().to_string())))
        };

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