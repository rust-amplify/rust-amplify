// Rust language amplification library providing multiple generic trait
// implementations, type wrappers, derive macros and other language enhancements
//
// Written in 2019-2022 by
//     Dr. Maxim Orlovsky <orlovsky@ubideco.org>
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

use core::error::Error;
use core::convert::Infallible;
use core::fmt::{Display, self, Formatter};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum MultiError<A: Error, B: Error, C: Error = Infallible> {
    A(A),
    B(B),
    C(C),
}

impl<A: Error, B: Error, C: Error> MultiError<A, B, C> {
    pub fn from_a(a: impl Into<A>) -> Self {
        Self::A(a.into())
    }
    pub fn from_b(a: impl Into<B>) -> Self {
        Self::B(a.into())
    }
    pub fn from_c(c: impl Into<C>) -> Self {
        Self::C(c.into())
    }

    pub fn from_other_a<A2: Error + Into<A>>(e: MultiError<A2, B, C>) -> Self {
        match e {
            MultiError::A(a) => Self::A(a.into()),
            MultiError::B(b) => Self::B(b),
            MultiError::C(c) => Self::C(c),
        }
    }

    pub fn from_other_b<B2: Error + Into<B>>(e: MultiError<A, B2, C>) -> Self {
        match e {
            MultiError::A(a) => Self::A(a),
            MultiError::B(b) => Self::B(b.into()),
            MultiError::C(c) => Self::C(c),
        }
    }

    pub fn from_other_c<C2: Error + Into<C>>(e: MultiError<A, B, C2>) -> Self {
        match e {
            MultiError::A(a) => Self::A(a),
            MultiError::B(b) => Self::B(b),
            MultiError::C(c) => Self::C(c.into()),
        }
    }
}

pub trait IntoMultiError: Error + Sized {
    fn into_multi_error(self) -> MultiError<Self, Infallible, Infallible>;
}

impl<E: Error> IntoMultiError for E {
    fn into_multi_error(self) -> MultiError<Self, Infallible, Infallible> {
        MultiError::A(self)
    }
}

impl<A: Error> MultiError<A, Infallible, Infallible> {
    pub fn from_error(e: impl Into<A>) -> Self {
        Self::A(e.into())
    }

    pub fn with_second<B: Error>(e: Self) -> MultiError<A, B, Infallible> {
        match e {
            Self::A(a) => MultiError::A(a),
            Self::B(_) => unreachable!(),
            Self::C(_) => unreachable!(),
        }
    }
}

impl<A: Error, B: Error> MultiError<A, B, Infallible> {
    pub fn with_third<C: Error>(e: Self) -> MultiError<A, B, C> {
        match e {
            Self::A(a) => MultiError::A(a),
            Self::B(b) => MultiError::B(b),
            Self::C(_) => unreachable!(),
        }
    }
}

impl<A: Error, B: Error, C: Error> Display for MultiError<A, B, C> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MultiError::A(e) => Display::fmt(e, f),
            MultiError::B(e) => Display::fmt(e, f),
            MultiError::C(e) => Display::fmt(e, f),
        }
    }
}

impl<A: Error, B: Error, C: Error> Error for MultiError<A, B, C> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            MultiError::A(e) => e.source(),
            MultiError::B(e) => e.source(),
            MultiError::C(e) => e.source(),
        }
    }
}
