macro_rules! c_enum {
    (
        $(#[doc=$enumeration_doc:literal])*
        $module:ident::$enumeration:ident($underlying:ty) => {$(
            $(#[doc=$variant_doc:literal])*
            $variant:ident = $value:literal,
        )*}

        $(extra => {$(
            $(#[doc=$extra_variant_doc:literal])*
            $extra_variant:ident = $extra_value:literal,
        )*})?
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

            $($(
                $(#[doc = $extra_variant_doc])*
                pub const $extra_variant: Self = Self($extra_value);
            )*)?

            #[must_use]
            pub const fn bits(self) -> $underlying {
                self.0
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
                    $($(
                        #[allow(unreachable_patterns)]
                        $extra_value => write!(f, "{}", core::stringify!($extra_variant)),
                    )*)?
                    unk => write!(f, "0x{unk:x}"),
                }
            }
        }

        pub(crate) mod $module {
            use super::$enumeration;

            $(
                $(#[doc = $variant_doc])*
                pub const $variant: $enumeration = $enumeration($value);
            )*

            $($(
                $(#[doc = $extra_variant_doc])*
                pub const $extra_variant: $enumeration = $enumeration($extra_value);
            )*)?
        }
    };
}

pub(crate) use c_enum;

macro_rules! c_bits {
    (
        $(#[doc=$enumeration_doc:literal])*
        $module:ident::$enumeration:ident($underlying:ty) => {$(
            $(#[doc=$variant_doc:literal])*
            $variant:ident = $value:literal,
        )*}

        $(extra => {$(
            $(#[doc=$extra_variant_doc:literal])*
            $extra_variant:ident = $extra_value:expr;
        )*})?
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
            $($(
                $(#[doc = $extra_variant_doc])*
                pub const $extra_variant: Self = $extra_value;
            )*)?

            #[allow(unused)]
            #[must_use]
            pub(crate) fn bits_fixed(self) -> u32 {
                self.bits() as u32
            }
        }

        pub(crate) mod $module {
            use super::$enumeration;

            $(
                $(#[doc = $variant_doc])*
                pub const $variant: $enumeration = unsafe { $enumeration::from_bits($value).unwrap_unchecked() };
            )*

            $($(
                $(#[doc = $extra_variant_doc])*
                pub const $extra_variant: $enumeration = $extra_value;
            )*)?
        }
    };
}

pub(crate) use c_bits;
