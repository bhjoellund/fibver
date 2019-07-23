use crate::fibonacci::next_fibo_number;
use crate::models::{AppArgs, AppCommands, AppErrors, Version};

fn validate_version(version: &Version) -> Result<(), AppErrors> {
    next_fibo_number(version.major, 1, 1)
        .and_then(|_| next_fibo_number(version.minor, 1, 1))
        .and_then(|_| next_fibo_number(version.patch, 1, 1))
        .map(|_| ())
}

fn print_version(major: i32, minor: i32, patch: i32) {
    println!("{}.{}.{}", major, minor, patch)
}

fn increment(part: &AppCommands, version: &Version) -> std::result::Result<(), AppErrors> {
    match part {
        AppCommands::Major => next_fibo_number(version.major, 1, 1).and_then(|major| { print_version(major, 1, 1); Ok(()) }),
        AppCommands::Minor => next_fibo_number(version.minor, 1, 1).and_then(|minor| { print_version(version.major, minor, 1); Ok(()) } ),
        AppCommands::Patch => next_fibo_number(version.patch, 1, 1).and_then(|patch| { print_version(version.major, version.minor, patch); Ok(()) }),
        _ => Ok(())
    }
}

pub fn run(args: AppArgs) -> std::result::Result<(), AppErrors> {
    match args.command {
        AppCommands::Verify => match validate_version(&args.version) {
            Ok(_) => {
                println!("{} is valid", args.version);
                Ok(())
            },
            x => x,
        },
        AppCommands::Major | AppCommands::Minor | AppCommands::Patch => increment(&args.command, &args.version)
    }
}
