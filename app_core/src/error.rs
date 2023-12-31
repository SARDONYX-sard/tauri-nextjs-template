#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("Unexpected value. Valid range is 0..=10. But got {0}.")]
    UnexpectedNumber(u64),

    /// Convert to json error.
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[error(transparent)]
    ParseIntError(#[from] core::num::ParseIntError),
    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

//? Implemented to facilitate testing with the `assert_eq!` macro.
impl PartialEq for CoreError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::JsonError(l0), Self::JsonError(r0)) => l0.to_string() == r0.to_string(),
            (Self::ParseIntError(l0), Self::ParseIntError(r0)) => l0 == r0,
            (Self::IOError(l0), Self::IOError(r0)) => l0.kind() == r0.kind(),
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

pub type Result<T, Error = CoreError> = core::result::Result<T, Error>;
