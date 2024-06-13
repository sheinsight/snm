use std::path::PathBuf;

use crate::snm_error::SnmError;

pub trait ToOk {
    fn to_ok<T>(self) -> Result<T, SnmError>
    where
        T: From<PathBuf>;
}

impl ToOk for PathBuf {
    fn to_ok<T>(self) -> Result<T, SnmError>
    where
        T: From<PathBuf>,
    {
        Ok(T::from(self))
    }
}
