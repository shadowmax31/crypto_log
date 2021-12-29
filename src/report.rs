use rental_rod::db::Db;

use crate::{util::{color::{GREEN, ENDC, YELLOW}, error::CryptoError, config::Config}, report::amount::get_amount};

use self::cost_basis::CostBasis;

mod amount;
mod cost_basis;

pub struct Report<'a> {
    db: &'a Db,
    config: &'a Config
}

impl<'a> Report<'a> {
    pub fn new(db: &'a Db, config: &'a Config) -> Report<'a> {
        Report { db, config }
    }

    pub fn amount(&self, ticker: &str) -> Result<(), CryptoError> {
        let amount = get_amount(&mut self.db.table(ticker)?)?;

        println!("You have {}{}{} {}", GREEN, amount, ENDC, ticker);
        Ok(())
    }
    
    pub fn cost_basis(&self, ticker: &str, details: bool) -> Result<(), CryptoError> {
        let mut table = self.db.table(ticker)?;
        // let amount = get_amount(&mut table)?;

        let cost = CostBasis::new(self.config);
        let msg = match cost.calculate(&mut table, None, details)? {
            Some(cost_basis) => {
                let msg: String =  "Cost basis for ".to_owned() + ticker + ": " + YELLOW + &cost_basis.to_string() + "$" + ENDC + "\n";
                // msg += "        Current cost:   " + str(realCost) + "$\n"

                // if realValue is None or gainLoss is None:
                //     sRealValue = RED + "UNKNOWN" + ENDC
                // else:
                //     sRealValue = str(realValue) + "$ (" + self.formatGainLoss(gainLoss) + ")"

                // msg += "        Current value:  " + sRealValue

                msg
            },
            None => format!("No {} found", ticker).to_owned()
        };

        // let real_value = None;
        // let gain_loss = None;
        // currentPrice = self.api.getPrice(ticker)
        // if currentPrice is not None {
        //     realValue = round(currentPrice * amount, 2)
        //     gainLoss = realValue - Decimal(realCost)
        // }

        println!("{}", msg);

        Ok(())
    }

}