// Rust language amplification library providing multiple generic trait
// implementations, type wrappers, derive macros and other language enhancements
//
// Written in 2021 by
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

use core::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign, BitAnd,
    BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Shl, ShlAssign, Shr, ShrAssign,
};
use core::convert::TryFrom;

use crate::error::{DivError, OverflowError};

macro_rules! construct_smallint {
    ($ty:ident, $inner:ident, $as:ident, $bits:literal, $max:expr, $doc:meta) => {
        #[$doc]
        #[derive(PartialEq, Eq, Debug, Copy, Clone, Default, PartialOrd, Ord, Hash)]
        #[cfg_attr(
            feature = "serde",
            derive(Serialize, Deserialize),
            serde(crate = "serde_crate", transparent)
        )]
        #[allow(non_camel_case_types)]
        pub struct $ty($inner);

        impl $ty {
            /// Bit dimension
            pub const BITS: u32 = $bits;

            /// Minimum value
            pub const MIN: Self = Self(0);

            /// Maximal value
            pub const MAX: Self = Self($max - 1);

            /// One value
            pub const ONE: Self = Self(1);

            /// One value
            pub const ZERO: Self = Self(0);

            /// Creates a new value from a provided `value.
            ///
            /// Panics if the value exceeds `Self::MAX`
            pub fn with(value: $inner) -> Self {
                assert!(value < $max, "provided value exceeds Self::MAX");
                Self(value)
            }

            /// Returns inner `u8` representation, which is always less or equal to `Self::MAX`
            pub fn $as(self) -> $inner {
                self.0 as $inner
            }
        }

        impl ::core::convert::TryFrom<$inner> for $ty {
            type Error = OverflowError;
            #[inline]
            fn try_from(value: $inner) -> Result<Self, Self::Error> {
                if value >= $max {
                    Err(OverflowError { max: $max as usize - 1, value: value as usize })
                } else {
                    Ok(Self(value))
                }
            }
        }

        impl From<$ty> for $inner {
            #[inline]
            fn from(val: $ty) -> Self {
                val.0
            }
        }

        impl AsRef<$inner> for $ty {
            #[inline]
            fn as_ref(&self) -> &$inner {
                &self.0
            }
        }

        impl ::core::str::FromStr for $ty {
            type Err = ::core::num::ParseIntError;
            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::try_from($inner::from_str(s)?).map_err(|_| u8::from_str("257").unwrap_err())
            }
        }

        impl ::core::fmt::Display for $ty {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl core::fmt::UpperHex for $ty {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
                core::fmt::UpperHex::fmt(&self.as_ref(), f)
            }
        }

        impl core::fmt::LowerHex for $ty {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
                core::fmt::LowerHex::fmt(&self.as_ref(), f)
            }
        }

        impl core::fmt::Octal for $ty {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
                core::fmt::Octal::fmt(&self.as_ref(), f)
            }
        }

        impl core::fmt::Binary for $ty {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
                core::fmt::Binary::fmt(&self.as_ref(), f)
            }
        }

        impl_op!($ty, $inner, Add, add, AddAssign, add_assign, +);
        impl_op!($ty, $inner, Sub, sub, SubAssign, sub_assign, -);
        impl_op!($ty, $inner, Mul, mul, MulAssign, mul_assign, *);
        impl_op!($ty, $inner, Div, div, DivAssign, div_assign, /);
        impl_op!($ty, $inner, Rem, rem, RemAssign, rem_assign, %);
        impl_op!($ty, $inner, BitAnd, bitand, BitAndAssign, bitand_assign, &);
        impl_op!($ty, $inner, BitOr, bitor, BitOrAssign, bitor_assign, |);
        impl_op!($ty, $inner, BitXor, bitxor, BitXorAssign, bitxor_assign, ^);
        impl_op!($ty, $inner, Shl, shl, ShlAssign, shl_assign, <<);
        impl_op!($ty, $inner, Shr, shr, ShrAssign, shr_assign, >>);

        impl $ty {
            /// Checked integer addition. Computes `self + rhs`, returning `None` if
            /// overflow occurred.
            pub fn checked_add<T>(self, rhs: T) -> Option<Self> where T: Into<$inner> {
                self.0.checked_add(rhs.into()).and_then(|val| Self::try_from(val).ok())
            }
            /// Saturating integer addition. Computes `self + rhs`, saturating at the
            /// numeric bounds instead of overflowing.
            pub fn saturating_add<T>(self, rhs: T) -> Self where T: Into<$inner> {
                let res = self.0.saturating_add(rhs.into());
                if res > Self::MAX.$as() {
                    Self::MAX
                } else {
                    Self(res)
                }
            }
            /// Calculates `self + rhs`
            ///
            /// Returns a tuple of the addition along with a boolean indicating whether
            /// an arithmetic overflow would occur. If an overflow would have occurred
            /// then the wrapped value is returned.
            pub fn overflowing_add<T>(self, rhs: T) -> (Self, bool) where T: Into<$inner> {
                let mut ret = self.0.overflowing_add(rhs.into());
                if ret.0 > Self::MAX.0 {
                    ret.0 %= Self::MAX.0;
                    ret.1 = true;
                }
                (Self(ret.0), ret.1)
            }
            /// Wrapping (modular) addition. Computes `self + rhs`, wrapping around at
            /// the boundary of the type.
            pub fn wrapping_add<T>(self, rhs: T) -> Self where T: Into<$inner> {
                Self(self.0.wrapping_add(rhs.into()) % Self::MAX.0)
            }

            /// Checked integer subtraction. Computes `self - rhs`, returning `None` if
            /// overflow occurred.
            pub fn checked_sub<T>(self, rhs: T) -> Option<Self> where T: Into<$inner> {
                self.0.checked_sub(rhs.into()).and_then(|val| Self::try_from(val).ok())
            }
            /// Saturating integer subtraction. Computes `self - rhs`, saturating at the
            /// numeric bounds instead of overflowing.
            pub fn saturating_sub<T>(self, rhs: T) -> Self where T: Into<$inner> {
                let res = self.0.saturating_sub(rhs.into());
                if res > Self::MAX.$as() {
                    Self::MAX
                } else {
                    Self(res)
                }
            }
            /// Calculates `self - rhs`
            ///
            /// Returns a tuple of the subtraction along with a boolean indicating whether
            /// an arithmetic overflow would occur. If an overflow would have occurred
            /// then the wrapped value is returned.
            pub fn overflowing_sub<T>(self, rhs: T) -> (Self, bool) where T: Into<$inner> {
                let mut ret = self.0.overflowing_sub(rhs.into());
                if ret.0 > Self::MAX.0 {
                    ret.0 %= Self::MAX.0;
                    ret.1 = true;
                }
                (Self(ret.0), ret.1)
            }
            /// Wrapping (modular) subtraction. Computes `self - rhs`, wrapping around at
            /// the boundary of the type.
            pub fn wrapping_sub<T>(self, rhs: T) -> Self where T: Into<$inner> {
                Self(self.0.wrapping_sub(rhs.into()) % Self::MAX.0)
            }

            /// Checked integer multiplication. Computes `self * rhs`, returning `None` if
            /// overflow occurred.
            pub fn checked_mul<T>(self, rhs: T) -> Option<Self> where T: Into<$inner> {
                self.0.checked_mul(rhs.into()).and_then(|val| Self::try_from(val).ok())
            }
            /// Saturating integer multiplication. Computes `self * rhs`, saturating at the
            /// numeric bounds instead of overflowing.
            pub fn saturating_mul<T>(self, rhs: T) -> Self where T: Into<$inner> {
                let res = self.0.saturating_mul(rhs.into());
                if res > Self::MAX.0 {
                    Self::MAX
                } else {
                    Self(res)
                }
            }
            /// Calculates `self * rhs`
            ///
            /// Returns a tuple of the multiplication along with a boolean indicating whether
            /// an arithmetic overflow would occur. If an overflow would have occurred
            /// then the wrapped value is returned.
            pub fn overflowing_mul<T>(self, rhs: T) -> (Self, bool) where T: Into<$inner> {
                let mut ret = self.0.overflowing_mul(rhs.into());
                if ret.0 > Self::MAX.0 {
                    ret.0 %= Self::MAX.0;
                    ret.1 = true;
                }
                (Self(ret.0), ret.1)
            }
            /// Wrapping (modular) multiplication. Computes `self * rhs`, wrapping around at
            /// the boundary of the type.
            pub fn wrapping_mul<T>(self, rhs: T) -> Self where T: Into<$inner> {
                Self(self.0.wrapping_mul(rhs.into()) % Self::MAX.0)
            }

            #[inline]
            pub fn div_rem(self, other: Self) -> Result<(Self, Self), DivError> {
                //quotient and remainder will always be smaller than self so they're going to be in bounds
                if other == Self(0) {
                    return Err(DivError::ZeroDiv)
                }
                let quotient = self / other;
                Ok((quotient, self - (quotient * other)))
            }
        }
    };
}
macro_rules! impl_op {
    ($ty:ty, $inner:ty, $op:ident, $fn:ident, $op_assign:ident, $fn_assign:ident, $sign:tt) => {
        impl<T> $op<T> for $ty where T: Into<$inner> {
            type Output = $ty;
            #[inline]
            fn $fn(self, rhs: T) -> Self::Output {
                Self::try_from((self.0).$fn(rhs.into())).expect(stringify!(
                    "attempt to ",
                    $fn,
                    " with overflow"
                ))
            }
        }
        impl<T> $op<T> for &$ty where T: Into<$inner> {
            type Output = $ty;
            #[inline]
            fn $fn(self, rhs: T) -> Self::Output {
                *self $sign rhs
            }
        }

        impl<T> $op_assign<T> for $ty where T: Into<$inner> {
            #[inline]
            fn $fn_assign(&mut self, rhs: T) {
                self.0 = (*self $sign rhs).0
            }
        }
    };
}

