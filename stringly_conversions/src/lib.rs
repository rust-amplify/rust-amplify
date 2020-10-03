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

//! A crate helping to convert to/from various representations of strings.
//!
//! This crate is `no_std` with an optional feature to enable `alloc`.

#![no_std]

#[cfg(feature = "alloc")]
pub extern crate alloc;

// We republish all supported external crates for access from the macros
#[cfg(feature = "serde_str_helpers")]
pub extern crate serde_str_helpers;

/// impls TryFrom<T> where T: Deref<Target=str> in terms of FromStr.
///
/// If your type implements `FromStr` then it could also implement `TryFrom<T>`
/// where `T` are various stringly types like `&str`, `String`, `Cow<'a, str>`...
/// Implementing these conversions as a blanket impl is impossible due to the
/// conflict with `T: Into<Self>`. Implementing them manually is tedious.
/// This macro will help you. However, take a look at
/// `impl_into_stringly_standard` which will help you even more!
///
/// This needs to be a macro instead of blanket imple in order to resolve the
/// conflict with T: Into<Self>
#[macro_export]
macro_rules! impl_try_from_stringly {
    ($to:ty $(, $from:ty)+ $(,)?) => {
        $(
            impl ::core::convert::TryFrom<$from> for $to {
                type Error = <$to as ::core::str::FromStr>::Err;
                #[inline]
                fn try_from(value: $from) -> Result<Self, Self::Error> {
                    <$to as core::str::FromStr>::from_str(&value)
                }
            }
        )*
    };

    (@std, $to:ty $(, $from:ty)+ $(,)?) => {
        $(
            #[cfg(feature = "std")]
            impl std::convert::TryFrom<$from> for $to {
                type Error = <$to as ::core::str::FromStr>::Err;
                #[inline]
                fn try_from(value: $from) -> Result<Self, Self::Error> {
                    <$to>::from_str(&value)
                }
            }
        )*
    }
}

/// Calls impl_try_from_stringly!() with a set of standard stringly types.
///
/// The currently supported types are:
///
/// * `Cow<'_, str>,`
/// * `Box<str>,`
/// * `Box<Cow<'_, str>>,`
/// * `Rc<str>,`
/// * `Rc<String>,`
/// * `Rc<Cow<'_, str>>,`
/// * `Arc<str>,`
/// * `Arc<String>,`
/// * `Arc<Cow<'_, str>>,`
///
/// Types from external crates:
/// * 
///
#[macro_export]
macro_rules! impl_try_from_stringly_standard {
    ($type:ty) => {
        mod __try_from_stringly_standard {
            use super::*;
            #[cfg(feature = "alloc")]
            use alloc::string::String;
            #[cfg(feature = "alloc")]
            use alloc::boxed::Box;
            #[cfg(feature = "alloc")]
            use alloc::borrow::Cow;
            #[cfg(feature = "alloc")]
            use alloc::rc::Rc;
            #[cfg(feature = "alloc")]
            use alloc::sync::Arc;

            impl_try_from_stringly! { $type,
                &str,
            }

            #[cfg(feature = "alloc")]
            impl_try_from_stringly! { $type,
                String,
                Cow<'_, str>,
                Box<str>,
                Box<Cow<'_, str>>,
                Rc<str>,
                Rc<String>,
                Rc<Cow<'_, str>>,
                Arc<str>,
                Arc<String>,
                Arc<Cow<'_, str>>,
            }

            #[cfg(feature = "serde_str_helpers")]
            impl_try_from_stringly!($type, $crate::serde_str_helpers::DeserBorrowStr<'_>);
        }
    };
}

/// Impls From<T> for Stringly where String: Into<Stringly>, T: Display
#[macro_export]
macro_rules! impl_into_stringly {
    ($from:ty $(, $into:ty)+ $(,)?) => {
        $(
            impl From<$from> for $into {
                fn from(value: $from) -> Self {
                    $crate::alloc::string::ToString::to_string(&value).into()
                }
            }
        )+
    }
}

/// Implements `impl_into_stringly` for `$type` and traits with `$type`
#[macro_export]
macro_rules! impl_into_stringly_standard {
    ($type:ty) => {
        mod __into_stringly_standard {
            use super::*;
            #[cfg(feature = "alloc")]
            use alloc::borrow::Cow;
            #[cfg(feature = "alloc")]
            use alloc::rc::Rc;
            #[cfg(feature = "alloc")]
            use alloc::sync::Arc;

            #[cfg(feature = "alloc")]
            impl_into_stringly! { $type,
                String,
                Cow<'_, str>,
                Box<str>,
                Rc<str>,
                Rc<String>,
                Arc<str>,
                Arc<String>,
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use core::convert::TryFrom;
    use core::fmt;

    #[cfg(feature = "alloc")]
    use alloc::string::String;
    #[cfg(feature = "alloc")]
    use alloc::boxed::Box;
    #[cfg(feature = "alloc")]
    use alloc::borrow::Cow;
    #[cfg(feature = "alloc")]
    use alloc::rc::Rc;
    #[cfg(feature = "alloc")]
    use alloc::sync::Arc;

    struct Number(u32);

    impl_try_from_stringly_standard!(Number);
    impl_into_stringly_standard!(Number);

    impl core::str::FromStr for Number {
        type Err = core::num::ParseIntError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            s.parse().map(Number)
        }
    }

    impl fmt::Display for Number {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }


    /* This doesn't compile because of missing hygiene

    struct Foo(u32);

    impl_try_from_stringly_standard!(Foo);

    impl core::str::FromStr for Foo {
        type Err = core::num::ParseIntError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            s.parse().map(Foo)
        }
    }
    */

    #[test]
    fn parse_str() {
        assert_eq!(Number::try_from("42").unwrap().0, 42);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn parse_alloc() {
        assert_eq!(Number::try_from(String::from("42")).unwrap().0, 42);
        assert_eq!(Number::try_from(<Cow<'_, str>>::from("42")).unwrap().0, 42);
        assert_eq!(Number::try_from(<Box<str>>::from("42")).unwrap().0, 42);
        assert_eq!(Number::try_from(<Rc<str>>::from("42")).unwrap().0, 42);
        assert_eq!(Number::try_from(Rc::new(String::from("42"))).unwrap().0, 42);
        assert_eq!(Number::try_from(<Arc<str>>::from("42")).unwrap().0, 42);
        assert_eq!(Number::try_from(Arc::new(String::from("42"))).unwrap().0, 42);
    }

    #[cfg(all(feature = "serde_str_helpers", feature = "alloc"))]
    #[test]
    fn test_serde_str_helpers() {
        assert_eq!(Number::try_from(serde_str_helpers::DeserBorrowStr::from(<Cow<'_, str>>::from("42"))).unwrap().0, 42);
    }

    #[test]
    fn display() {
        assert_eq!(&*<String>::from(Number(42)), "42");
        assert_eq!(&*<Cow<'_, str>>::from(Number(42)), "42");
        assert_eq!(&*<Box<str>>::from(Number(42)), "42");
        assert_eq!(&*<Rc<str>>::from(Number(42)), "42");
        assert_eq!(&*<Rc<String>>::from(Number(42)), "42");
        assert_eq!(&*<Arc<str>>::from(Number(42)), "42");
        assert_eq!(&*<Arc<String>>::from(Number(42)), "42");
    }
}
