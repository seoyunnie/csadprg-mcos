use crate::io_utils::{print_ordered_list, prompt};
use std::{collections::HashMap, num::NonZeroUsize};

pub(crate) const CURRENCY_COUNT: usize = 6;
pub(crate) const CURRENCIES_TITLES: [&str; CURRENCY_COUNT] = [
    "Philippine Peso (PHP)",
    "United States Dollar (USD)",
    "Japanese Yen (JPY)",
    "British Pound Sterling (GBP)",
    "Euro (EUR)",
    "Chinese Yuan Renminni (CNY)",
];
pub(crate) const CURRENCIES_CODES: [&str; CURRENCY_COUNT] = ["PHP", "USD", "JPY", "GBP", "EUR", "CNY"];

pub(crate) fn convert(amount: f64, src: &&str, dest: &&str, rates: &HashMap<&str, f64>) -> f64 {
    let src_php_amount = if *src == "PHP" { amount } else { amount / rates[src] };

    if *dest == "PHP" {
        src_php_amount
    } else {
        src_php_amount * rates[dest]
    }
}

pub(crate) fn exchange(currency_to_rate: &HashMap<&str, f64>) {
    println!("Source Currency Options:");
    print_ordered_list(&CURRENCIES_TITLES);

    println!();

    let Ok(src_idx) = prompt("Source Currency: ").parse::<NonZeroUsize>().map(|i| i.get() - 1) else {
        println!("ID must be a positive whole number (integer)!");

        return;
    };

    if src_idx >= CURRENCY_COUNT {
        println!("No currency with this ID exists!");

        return;
    }

    let Ok(src_amount) = prompt("Source Amount: ").parse::<f64>() else {
        println!("Amount must be a floating point number!");

        return;
    };

    println!();

    println!("Exchanged Currency Options:");
    print_ordered_list(&CURRENCIES_TITLES);

    println!();

    let Ok(exchange_idx) = prompt("Exchange Currency: ")
        .parse::<NonZeroUsize>()
        .map(|i| i.get() - 1)
    else {
        println!("ID must be a positive whole number (integer)!");

        return;
    };

    if exchange_idx >= CURRENCY_COUNT {
        println!("No currency with this ID exists!");

        return;
    }

    println!(
        "Exchange Amount: {amount:.2}",
        amount = convert(
            src_amount,
            &CURRENCIES_CODES[src_idx],
            &CURRENCIES_CODES[exchange_idx],
            currency_to_rate
        )
    );
}

pub(crate) fn set_exchange_rates(currency_to_rate: &mut HashMap<&str, f64>) {
    print_ordered_list(&CURRENCIES_TITLES[1..]);

    println!();

    let Ok(idx) = prompt("Select Foreign Currency: ").parse::<usize>() else {
        println!("ID must be a positive whole number (integer)!");

        return;
    };

    if idx >= CURRENCY_COUNT {
        println!("No currency with this ID exists!");

        return;
    }

    let Ok(rate) = prompt("Exchange Rate: ").parse::<f64>() else {
        println!("Amount must be a floating point number!");

        return;
    };

    currency_to_rate.insert(CURRENCIES_CODES[idx], rate);
}
