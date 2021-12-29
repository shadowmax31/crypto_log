use clap::{App, SubCommand, Arg};
use crypto::Crypto;
use util::error::CryptoError;

#[allow(dead_code, unused_variables)]
mod crypto;
mod transaction;
mod util;

fn main() -> Result<(), CryptoError> {
    let m = App::new("crypto")
        .subcommand(
            SubCommand::with_name("list")
        )
        .subcommand(
            SubCommand::with_name("buy")
                .arg(Arg::with_name("date").required(true).index(1))
                .arg(Arg::with_name("amount").required(true).index(2))
                .arg(Arg::with_name("ticker").required(true).index(3))
                .arg(Arg::with_name("price").required(true).index(4))
                .arg(Arg::with_name("description").required(true).index(5))
        )
        .subcommand(
            SubCommand::with_name("sell")
                .arg(Arg::with_name("date").required(true).index(1))
                .arg(Arg::with_name("amount").required(true).index(2))
                .arg(Arg::with_name("ticker").required(true).index(3))
                .arg(Arg::with_name("for_price").required(true).index(4))
                .arg(Arg::with_name("description").required(true).index(5))
        )
        .subcommand(
            SubCommand::with_name("exchange")
                .arg(Arg::with_name("date").required(true).index(1))
                .arg(Arg::with_name("from_amount").required(true).index(2))
                .arg(Arg::with_name("from_ticker").required(true).index(3))
                .arg(Arg::with_name("to_amount").required(true).index(4))
                .arg(Arg::with_name("to_ticker").required(true).index(5))
                .arg(Arg::with_name("at_price").required(true).index(6))
                .arg(Arg::with_name("description").required(false).index(7))
        )
        .get_matches();

    let mut crypto = Crypto::new()?;
    if let Some(m) = m.subcommand_matches("buy") {  
        crypto.buy(m)?;
    }

    if let Some(m) = m.subcommand_matches("sell") {
        crypto.sell(m)?;
    }

    if let Some(m) = m.subcommand_matches("exchange") {
        crypto.exchange(m)?;
    }

    if let Some(_) = m.subcommand_matches("list") {
        crypto.list()?;
    }

    Ok(())
}
