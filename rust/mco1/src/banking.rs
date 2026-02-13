use crate::{currency, io_utils::prompt};
use std::collections::HashMap;

#[derive(PartialEq)]
pub(crate) struct Account {
    pub(crate) name: String,
    pub(crate) balance: f64,
    pub(crate) currency: String,
}
impl Account {
    const ANNUAL_INTEREST_RATE: f64 = 0.05;

    pub(crate) fn new(name: String) -> Account {
        Account {
            name,
            balance: 0.0,
            currency: String::from("PHP"),
        }
    }

    pub(crate) fn deposit_balance(&mut self, currency_to_rate: &HashMap<&str, f64>) {
        println!("Current Balance: {:.2}", self.balance);

        let currency = prompt("Currency: ").to_uppercase();

        if !currency::CURRENCIES_CODES.iter().any(|c| *c == currency) {
            println!("No currency with this code exists!");

            return;
        }

        println!();

        if let Ok(amount) = prompt("Deposit Amount: ").parse::<f64>() {
            self.balance += if currency == "PHP" {
                amount
            } else {
                currency::convert(amount, &currency.as_str(), &"PHP", currency_to_rate)
            };

            println!("Updated Balance: {:.2}", self.balance);
        } else {
            println!("Deposit amount must be a floating point number!");
        }
    }

    pub(crate) fn withdraw_balance(&mut self, rates: &HashMap<&str, f64>) {
        println!("Current Balance: {:.2}", self.balance);

        let currency = prompt("Currency: ").to_uppercase();

        if !currency::CURRENCIES_CODES.iter().any(|c| *c == currency) {
            println!("No currency with this code exists!");

            return;
        }

        println!();

        if let Ok(amount) = prompt("Withdraw Amount: ").parse::<f64>() {
            let updated_balance = self.balance
                - (if currency == "PHP" {
                    amount
                } else {
                    currency::convert(amount, &currency.as_str(), &"PHP", rates)
                });

            if updated_balance < 0.0 {
                println!("Withdraw amount must be less than the current balance!");

                return;
            }

            self.balance = updated_balance;

            println!("Updated Balance: {:.2}", self.balance);
        } else {
            println!("Withdraw amount must be a floating point number!");
        }
    }

    pub(crate) fn calculate_interest(&self) {
        let &Account { mut balance, .. } = self;

        println!("Current Balance: {balance:.2}");
        println!("Currency: {}", self.currency);
        println!(
            "Interest Rate: {annual_interest:.2}%",
            annual_interest = Account::ANNUAL_INTEREST_RATE * 100.0
        );

        println!();

        if let Ok(day_cnt) = prompt("Total Number of Days: ").parse::<u32>() {
            println!();

            println!("Day | Interest | Balance |");

            let daily_interest = (balance * (Account::ANNUAL_INTEREST_RATE / 365.0) * 100.0).round() / 100.0;

            for i in 1..=day_cnt {
                balance += daily_interest;

                println!("{i:<3} | {daily_interest:<8} | {balance:<7.2} |");
            }
        } else {
            println!("Number must be a positive whole number (integer)!");
        }
    }
}
