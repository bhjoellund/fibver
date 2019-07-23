#[derive(Debug)]
pub enum AppErrors {
    WrongNumberOfArguments(usize, usize),
    InvalidCommand(String),
    InvalidVersionFormat(String),
    InvalidFibonacciNumber(i32),
    UnknownError
}

impl std::convert::From<std::option::NoneError> for AppErrors {
    fn from(_: std::option::NoneError) -> Self {
        AppErrors::UnknownError
    }
}

pub enum AppCommands {
    Major,
    Minor,
    Patch,
    Verify
}

pub struct AppArgs {
    pub command: AppCommands,
    pub version: Version,
}

pub struct Version {
    pub major: i32,
    pub minor: i32,
    pub patch: i32
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}
