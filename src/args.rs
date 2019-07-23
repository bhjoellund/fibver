use crate::models::{AppArgs, AppCommands, AppErrors, Version};
use lazy_static::lazy_static;
use regex::Regex;
use std::env;
use std::io::Read;

lazy_static! {
    static ref VERSION: Regex =
        Regex::new(r#"^(?P<major>\d+)\.(?P<minor>\d+)\.(?P<patch>\d+)$"#).unwrap();
}

fn parse_version(version: &str) -> Result<Version, AppErrors> {
    let captures = VERSION.captures(version)?;
    let major: i32 = (&captures["major"]).parse().unwrap();
    let minor: i32 = (&captures["minor"]).parse().unwrap();
    let patch: i32 = (&captures["patch"]).parse().unwrap();

    Ok(Version {
        major,
        minor,
        patch
    })
}

fn get_command(cmd: &str) -> Result<AppCommands, AppErrors> {
    match cmd {
        "major" => Ok(AppCommands::Major),
        "minor" => Ok(AppCommands::Minor),
        "patch" => Ok(AppCommands::Patch),
        "verify" => Ok(AppCommands::Verify),
        x => Err(AppErrors::InvalidCommand(x.to_string()))
    }
}

fn get_version(input: &str) -> Result<Version, AppErrors> {
    let version = if input == "-" {
        let mut buffer: String = String::new();
        std::io::stdin().read_to_string(&mut buffer).unwrap();
        buffer.trim().to_string()
    } else {
        input.to_string()
    };

    if !VERSION.is_match(&version) {
        return Err(AppErrors::InvalidVersionFormat(version.to_string()));
    }

    parse_version(&version)
}

pub fn get_args() -> Result<AppArgs, AppErrors> {
    let args = env::args().collect::<Vec<String>>();

    if args.len() != 3 {
        return Err(AppErrors::WrongNumberOfArguments(3, args.len()));
    }

    let command = get_command(&args[1])?;
    let version = get_version(&args[2])?;

    Ok(AppArgs { command, version })
}