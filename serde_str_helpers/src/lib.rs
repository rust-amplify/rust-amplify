// Rust language amplification library providing multiple generic trait
// implementations, type wrappers, derive macros and other language enhancements
//
// Written in 2019-2020 by
//     Martin Habovstiak <martin.habovstiak@gmail.com>
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

//! This crate contains helpers for using serde with strings.
//!
//! Currently there is only a helper for deserializing stringly values more
//! efficiently by avoiding allocation (and copying) in certain cases.

use std::borrow::Cow;
use std::ops::Deref;

/// This is a helper for deserializing using `TryFrom` more efficiently.
///
/// When using `#[serde(try_from = "String"]` when deserializing a value that
/// doesn't neet to hold the string (e.g. an integer value) `serde` would
/// allocate the string even if it doesn't have to.
///
/// A naive idea is to use `std::borrow::Cow` to solve it. Sadly, the
/// implementation of Deserialize for Cow<'de, str> doesn't borrow the string,
/// so it still allocates needlessly. This helper solves the issue.
///
/// Our DeserStrHelper is written such that it borrows the `str` when possible,
/// avoiding the allocation. It may still need to allocate, for example if
/// string decoding (unescaping) has to be performed.
///
/// ## Example
///
/// ```
/// use serde_derive::Deserialize;
/// use serde_str_helpers::DeserStrHelper;
/// use std::convert::TryFrom;
///
/// #[derive(Deserialize)]
/// #[serde(try_from = "DeserStrHelper")]
/// struct StringlyNumber(u64);
///
/// impl<'a> TryFrom<DeserStrHelper<'a>> for StringlyNumber {
///     type Error = std::num::ParseIntError;
///
///     fn try_from(value: DeserStrHelper<'a>) -> Result<Self, Self::Error> {
///         value.parse().map(StringlyNumber)
///     }
/// }
///
/// let x = serde_json::from_str::<StringlyNumber>("\"42\"")
///     .expect("Failed to deserialize");
///
/// assert_eq!(x.0, 42);
/// ```
#[derive(serde_derive::Deserialize)]
pub struct DeserStrHelper<'a>(#[serde(borrow)] Cow<'a, str>);

impl<'a> From<Cow<'a, str>> for DeserStrHelper<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        DeserStrHelper(value)
    }
}

impl<'a> From<DeserStrHelper<'a>> for Cow<'a, str> {
    fn from(value: DeserStrHelper<'a>) -> Self {
        value.0
    }
}

impl<'a> Deref for DeserStrHelper<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::DeserStrHelper;
    use std::borrow::Cow;
    use std::convert::TryFrom;
    use std::fmt;

    #[test]
    fn actually_borrows_str() {
        #[derive(serde_derive::Deserialize)]
        #[serde(try_from = "DeserStrHelper")]
        struct CheckDeser;

        #[derive(Debug)]
        enum Never {}

        impl fmt::Display for Never {
            fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
                match *self {}
            }
        }

        impl<'a> TryFrom<DeserStrHelper<'a>> for CheckDeser {
            type Error = Never;

            fn try_from(value: DeserStrHelper<'a>) -> Result<Self, Self::Error> {
                if let Cow::Owned(_) = value.into() {
                    panic!("String not borrowed");
                }

                Ok(CheckDeser)
            }
        }

        let _ = serde_json::from_str::<CheckDeser>("\"The Times 03/Jan/2009 Chancellor on brink of second bailout for banks\"").unwrap();
    }
}
