use core::fmt::{self, Debug, Display, Formatter};
use std::error;

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

impl Debug for HResultError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "HResultError({self})")
    }
}

impl Display for HResultError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "0x{:08X} {:?}",
            self.0 .0,
            winresult::HResultError::from_constant(self.0 .0)
        )
    }
}

impl error::Error for HResultError {}

#[derive(Clone, Copy, Default, Eq, PartialEq)]
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

#[cfg(test)]
mod tests {
    use crate::hresult::HResult;

    #[test]
    fn error() {
        macro_rules! test {
            ($error_code:literal, $message:literal) => {{
                let error = {
                    let result = HResult::from($error_code);
                    assert!(!result.is_success());
                    let success = result.success(0);
                    assert!(success.is_err());
                    success.unwrap_err()
                };
                let display = format!("{error}");
                assert!(display.ends_with($message));
                let debug = format!("{error:?}");
                assert!(debug.ends_with(core::concat!($message, ")")));
            }};
        }

        test!(0x80080200, "E_PACKAGING_INTERNAL");
        test!(0x800B0101, "E_EXPIRED");
        test!(0x80094001, "E_BAD_REQUESTSUBJECT");
        test!(0x80004006, "E_INIT_TLS");
        test!(0x80110401, "E_OBJECTERRORS");
    }
}
