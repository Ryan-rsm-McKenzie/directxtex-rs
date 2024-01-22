macro_rules! c_enum {
	(
		$(#[doc=$enumeration_doc:literal])*
		$enumeration:ident($underlying:ty) => {$(
			$(#[doc=$variant_doc:literal])*
			$variant:ident = $value:literal,
		)*}
	) => {
		#[allow(non_camel_case_types)]
		#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
		#[repr(transparent)]
		$(#[doc = $enumeration_doc])*
		pub struct $enumeration($underlying);

		impl $enumeration {
			$(
				$(#[doc = $variant_doc])*
				pub const $variant: Self = Self($value);
			)*

			#[must_use]
			pub const fn bits(self) -> $underlying {
				self.0
			}

			#[allow(unused)]
			#[must_use]
			pub(crate) const fn bits_fixed(self) -> u32 {
				self.bits() as u32
			}
		}

		impl Default for $enumeration {
			fn default() -> Self {
				Self(0)
			}
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

macro_rules! c_bits {
	(
		$(#[doc=$enumeration_doc:literal])*
		$enumeration:ident($underlying:ty) => {$(
			$(#[doc=$variant_doc:literal])*
			$variant:ident = $value:literal,
		)*}
	) => {
		bitflags::bitflags! {
			#[allow(non_camel_case_types)]
			#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
			#[repr(transparent)]
			$(#[doc = $enumeration_doc])*
			pub struct $enumeration: $underlying {
				$(
					$(#[doc = $variant_doc])*
					const $variant = $value;
				)*
				const _ = !0;
			}
		}

		impl $enumeration {
			#[allow(unused)]
			#[must_use]
			pub(crate) fn bits_fixed(self) -> u32 {
				self.bits() as u32
			}
		}
	};
}

pub(crate) use c_bits;
