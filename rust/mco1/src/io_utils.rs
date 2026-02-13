use std::{
    fmt,
    io::{self, Write},
};

pub(crate) fn print_ordered_list<T: fmt::Display>(list: &[T]) {
    for (i, elm) in list.iter().enumerate() {
        println!("[{}] {elm}", i + 1);
    }
}

pub(crate) fn prompt(msg: &str) -> String {
    print!("{msg}");

    io::stdout().flush().expect("Failed to flush the output string...");

    let mut user_in = String::new();

    io::stdin()
        .read_line(&mut user_in)
        .expect("Failed to read the input string...");

    user_in.trim().to_string()
}
