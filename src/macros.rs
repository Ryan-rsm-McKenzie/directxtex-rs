#[derive(Debug)]
pub struct InvalidEnumRepresentation<T>(pub(crate) T);

impl<T> core::fmt::Display for InvalidEnumRepresentation<T>
where
    T: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} is not a valid variant of the enumeration", self.0)
    }
}

impl<T> std::error::Error for InvalidEnumRepresentation<T> where
    T: core::fmt::Debug + core::fmt::Display
{
}

macro_rules! c_enum {
	($enumeration:ident($underlying:ty) => { $($variant:ident = $value:literal,)* }) => {
		#[allow(non_camel_case_types)]
		#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
		#[repr(transparent)]
		pub struct $enumeration($underlying);

		impl $enumeration {
			$(pub const $variant: Self = Self($value);)*

			pub fn from_unchecked(value: $underlying) -> Self {
				Self(value)
			}
		}

		impl From<$enumeration> for $underlying {
			fn from(value: $enumeration) -> Self {
				value.0
			}
		}

		impl TryFrom<$underlying> for $enumeration {
			type Error = crate::InvalidEnumRepresentation<$underlying>;

			fn try_from(value: $underlying) -> Result<Self, Self::Error> {
				match value {
					$($value => Ok(Self::$variant),)*
					_ => Err(crate::InvalidEnumRepresentation(value))
				}
			}
		}

		impl core::fmt::Display for $enumeration {
			fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
				match self.0 {
					$($value => write!(f, "{}", core::stringify!($variant)),)*
					unk => write!(f, "{unk:x}"),
				}
			}
		}
	};
}

pub(crate) use c_enum;
