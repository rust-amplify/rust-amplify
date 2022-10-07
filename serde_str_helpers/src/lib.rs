// Rust language amplification library providing multiple generic trait
// implementations, type wrappers, derive macros and other language enhancements
//
// Written in 2019-2020 by
//     Martin Habovstiak <martin.habovstiak@gmail.com>
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
//!
//! This crate is `no_std`, but **does** require `alloc`.

#![no_std]

extern crate alloc;
use alloc::borrow::{Borrow, Cow};
use alloc::string::String;
use core::ops::Deref;

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
/// Our DeserBorrowStr is written such that it borrows the `str` when possible,
/// avoiding the allocation. It may still need to allocate, for example if
/// string decoding (unescaping) has to be performed.
///
/// ## Example
///
/// ```
/// use serde_derive::Deserialize;
/// use serde_str_helpers::DeserBorrowStr;
/// use core::convert::TryFrom;
///
/// #[derive(Deserialize)]
/// #[serde(try_from = "DeserBorrowStr")]
/// struct StringlyNumber(u64);
///
/// impl<'a> TryFrom<DeserBorrowStr<'a>> for StringlyNumber {
///     type Error = core::num::ParseIntError;
///
///     fn try_from(value: DeserBorrowStr<'a>) -> Result<Self, Self::Error> {
///         value.parse().map(StringlyNumber)
///     }
/// }
///
/// let x = serde_json::from_str::<StringlyNumber>("\"42\"").expect("Failed to deserialize");
///
/// assert_eq!(x.0, 42);
/// ```
#[derive(serde_derive::Deserialize)]
pub struct DeserBorrowStr<'a>(#[serde(borrow)] Cow<'a, str>);

impl<'a> From<Cow<'a, str>> for DeserBorrowStr<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        DeserBorrowStr(value)
    }
}

impl<'a> From<DeserBorrowStr<'a>> for Cow<'a, str> {
    fn from(value: DeserBorrowStr<'a>) -> Self {
        value.0
    }
}

// Useful in cases conversion is conditional (saves allocation if not needed)
impl<'a> From<DeserBorrowStr<'a>> for String {
    fn from(value: DeserBorrowStr<'a>) -> Self {
        value.0.into_owned()
    }
}

impl<'a> Borrow<str> for DeserBorrowStr<'a> {
    fn borrow(&self) -> &str {
        self
    }
}

impl<'a> AsRef<str> for DeserBorrowStr<'a> {
    fn as_ref(&self) -> &str {
        self
    }
}

impl<'a> Deref for DeserBorrowStr<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::DeserBorrowStr;
    use alloc::borrow::{Borrow, Cow, ToOwned};
    use alloc::string::String;
    use core::convert::TryFrom;
    use core::fmt;

    #[test]
    fn actually_borrows_str() {
        #[derive(serde_derive::Deserialize)]
        #[serde(try_from = "DeserBorrowStr")]
        struct CheckDeser;

        #[derive(Debug)]
        enum Never {}

        impl fmt::Display for Never {
            fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
                match *self {}
            }
        }

        impl<'a> TryFrom<DeserBorrowStr<'a>> for CheckDeser {
            type Error = Never;

            fn try_from(value: DeserBorrowStr<'a>) -> Result<Self, Self::Error> {
                if let Cow::Owned(_) = value.into() {
                    panic!("String not borrowed");
                }

                Ok(CheckDeser)
            }
        }

        let _ = serde_json::from_str::<CheckDeser>(
            "\"The Times 03/Jan/2009 Chancellor on brink of second bailout for banks\"",
        )
        .unwrap();
    }

    #[test]
    fn conversions_cow() {
        // I have no clue why Rust fails to infer the type.
        let before_conversion = <Cow<'_, str>>::Borrowed("foo");
        let after_conversion = Cow::from(DeserBorrowStr::from(before_conversion.clone()));
        assert_eq!(after_conversion, before_conversion);

        let before_conversion = <Cow<'_, str>>::Owned("foo".to_owned());
        let after_conversion = Cow::from(DeserBorrowStr::from(before_conversion.clone()));
        assert_eq!(after_conversion, before_conversion);
    }

    #[test]
    fn conversions_string() {
        // I have no clue why Rust fails to infer the type.
        let before_conversion = <Cow<'_, str>>::Borrowed("foo");
        let after_conversion = String::from(DeserBorrowStr::from(before_conversion.clone()));
        assert_eq!(&*after_conversion, &*before_conversion);

        let before_conversion = <Cow<'_, str>>::Owned("foo".to_owned());
        let after_conversion = String::from(DeserBorrowStr::from(before_conversion.clone()));
        assert_eq!(&*after_conversion, &*before_conversion);
    }

    #[test]
    fn conversions_refs() {
        // I have no clue why Rust fails to infer the type.
        let borrowed = DeserBorrowStr::from(<Cow<'_, str>>::Borrowed("foo"));
        assert_eq!(borrowed.as_ref(), "foo");
        let borrowed: &str = borrowed.borrow();
        assert_eq!(borrowed, "foo");

        let owned = DeserBorrowStr::from(<Cow<'_, str>>::Owned("foo".to_owned()));
        assert_eq!(owned.as_ref(), "foo");
        let borrowed: &str = owned.borrow();
        assert_eq!(borrowed, "foo");
    }

    #[test]
    fn deref() {
        let string = "This isn't the kind of software where we can leave so many unresolved bugs that we need a tracker for them.";
        let wrapped = DeserBorrowStr::from(<Cow<'_, str>>::Borrowed(string));
        assert_eq!(&*wrapped, string);
    }
}
