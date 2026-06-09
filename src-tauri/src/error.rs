use serde::Serialize;
use std::error::Error;

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct BDMError(pub String);

pub type BDMResult<T> = Result<T, BDMError>;

impl<T> From<T> for BDMError
where
    T: Error,
{
    fn from(err: T) -> Self {
        BDMError(err.to_string())
    }
}
