use clap::{App, SubCommand, Arg};

#[allow(dead_code, unused_variables)]
mod crypto;

fn main() {
    let m = App::new("crypto")
        .subcommand(
            SubCommand::with_name("buy")
                .arg(
                    Arg::with_name("ticker")
                        .required(true)
                        .index(1))
        ).get_matches();

    if let Some(m) = m.subcommand_matches("buy") {
        let ticker = m.value_of("ticker").unwrap();
        println!("{}", ticker);
    }
}

