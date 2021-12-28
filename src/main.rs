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
        ).get_matches();

    let mut crypto = Crypto::new()?;
    if let Some(m) = m.subcommand_matches("buy") {  
        crypto.buy(m)?;
    }

    if let Some(_) = m.subcommand_matches("list") {
        crypto.list()?;
    }

    Ok(())
}
