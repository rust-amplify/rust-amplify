// Rust language amplification library providing multiple generic trait
// implementations, type wrappers, derive macros and other language enhancements
//
// Written in 2019-2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
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

//! This module contains various tools for converting values

/// impls TryFrom<T> where T: Deref<Target=str> in terms of FromStr.
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
                    <$to>::from_str(&value)
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
#[macro_export]
macro_rules! impl_try_from_stringly_standard {
    ($type:ty) => {
        #[cfg(feature = "std")]
        use ::std::borrow::Cow;
        #[cfg(feature = "std")]
        use ::std::rc::Rc;
        #[cfg(feature = "std")]
        use ::std::sync::Arc;

        impl_try_from_stringly! { $type,
            &str,
            String,
        }

        #[cfg(feature = "std")]
        impl_try_from_stringly! { @std $type,
            Cow<'_, str>
            Box<str>,
            Box<Cow<'_, str>>,
            Rc<str>,
            Rc<String>,
            Rc<Cow<'_, str>>,
            Arc<str>,
            Arc<String>,
            Arc<Cow<'_, str>>,
        }

        #[cfg(feature = "serde")]
        impl_try_from_stringly!($type, $crate::CowHelper<'_>);
    };
}

/// Impls From<T> for Stringly where String: Into<Stringly>, T: Display
#[macro_export]
macro_rules! impl_into_stringly {
    ($from:ty $(, $into:ty)+ $(,)?) => {
        $(
            impl From<$from> for $into {
                fn from(value: $from) -> Self {
                    value.to_string().into()
                }
            }
        )+
    }
}

#[macro_export]
macro_rules! impl_into_stringly_standard {
    ($type:ty) => {
        #[cfg(feature = "std")]
        use ::core::rc::Rc;
        #[cfg(feature = "std")]
        use ::core::sync::Arc;
        #[cfg(feature = "std")]
        use ::std::borrow::Cow;

        impl_into_stringly! { $type,
            String,
        }

        #[cfg(feature = "std")]
        impl_into_stringly! { @std, $type,
            Cow<'_, str>
            Box<str>,
            Rc<str>,
            Rc<String>,
            Arc<str>,
            Arc<String>,
        }
    };
}
