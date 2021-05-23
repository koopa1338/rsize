use std::{error::Error, fmt, num::ParseIntError};

#[derive(Debug)]
pub enum ConfigErr {
    EmptyVal,
    Parse(ParseIntError),
}

impl fmt::Display for ConfigErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigErr::EmptyVal => write!(f, "Empty or invalid Argument"),
            ConfigErr::Parse(..) => write!(f, "the provided string could not be parsed as int"),
        }
    }
}

impl Error for ConfigErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            ConfigErr::EmptyVal => None,
            ConfigErr::Parse(ref err) => Some(err),
        }
    }
}

impl From<ParseIntError> for ConfigErr {
    fn from(error: ParseIntError) -> Self {
        ConfigErr::Parse(error)
    }
}
