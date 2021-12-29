use rental_rod::db::Db;

use super::error::CryptoError;

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