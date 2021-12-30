use chrono::{Datelike, Local};
use rental_rod::db::Db;
use rust_decimal::Decimal;

use crate::{util::{color::{GREEN, ENDC, YELLOW, RED}, error::CryptoError, config::Config}, report::amount::get_amount};

use self::{cost_basis::CostBasis, capital_gain::CapitalGain};

mod amount;
mod cost_basis;
mod capital_gain;

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



    // pub fn format_gain_loss(&self, value: &Decimal) -> String {
    //     let sign;
    //     if value.is_sign_positive() {
    //         sign = format!("{}+", GREEN);
    //     }
    //     else {
    //         sign = RED.to_owned();
    //     }

    //     sign.to_owned() + &value.to_string() + "$" + ENDC
    // }


    pub fn capital_gain(&self, year: Option<i32>, details: bool) -> Result<(), CryptoError> {
        let year = match year {
            Some(y) => y,
            None => Local::now().year()
        };

        let cg = CapitalGain::new(self.db, self.config);
        let gain = cg.gain(year, details)?;
        if details && gain != Decimal::ZERO {
            println!();
        }
                        
        let gain = gain.round_dp(2);
        if gain > Decimal::ZERO {
            println!("You have {}{}${} of capital gain", YELLOW, gain, ENDC);
        }
        else if gain < Decimal::ZERO {
            println!("You have {}{}${} of capital losses", RED, gain.abs(), ENDC);
        }
        else {
            println!("{}You have no capital gains or losses{}", GREEN, ENDC);
        }

        Ok(())
    }




}