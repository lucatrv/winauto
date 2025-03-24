// `winauto`: a Rust library to automate Windows applications with ease
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2025 Luca Trevisani <lucatrv@hotmail.com>

use std::fmt::{self, Display, Formatter};
use windows::Win32::System::Variant::{VARENUM, VARIANT};
use windows::core::*;

/// A thin wrapper around
/// [`VARIANT`](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Variant/struct.VARIANT.html)
/// with simple conversions.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Variant(VARIANT);

impl Display for Variant {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Variant {
    pub fn vt(&self) -> VARENUM {
        self.0.vt()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

macro_rules! impl_from_t_for_variant {
    ($($t:ty, $from:expr),+) => {
        $(
            impl From<$t> for Variant {
                fn from(value: $t) -> Self {
                    Self($from(value))
                }
            }
        )+
    };
}

impl_from_t_for_variant!(
    &str,
    VARIANT::from,
    String,
    |string| VARIANT::from(BSTR::from(string)),
    // ComObject,
    // |com_object: ComObject| VARIANT::from(com_object.0),
    bool,
    VARIANT::from,
    f32,
    VARIANT::from,
    f64,
    VARIANT::from,
    i8,
    VARIANT::from,
    i16,
    VARIANT::from,
    i32,
    VARIANT::from,
    i64,
    VARIANT::from,
    u8,
    VARIANT::from,
    u16,
    VARIANT::from,
    u32,
    VARIANT::from,
    u64,
    VARIANT::from
);

macro_rules! impl_tryfrom_variant_for_t {
    ($($t:ty, $try_from:expr),+) => {
        $(
            impl TryFrom<Variant> for $t {
                type Error = Error;

                fn try_from(variant: Variant) -> Result<Self> {
                    $try_from(&variant.0)
                }
            }

            impl TryFrom<&Variant> for $t {
                type Error = Error;

                fn try_from(variant: &Variant) -> Result<Self> {
                    $try_from(&variant.0)
                }
            }
        )+
    };
}

impl_tryfrom_variant_for_t!(
    String,
    |variant| Ok(BSTR::try_from(variant)?.to_string()),
    // ComObject,
    // |variant| Ok(ComObject(IDispatch::try_from(variant)?)),
    bool,
    bool::try_from,
    f64,
    f64::try_from,
    i16,
    i16::try_from,
    i32,
    i32::try_from,
    i64,
    i64::try_from,
    u16,
    u16::try_from,
    u32,
    u32::try_from,
    u64,
    u64::try_from
);
