#![warn(clippy::pedantic)]

mod banking;
mod currency;
mod io_utils;

use banking::Account;
use io_utils::{print_ordered_list, prompt};
use std::collections::HashMap;

const TRANSACTION_TITLES: [&str; 6] = [
    "Register Account Name",
    "Deposit Amount",
    "Withdraw Amount",
    "Currency Exchange",
    "Record Exchange Rates",
    "Show Interest Amount",
];

fn main() {
    let mut accounts = Vec::new();
    let mut exchange_rates = HashMap::<&str, f64>::new();

    for code in currency::CURRENCIES_CODES.iter().skip(1) {
        exchange_rates.insert(code, 1.0);
    }

    'main_menu: loop {
        println!("Select Transaction:");
        print_ordered_list(&TRANSACTION_TITLES);

        println!();

        let chosen_idx = prompt("> ").parse::<usize>().unwrap_or_default();

        println!();

        if chosen_idx > 0 && chosen_idx <= TRANSACTION_TITLES.len() {
            println!("{}", TRANSACTION_TITLES[chosen_idx - 1]);
        }

        match chosen_idx {
            1 => {
                let account = Account::new(prompt("Account Name: "));

                if accounts.contains(&account) {
                    println!("An account with this name already exists!");
                } else {
                    accounts.push(account);
                }
            }
            2 | 3 => {
                if let Some(account) = accounts.iter_mut().find(|a| a.name == prompt("Account Name: ")) {
                    if chosen_idx == 2 {
                        account.deposit_balance(&exchange_rates);
                    } else {
                        account.withdraw_balance(&exchange_rates);
                    }
                } else {
                    println!("No account with this name exists!");
                }
            }
            4 => 'currency_exchange: loop {
                currency::exchange(&exchange_rates);

                println!();

                loop {
                    let is_repeating = prompt("Convert another currency? (Y/N): ").to_uppercase();

                    if is_repeating == "Y" {
                        println!();

                        break;
                    } else if is_repeating == "N" {
                        break 'currency_exchange;
                    }

                    println!("Only accepting a [Y]es or [N]o answer!");

                    println!();
                }
            },
            5 => {
                println!();

                currency::set_exchange_rates(&mut exchange_rates);
            }
            6 => {
                if let Some(account) = accounts.iter().find(|a| a.name == prompt("Account Name: ")) {
                    account.calculate_interest();
                } else {
                    println!("No account with this name exists!");
                }
            }
            _ => {
                println!("No transaction with this ID exists!");
            }
        }

        println!();

        loop {
            let is_continuing = prompt("Back to the Main Menu (Y/N): ").to_uppercase();

            if is_continuing == "Y" {
                println!();

                break;
            } else if is_continuing == "N" {
                break 'main_menu;
            }

            println!("Only accepting a [Y]es or [N]o answer!");

            println!();
        }
    }
}
