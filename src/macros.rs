macro_rules! c_enum {
	($enumeration:ident($underlying:ty) => { $($variant:ident = $value:literal,)* }) => {
		#[allow(non_camel_case_types)]
		#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
		#[repr(transparent)]
		pub struct $enumeration($underlying);

		impl $enumeration {
			$(pub const $variant: Self = Self($value);)*
		}

		impl From<$enumeration> for $underlying {
			fn from(value: $enumeration) -> Self {
				value.0
			}
		}

		impl From<$underlying> for $enumeration {
			fn from(value: $underlying) -> Self {
				Self(value)
			}
		}

		impl core::fmt::Debug for $enumeration {
			fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
				match self.0 {
					$($value => write!(f, "{}", core::stringify!($variant)),)*
					unk => write!(f, "0x{unk:x}"),
				}
			}
		}
	};
}

pub(crate) use c_enum;