construct_smallint!(
    u1,
    u8,
    as_u8,
    1,
    2,
    doc =
        "1-bit unsigned integer in the range `0..1`. It can be used instead of `bool` when 1-bit \
    numeric (and not boolean) arithmetic is required"
);
construct_smallint!(
    u2,
    u8,
    as_u8,
    2,
    4,
    doc = "2-bit unsigned integer in the range `0..4`"
);
construct_smallint!(
    u3,
    u8,
    as_u8,
    3,
    8,
    doc = "3-bit unsigned integer in the range `0..8`"
);
construct_smallint!(
    u4,
    u8,
    as_u8,
    4,
    16,
    doc = "4-bit unsigned integer in the range `0..16`"
);
construct_smallint!(
    u5,
    u8,
    as_u8,
    5,
    32,
    doc = "5-bit unsigned integer in the range `0..32`"
);
construct_smallint!(
    u6,
    u8,
    as_u8,
    6,
    64,
    doc = "6-bit unsigned integer in the range `0..64`"
);
construct_smallint!(
    u7,
    u8,
    as_u8,
    7,
    128,
    doc = "7-bit unsigned integer in the range `0..128`"
);
construct_smallint!(
    u24,
    u32,
    as_u32,
    24,
    1u32 << 24,
    doc = "24-bit unsigned integer in the range `0..16_777_216`"
);

