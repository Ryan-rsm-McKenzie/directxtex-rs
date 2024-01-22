use core::fmt::{self, Display, Formatter};
use std::error;

#[derive(Debug)]
#[repr(transparent)]
pub struct HResultError(HResult);

impl HResultError {
    #[must_use]
    pub fn code(self) -> u16 {
        self.0.code()
    }

    #[must_use]
    pub fn facility(self) -> u16 {
        self.0.facility()
    }

    #[must_use]
    pub fn is_customer(self) -> bool {
        self.0.is_customer()
    }

    #[must_use]
    pub fn is_microsoft(self) -> bool {
        self.0.is_microsoft()
    }

    #[must_use]
    pub fn into_underlying(self) -> u32 {
        self.0.into()
    }
}

impl Display for HResultError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "HRESULT 0x{:08X}", self.0 .0)
    }
}

impl error::Error for HResultError {}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[must_use]
#[repr(transparent)]
pub(crate) struct HResult(u32);

impl HResult {
    #[must_use]
    pub(crate) fn code(self) -> u16 {
        (self.0 & 0x0000_FFFF) as _
    }

    #[must_use]
    pub(crate) fn facility(self) -> u16 {
        ((self.0 & 0x07FF_0000) >> 16) as _
    }

    #[must_use]
    pub(crate) fn is_customer(self) -> bool {
        !self.is_microsoft()
    }

    #[must_use]
    pub(crate) fn is_microsoft(self) -> bool {
        (self.0 & 0x2000_0000) == 0
    }

    #[must_use]
    pub(crate) fn is_success(self) -> bool {
        (self.0 & 0x8000_0000) == 0
    }

    pub(crate) fn success<T>(self, value: T) -> Result<T, HResultError> {
        if self.is_success() {
            Ok(value)
        } else {
            Err(HResultError(self))
        }
    }
}

impl From<u32> for HResult {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<HResult> for u32 {
    fn from(value: HResult) -> Self {
        value.0
    }
}
