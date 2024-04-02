use std::array::TryFromSliceError;
use std::num::TryFromIntError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    VersionTooHigh {
        class: String,
        version_read: i16,
        max_expected: i16,
    },
    VersionTooLow {
        class: String,
        version_read: i16,
        min_expected: i16,
    },

    Misc(String),
    WrongClass {
        expected: String,
        found: String,
    },

    RbufferExtractAsArrayNotPossible(TryFromSliceError),
    RMeta(crate::rmeta::CantMakeError),
    RTypes(crate::rtypes::error::Error),
    TryFromInt(TryFromIntError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "IO/Root Error: {:?}", self)
    }
}

impl std::error::Error for Error {}

impl From<TryFromSliceError> for Error {
    fn from(e: TryFromSliceError) -> Self {
        Error::RbufferExtractAsArrayNotPossible(e)
    }
}

impl From<crate::rmeta::CantMakeError> for Error {
    fn from(e: crate::rmeta::CantMakeError) -> Self {
        Error::RMeta(e)
    }
}

impl From<crate::rtypes::Error> for Error {
    fn from(e: crate::rtypes::Error) -> Self {
        Error::RTypes(e)
    }
}

impl From<TryFromIntError> for Error {
    fn from(value: TryFromIntError) -> Self {
        Error::TryFromInt(value)
    }
}
