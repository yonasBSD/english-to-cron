#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInput,
    Capture { state: String, token: String },
    ParseToNumber { state: String, value: String },
    IncorrectValue { state: String, error: String },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidInput => write!(f, "Please enter human readable"),
            Self::Capture { state, token } => {
                write!(f, "Could not capture: {token} in state: {state} ")
            }
            Self::ParseToNumber { state, value } => {
                write!(f, "Could not parse: {value} to number. state: {state} ")
            }
            Self::IncorrectValue { state, error } => {
                write!(
                    f,
                    "value is invalid in state: {state}. description: {error} "
                )
            }
        }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
