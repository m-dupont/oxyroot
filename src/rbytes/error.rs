use std::array::TryFromSliceError;

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

    MiscError(String),
    WrongClass {
        expected: String,
        found: String,
    },

    RbufferExtractAsArrayNotPossible(TryFromSliceError),
    RMetaError(crate::rmeta::Error),
    RTypes(crate::rtypes::error::Error),
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

impl From<crate::rmeta::Error> for Error {
    fn from(e: crate::rmeta::Error) -> Self {
        Error::RMetaError(e)
    }
}

impl From<crate::rtypes::Error> for Error {
    fn from(e: crate::rtypes::Error) -> Self {
        Error::RTypes(e)
    }
}
