use clap::{ArgMatches, value_t};
use rental_rod::db::Db;
use uuid::Uuid;

use crate::generator::Generator;
use crate::report::Report;
use crate::util::config::Config;
use crate::transaction::Transaction;
use crate::util::error::CryptoError;

pub struct Crypto {
    db: Db,
    config: Config
}

impl Crypto {
    /**
    This tool is for tracking your crypto transactions. 
    It tracks the cost basis for any crypto and the capital gain
    */
    pub fn new() -> Result<Crypto, CryptoError> {
        let config = Config::new()?;
        
        let path = config.db_path()?;
        let db = Db::new(&path)?;
        
        Ok(Crypto { db, config })
    }

    pub fn git(&mut self, args: &ArgMatches) -> Result<(), CryptoError> {
        let on_off = args.value_of("state").unwrap();
        let mut msg = None;

        if args.is_present("message") {
            msg = Some(args.value_of("message").unwrap());
        }

        self.db.set_use_git(on_off == "on", msg)?;

        Ok(())
    }
    
    /**
    Lists all the crypto ticker in the database
    */
    pub fn list(&self) -> Result<(), CryptoError> {
        let tables = crate::util::helper::list_tickers(&self.db)?;
        
        for table in tables {
            println!("{}", table);
        }
        
        Ok(())
    }
    
    /**
    Mostly used for developpement. It allows to delete a document
    :param table: Table to use to delete the document
    :param docId: Id of the document to delete
    */
    pub fn delete(&mut self, args: &ArgMatches) -> Result<(), CryptoError> {
        let ticker = args.value_of("ticker").unwrap().to_uppercase();
        let id = Uuid::parse_str(args.value_of("id").unwrap())?;

        let mut table = self.db.table(&ticker)?;
        table.delete(&id);
        
        self.db.write(&mut table)?;
        Ok(())
    }
    
    /**
    Adds info to the cost basis for the specified crypto ticker
    :param date: The date and time at which the transaction was made
    :param amount: How much of the crypto did you buy?
    :param ticker: Name of the crypto ticker
    :param price: Price you paid for the crypto (including fees)
    :param description: The description of the transaction
    */
    pub fn buy(&mut self, args: &ArgMatches) -> Result<(), CryptoError> {
        let date = args.value_of("date").unwrap();
        let amount = args.value_of("amount").unwrap();
        let ticker = args.value_of("ticker").unwrap().to_uppercase();
        let price = args.value_of("price").unwrap();
        let description = args.value_of("description").unwrap();
        
        let transaction = Transaction::new(&mut self.db, &self.config);
        transaction.buy(date, amount, &ticker, price, description)
    }
    
    
    /**
    Tracks details of selling a crypto
    :param date: The date and time at which the transaction was made
    :param amount: How much of the crypto did you sell?
    :param ticker: Name of the crypto ticker
    :param forPrice: The price you sold the crypto for
    :param description: The description of the transaction
    */
    pub fn sell(&mut self, args: &ArgMatches) -> Result<(), CryptoError> {
        let date = args.value_of("date").unwrap();
        let amount = args.value_of("amount").unwrap();
        let ticker = args.value_of("ticker").unwrap().to_uppercase();
        let for_price = args.value_of("for_price").unwrap();
        let description = args.value_of("description").unwrap();
        
        let transaction = Transaction::new(&mut self.db, &self.config);
        transaction.sell(date, amount, &ticker, for_price, description)
    }
    
    /**
    Tracks the value of an exchange (crypto for crypto)
    :param date: The date and time at which the transaction was made
    :param fromAmount: Amount to remove from fromTicker
    :param fromTicker: crypto that is exchanged for toTicker
    :param toAmount: amount of toTicker you bought with fromTicker
    :param toTicker: the new crypto that was bought
    :param atPrice: the current price (in fiat currency) for the amount of crypto (toTicker) that was bought
    :param description: The description of the transaction
    */
    pub fn exchange(&mut self, args: &ArgMatches) -> Result<(), CryptoError> {
        let date = args.value_of("date").unwrap();
        let from_amount = args.value_of("from_amount").unwrap();
        let from_ticker = args.value_of("from_ticker").unwrap().to_uppercase();
        let to_amount = args.value_of("to_amount").unwrap();
        let to_ticker = args.value_of("to_ticker").unwrap().to_uppercase();
        let at_price = args.value_of("at_price").unwrap();
        let description = args.value_of("description");
        
        let transaction = Transaction::new(&mut self.db, &self.config);
        transaction.exchange(date, from_amount, &from_ticker, to_amount, &to_ticker, at_price, description)
    }
    
    /**
    Import a file from any supported exchange (the app supports ShakePay, Crypto.com and Newton)
    :param path: The path to the file to import
    */
    pub fn generate(&mut self, args: &ArgMatches) -> Result<(), CryptoError> {
        let path = args.value_of("path").unwrap();

        let generator = Generator::new(&mut self.db, &self.config, path);
        generator.gen()
    }
    
    // Reports
    
    /**
    Gets the amount of crypto for the ticker
    :param ticker: Crypto ticker
    */
    pub fn amount(&self, args: &ArgMatches) -> Result<(), CryptoError> {
        let ticker = args.value_of("ticker").unwrap().to_uppercase();
        
        let report = Report::new(&self.db, &self.config);
        report.amount(&ticker)
    }
    
    /**
    Returns the cost basis of the specified ticker
    :param ticker: Crypto ticker
    :param details: Use this parameter if you want more details on the report
    */
    pub fn cost(&self, args: &ArgMatches) -> Result<(), CryptoError> {
        let ticker = args.value_of("ticker").unwrap().to_uppercase();
        let details = args.is_present("details");
        
        let report = Report::new(&self.db, &self.config);
        report.cost_basis(&ticker, details)
    }
    
    /**
    Returns info about your capital gain or loss
    :param year: Choose the related year for the capital gain (current year by detault)
    :param details: Use this parameter if you want more details on the report
    */
    pub fn cg(&self, args: &ArgMatches) -> Result<(), CryptoError> {
        let mut year = None;
        if args.is_present("year") {
            year = Some(value_t!(args.value_of("year"), i32).unwrap_or_else(|error| error.exit()));
        }
        
        let details = args.is_present("details");
        
        let report = Report::new(&self.db, &self.config);
        report.capital_gain(year, details)
    }    
}