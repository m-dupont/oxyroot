pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    TryFromSliceError,
    Io(std::io::Error),
    FileHasAnIncorrectHeaderLength,
    CantReadDirectoryInfo {
        n_bytes_name_read: i32,
        n_bytes_name_min_allowed: i32,
        n_bytes_name_max_allowed: i32,
    },
    CantDecodeNameCycle(String),
    Rbytes(crate::rbytes::Error),
    KeyNotInFile {
        key: String,
        file: String,
    },
    CantLoadKeyPayload(String),
    ObjectNotInDirectory(String),

    DirectoryNegativeSeekKeys(i64),
    CantReadAmountOfBytesFromFile {
        requested: usize,
        read: usize,
    },
    InvalidPointerToStreamerInfo {
        seek: i64,
        min_allowed: i64,
        max_allowed: i64,
    },
    RCompress(crate::rcompress::Error),
    RTypes(crate::rtypes::error::Error),
    FileIsOpenedWriteOnly,
    FileIsOpenedReadOnly,
    FileIsNotOpened,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "IO/Root Error: {:?}", self)
    }
}

impl std::error::Error for Error {}

impl From<crate::rbytes::Error> for Error {
    fn from(e: crate::rbytes::Error) -> Self {
        Error::Rbytes(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<crate::rcompress::Error> for Error {
    fn from(e: crate::rcompress::Error) -> Self {
        Error::RCompress(e)
    }
}

impl From<crate::rtypes::Error> for Error {
    fn from(e: crate::rtypes::Error) -> Self {
        Error::RTypes(e)
    }
}

impl From<std::array::TryFromSliceError> for Error {
    fn from(_: std::array::TryFromSliceError) -> Self {
        Error::TryFromSliceError
    }
}
