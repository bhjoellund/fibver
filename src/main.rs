#![feature(try_trait)]
#![feature(int_error_internals)]

use crate::commands::run;
use crate::args::get_args;
use crate::models::AppErrors;

mod fibonacci;
mod args;
mod models;
mod commands;

fn print_help(err_msg: Option<String>) -> ! {
    err_msg.iter().for_each(|x| println!("ERROR: {}", x));
    println!("fibver v{}", env!("CARGO_PKG_VERSION"));
    println!("usage: fibver <command> <input>");
    println!();
    println!("command:");
    println!("  One of major, minor, patch or verify");
    println!("input");
    println!("  Either a version number in the form major.minor.patch or - to read from stdin");
    std::process::exit(1)
}

fn format_error(err: AppErrors) -> String {
    match err {
        AppErrors::InvalidCommand(command) => format!("{} is not a valid command", command),
        AppErrors::InvalidFibonacciNumber(number) => {
            format!("{} is not a fibonacci number", number)
        }
        AppErrors::InvalidVersionFormat(version) => {
            format!("{} is not a recognized version format", version)
        }
        AppErrors::WrongNumberOfArguments(expected, found) => {
            format!("Expected {} arguments, found {}", expected, found)
        }
        AppErrors::UnknownError => {
            "Unknown error occurred".to_string()
        }
    }
}

fn main() {
    let args = get_args();

    match args {
        Err(err) => print_help(Some(format_error(err))),
        Ok(args) => run(args).unwrap_or_else(|e| print_help(Some(format_error(e)))),
    }
}
