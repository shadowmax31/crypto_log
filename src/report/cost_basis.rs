use rental_rod::db::{line::Line, table::Table};
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::{util::{config::Config, error::CryptoError, helper::{Transaction, sort_by_date}, color::{BOLD, RED, ENDC}}, transaction::transaction_type::TransactionType};

pub struct CostBasis<'a> {
    config: &'a Config
}


impl<'a> CostBasis<'a> {
    pub fn new(config: &'a Config) -> CostBasis<'a> {
        CostBasis { config }
    }
    
    pub fn calculate(&self, table: &Table, stop_at: Option<&Uuid>, print_details: bool) -> Result<Option<Decimal>, CryptoError> {
        let mut amount = Decimal::ZERO;
        let mut total_cost = Decimal::ZERO;

        let ticker = table.get_name().to_owned();
        let mut sorted_lines = table.get_lines();
        sort_by_date(&mut sorted_lines);

        for line in &sorted_lines {
            let t_type = line.get_type()?;
            if t_type == TransactionType::Buy.value() {
                amount += line.get_amount()?;
                total_cost += line.get_price()?;
            }
            else if t_type == TransactionType::Sell.value() {
                let mut per_unit_cost = Decimal::ZERO;
                if amount > Decimal::ZERO {
                    per_unit_cost = total_cost / amount;
                }
        
                let l_amount = line.get_amount()?;
                amount -= l_amount;
                total_cost -= per_unit_cost * l_amount;
            }
            else {
                return Err(CryptoError::Custom(format!("Transaction type [{}] not implemented", t_type)));
            }
        
            self.print_details(print_details, &ticker, line)?;
            if let Some(id) = stop_at {
                if id == line.get_id() {
                    break;
                }
            }
        }
        
        let mut cost_basis = Some(Decimal::ZERO);
        if amount > Decimal::ZERO {
            let calc = total_cost / amount;
            cost_basis = Some(calc.round_dp(4));
        }
        else if amount < Decimal::ZERO {
            cost_basis = None;
            println!("{}{}You have {} amount of {}. Please check your data.{}", BOLD, RED, amount, ticker, ENDC);
        }
        
        if print_details && sorted_lines.len() > 0 {
            println!();
        }
        
        Ok(cost_basis)
    }
    
    pub fn print_details(&self, print_details: bool, ticker: &str, line: &Line) -> Result<(), CryptoError> {
        let amount = line.get_amount()?;

        if print_details {
            let price = line.get_price()?;

            let s_date = line.get_date_str(&self.config)?;
            let current_cost = (price / amount).round_dp(4);
            let mut msg = format!("{}: {} / {} {} {} for {}$ ({}$)", line.to_short_id(), s_date, line.get_type()?, amount, ticker, price, current_cost);

            if let Some(desc) = line.get("description") {
                msg = msg + " (" + &desc.get().to_str()? + ")";
            }
        
            println!("{}", msg);
        }
        
        if amount == &Decimal::ZERO {
            println!("----- No more {} -----", ticker);
            println!();
        }

        Ok(())
    }
}