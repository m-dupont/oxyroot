pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    TypeMismatch { expected: String, given: String },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Root Error: {:?}", self)
    }
}

impl std::error::Error for Error {}
