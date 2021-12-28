use clap::{App, SubCommand, Arg};
use config::Config;
use crypto::Crypto;
use error::CryptoError;
use rental_rod::db::Db;

#[allow(dead_code, unused_variables)]
mod crypto;
mod config;
mod transaction;
mod error;

fn main() -> Result<(), CryptoError> {
    let m = App::new("crypto")
        .subcommand(
            SubCommand::with_name("buy")
                .arg(Arg::with_name("date").required(true).index(1))
                .arg(Arg::with_name("amount").required(true).index(2))
                .arg(Arg::with_name("ticker").required(true).index(3))
                .arg(Arg::with_name("price").required(true).index(4))
                .arg(Arg::with_name("description").required(true).index(5))
        ).get_matches();

    let config = Config::new()?;
    let mut db = init_db(&config)?;
    let mut crypto = Crypto::new(&mut db, &config);

    if let Some(m) = m.subcommand_matches("buy") {
        let date = m.value_of("date").unwrap();
        let amount = m.value_of("amount").unwrap();
        let ticker = m.value_of("ticker").unwrap();
        let price = m.value_of("price").unwrap();

        let description = m.value_of("description").unwrap();
       
        crypto.buy(date, amount, ticker, price, description)?;
    }

    Ok(())
}

fn init_db(config: &Config) -> Result<Db, CryptoError> {
    let path = config.db_path()?;

    Ok(Db::new(&path)?)
}