impl u24 {
    /// Create a native endian integer value from its representation as a byte
    /// array in little endian.
    pub fn from_le_bytes(bytes: [u8; 3]) -> u24 {
        let mut inner = [0u8; 4];
        inner[..3].copy_from_slice(&bytes);
        Self(u32::from_le_bytes(inner))
    }

    /// Return the memory representation of this integer as a byte array in
    /// little-endian byte order.
    pub fn to_le_bytes(self) -> [u8; 3] {
        let mut inner = [0u8; 3];
        inner.copy_from_slice(&self.0.to_le_bytes()[..3]);
        inner
    }

    /// Create a native endian integer value from its representation as a byte
    /// array in big endian.
    pub fn from_be_bytes(bytes: [u8; 3]) -> u24 {
        let mut inner = [0u8; 4];
        inner[1..].copy_from_slice(&bytes);
        Self(u32::from_be_bytes(inner))
    }

    /// Return the memory representation of this integer as a byte array in
    /// big-endian byte order.
    pub fn to_be_bytes(self) -> [u8; 3] {
        let mut inner = [0u8; 3];
        inner.copy_from_slice(&self.0.to_be_bytes()[1..]);
        inner
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ubit_test() {
        let mut u_2 = u2::try_from(u2::MAX.as_u8()).unwrap();
        let mut u_3 = u3::try_from(u3::MAX.as_u8()).unwrap();
        let mut u_4 = u4::try_from(u4::MAX.as_u8()).unwrap();
        let mut u_5 = u5::try_from(u5::MAX.as_u8()).unwrap();
        let mut u_6 = u6::try_from(u6::MAX.as_u8()).unwrap();
        let mut u_7 = u7::try_from(u7::MAX.as_u8()).unwrap();
        let mut u_24 = u24::try_from(u24::MAX.as_u32()).unwrap();

        assert_eq!(u_2, u2::with(3));
        assert_eq!(u_3, u3::with(7));
        assert_eq!(u_4, u4::with(15));
        assert_eq!(u_5, u5::with(31));
        assert_eq!(u_6, u6::with(63));
        assert_eq!(u_7, u7::with(127));

        assert_eq!(u_2.as_u8(), 3u8);
        assert_eq!(u_3.as_u8(), 7u8);
        assert_eq!(u_4.as_u8(), 15u8);
        assert_eq!(u_5.as_u8(), 31u8);
        assert_eq!(u_6.as_u8(), 63u8);
        assert_eq!(u_7.as_u8(), 127u8);
        assert_eq!(u_24.as_u32(), (1 << 24) - 1);

        u_2 -= 1;
        u_3 -= 1;
        u_4 -= 1;
        u_5 -= 1;
        u_6 -= 1;
        u_7 -= 1;
        u_24 -= 1u32;

        assert_eq!(u_2.as_u8(), 2u8);
        assert_eq!(u_3.as_u8(), 6u8);
        assert_eq!(u_4.as_u8(), 14u8);
        assert_eq!(u_5.as_u8(), 30u8);
        assert_eq!(u_6.as_u8(), 62u8);
        assert_eq!(u_7.as_u8(), 126u8);
        assert_eq!(u_24.as_u32(), (1 << 24) - 2);

        u_2 /= 2;
        u_2 *= 2;
        u_2 += 1;

        u_3 /= 2;
        u_3 *= 2;
        u_3 += 1;

        u_4 /= 2;
        u_4 *= 2;
        u_4 += 1;

        u_5 /= 2;
        u_5 *= 2;
        u_5 += 1;

        u_6 /= 2;
        u_6 *= 2;
        u_6 += 1;

        u_7 /= 2;
        u_7 *= 2;
        u_7 += 1;

        u_24 /= 2u32;
        u_24 *= 2u32;
        u_24 += 1u32;

        assert_eq!(u_2.as_u8(), 3u8);
        assert_eq!(u_3.as_u8(), 7u8);
        assert_eq!(u_4.as_u8(), 15u8);
        assert_eq!(u_5.as_u8(), 31u8);
        assert_eq!(u_6.as_u8(), 63u8);
        assert_eq!(u_7.as_u8(), 127u8);
        assert_eq!(u_24.as_u32(), (1 << 24) - 1);

        assert_eq!(u_2.as_u8() % 2, 1);
        assert_eq!(u_3.as_u8() % 2, 1);
        assert_eq!(u_4.as_u8() % 2, 1);
        assert_eq!(u_5.as_u8() % 2, 1);
        assert_eq!(u_6.as_u8() % 2, 1);
        assert_eq!(u_7.as_u8() % 2, 1);
        assert_eq!(u_24.as_u32() % 2, 1);
    }

    #[test]
    #[should_panic(expected = "OverflowError { max: 3, value: 4 }")]
    fn u2_overflow_test() {
        u2::try_from(4).unwrap();
    }

    #[test]
    #[should_panic(expected = "OverflowError { max: 7, value: 8 }")]
    fn u3_overflow_test() {
        u3::try_from(8).unwrap();
    }

    #[test]
    #[should_panic(expected = "OverflowError { max: 15, value: 16 }")]
    fn u4_overflow_test() {
        u4::try_from(16).unwrap();
    }

    #[test]
    #[should_panic(expected = "OverflowError { max: 31, value: 32 }")]
    fn u5_overflow_test() {
        u5::try_from(32).unwrap();
    }

    #[test]
    #[should_panic(expected = "OverflowError { max: 63, value: 64 }")]
    fn u6_overflow_test() {
        u6::try_from(64).unwrap();
    }

    #[test]
    #[should_panic(expected = "OverflowError { max: 127, value: 128 }")]
    fn u7_overflow_test() {
        u7::try_from(128).unwrap();
    }

    #[test]
    #[should_panic(expected = "OverflowError { max: 16777215, value: 16777216 }")]
    fn u24_overflow_test() {
        u24::try_from(1 << 24).unwrap();
    }

    #[test]
    fn u24_endianess() {
        let val: u32 = 0x00adbeef;
        let le = [0xef, 0xbe, 0xad];
        let v1 = u24::with(val);
        assert_eq!(v1.as_u32(), val);
        assert_eq!(v1.to_le_bytes(), le);
        let v2 = u24::from_le_bytes(le);
        assert_eq!(v2.to_le_bytes(), le);
        assert_eq!(v2, v1);
        assert_eq!(v2.as_u32(), v1.as_u32());
    }

    #[test]
    fn smallint_div_rem_0() {
        let u_2 = u2::MAX;
        let u_2_2 = u2::try_from(2).unwrap();
        let u_2_half = (
            u2::try_from(u2::MAX / 2).unwrap(),
            u2::try_from(u2::MAX % 2).unwrap(),
        );
        let u_2_zero = u2::ZERO;

        assert_eq!(u2::div_rem(u_2, u_2_2), Ok(u_2_half));
        assert_eq!(u2::div_rem(u_2, u_2_zero), Err(DivError::ZeroDiv));
    }

    #[test]
    fn smallint_div_rem() {
        let u_2 = u2::MAX;
        let u_2_zero = u2::ZERO;
        assert_eq!(u2::div_rem(u_2, u_2_zero), Err(DivError::ZeroDiv));
    }

    #[test]
    fn fmt_test() {
        let u_2 = u2::MAX;
        let u_3 = u3::MAX;
        let u_4 = u4::MAX;
        let u_5 = u5::MAX;
        let u_6 = u6::MAX;
        let u_7 = u7::MAX;
        let u_24 = u24::MAX;

        // UpperHex
        assert_eq!(format!("{:X}", u_2), "3");
        assert_eq!(format!("{:X}", u_3), "7");
        assert_eq!(format!("{:X}", u_4), "F");
        assert_eq!(format!("{:X}", u_5), "1F");
        assert_eq!(format!("{:X}", u_6), "3F");
        assert_eq!(format!("{:X}", u_7), "7F");
        assert_eq!(format!("{:X}", u_24), "FFFFFF");
        assert_eq!(format!("{:#X}", u_2), "0x3");
        assert_eq!(format!("{:#X}", u_3), "0x7");
        assert_eq!(format!("{:#X}", u_4), "0xF");
        assert_eq!(format!("{:#X}", u_5), "0x1F");
        assert_eq!(format!("{:#X}", u_6), "0x3F");
        assert_eq!(format!("{:#X}", u_7), "0x7F");
        assert_eq!(format!("{:#X}", u_24), "0xFFFFFF");

        // LowerHex
        assert_eq!(format!("{:x}", u_2), "3");
        assert_eq!(format!("{:x}", u_3), "7");
        assert_eq!(format!("{:x}", u_4), "f");
        assert_eq!(format!("{:x}", u_5), "1f");
        assert_eq!(format!("{:x}", u_6), "3f");
        assert_eq!(format!("{:x}", u_7), "7f");
        assert_eq!(format!("{:x}", u_24), "ffffff");
        assert_eq!(format!("{:#x}", u_2), "0x3");
        assert_eq!(format!("{:#x}", u_3), "0x7");
        assert_eq!(format!("{:#x}", u_4), "0xf");
        assert_eq!(format!("{:#x}", u_5), "0x1f");
        assert_eq!(format!("{:#x}", u_6), "0x3f");
        assert_eq!(format!("{:#x}", u_7), "0x7f");
        assert_eq!(format!("{:#x}", u_24), "0xffffff");

        // Octal
        assert_eq!(format!("{:o}", u_2), "3");
        assert_eq!(format!("{:o}", u_3), "7");
        assert_eq!(format!("{:o}", u_4), "17");
        assert_eq!(format!("{:o}", u_5), "37");
        assert_eq!(format!("{:o}", u_6), "77");
        assert_eq!(format!("{:o}", u_7), "177");
        assert_eq!(format!("{:o}", u_24), "77777777");
        assert_eq!(format!("{:#o}", u_2), "0o3");
        assert_eq!(format!("{:#o}", u_3), "0o7");
        assert_eq!(format!("{:#o}", u_4), "0o17");
        assert_eq!(format!("{:#o}", u_5), "0o37");
        assert_eq!(format!("{:#o}", u_6), "0o77");
        assert_eq!(format!("{:#o}", u_7), "0o177");
        assert_eq!(format!("{:#o}", u_24), "0o77777777");

        // Binary
        assert_eq!(format!("{:b}", u_2), "11");
        assert_eq!(format!("{:b}", u_3), "111");
        assert_eq!(format!("{:b}", u_4), "1111");
        assert_eq!(format!("{:b}", u_5), "11111");
        assert_eq!(format!("{:b}", u_6), "111111");
        assert_eq!(format!("{:b}", u_7), "1111111");
        assert_eq!(format!("{:b}", u_24), "111111111111111111111111");
        assert_eq!(format!("{:#b}", u_2), "0b11");
        assert_eq!(format!("{:#b}", u_3), "0b111");
        assert_eq!(format!("{:#b}", u_4), "0b1111");
        assert_eq!(format!("{:#b}", u_5), "0b11111");
        assert_eq!(format!("{:#b}", u_6), "0b111111");
        assert_eq!(format!("{:#b}", u_7), "0b1111111");
        assert_eq!(format!("{:#b}", u_24), "0b111111111111111111111111");
    }
}
