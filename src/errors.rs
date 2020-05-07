use std::{fmt, result};
use std::error;

pub type Result<T> = result::Result<T, HangeulError>;

#[derive(Debug, PartialEq)]
pub enum HangeulError {
    JamoNotFound,
    NotASyllable,
    Uncomposable,
}

impl fmt::Display for HangeulError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HangeulError::JamoNotFound => write!(f, "HangeulError: Jamo not found"),
            HangeulError::NotASyllable => write!(f, "HangeulError: Not a correct Hangeul syllable"),
            HangeulError::Uncomposable => write!(f, "HangeulError: Uncomposable"),
        }
    }
}

impl error::Error for HangeulError {
    fn description(&self) -> &str {
        match *self {
            HangeulError::JamoNotFound => "HangeulError: Jamo not found",
            HangeulError::NotASyllable => "HangeulError: Not a correct Hangeul syllable",
            HangeulError::Uncomposable => "HangeulError: Uncomposable",
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            _ => None
        }
    }
}
