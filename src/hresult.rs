use core::fmt::{self, Display, Formatter};
use std::error;

#[derive(Debug)]
#[repr(transparent)]
pub struct Error(i32);

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "HRESULT 0x{:X}", self.0)
    }
}

impl error::Error for Error {}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[must_use]
#[repr(transparent)]
pub struct HRESULT(i32);

impl HRESULT {
    #[must_use]
    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }

    #[must_use]
    pub fn is_ok(&self) -> bool {
        self.0 >= 0
    }

    pub fn ok(self) -> Result<(), Error> {
        if self.is_ok() {
            Ok(())
        } else {
            Err(Error(self.0))
        }
    }

    /// # Panics
    ///
    /// Panics if [`is_err`](Self::is_err)
    pub fn unwrap(&self) {
        assert!(self.is_ok(), "HRESULT 0x{:X}", self.0);
    }
}

impl From<i32> for HRESULT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<HRESULT> for i32 {
    fn from(value: HRESULT) -> Self {
        value.0
    }
}
