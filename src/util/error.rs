use rental_rod::db::db_error::DbError;

#[derive(Debug)]
pub enum CryptoError {
    Db(DbError),
    Custom(String)
}

impl From<DbError> for CryptoError {
    fn from(error: DbError) -> Self {
        CryptoError::Db(error)
    }
}

impl From<String> for CryptoError {
    fn from(error: String) -> Self {
        CryptoError::Custom(error)
    }
}

impl From<rust_decimal::Error> for CryptoError {
    fn from(error: rust_decimal::Error) -> Self {
        CryptoError::Custom(error.to_string())
    }
}

impl From<csv::Error> for CryptoError {
    fn from(error: csv::Error) -> Self {
        CryptoError::Custom(error.to_string())
    }
}