// Rust language amplification library providing multiple generic trait
// implementations, type wrappers, derive macros and other language enhancements
//
// Written in 2022 by
//     Yudai Kiyofuji <own7000hr@gmail.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

use crate::{u256, u512, u1024};
use core::ops::Add;

macro_rules! construct_posit {
    ($name:ident, $bits:expr, $es:expr, $internal:ident, $zeros: expr, $ones: expr, $nar: expr, $guard:ident) => {
        #[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
        pub struct $name($internal);

        impl $name {
            pub const ZERO: $name = $name($zeros);
            pub const NAR: $name = $name($nar);

            #[inline]
            pub fn as_inner(&self) -> &$internal {
                &self.0
            }

            #[inline]
            pub fn into_inner(self) -> $internal {
                self.0
            }

            #[inline]
            pub fn is_nar(&self) -> bool {
                self == &Self::NAR
            }

            #[inline]
            pub fn is_zero(&self) -> bool {
                self.0 == $zeros
            }

            #[inline]
            pub fn is_negative(&self) -> bool {
                !self.is_nar() && (self.0 & Self::NAR.0 == Self::NAR.0)
            }

            #[inline]
            pub fn is_positive(&self) -> bool {
                !self.is_nar() && !self.is_negative() && !self.is_zero()
            }

            #[inline]
            pub fn abs(self) -> Self {
                match self.is_negative() {
                    true => -self,
                    false => self,
                }
            }

            fn regime<T: Into<i32>>(exp: T) -> (i16, $internal) {
                let exp = exp.into();
                let regime = exp >> $es;
                (
                    regime as i16,
                    $internal::from((exp - (regime << $es)) as u8),
                )
            }

            fn decode(&self) -> Option<(bool, i16, $internal, $internal)> {
                match (self.is_zero(), self.is_nar()) {
                    (false, false) => (),
                    _ => return None,
                }
                let sign = self.is_negative();
                let input = self.abs().0 << 1;
                let (regime, input) = match ((!input).leading_zeros(), input.leading_zeros()) {
                    (0, zeros) => (-(zeros as i16), input << (zeros + 1) as usize),
                    (ones, _) => ((ones - 1) as i16, input << (ones + 1) as usize),
                };
                let exp = input.checked_shr($bits - $es).unwrap_or($zeros);
                let mantissa = input << $es;
                Some((sign, regime, exp, mantissa))
            }

            pub fn with(sign: bool, regime: i16, exp: $internal, mantissa: $internal) -> Self {
                let shl = |x: $internal, shift| x.checked_shl(shift).unwrap_or($zeros);
                let shr = |x: $internal, shift| x.checked_shr(shift).unwrap_or($zeros);
                let mut ret = $zeros;
                let len: u32 = (regime.abs() as u16 + (!regime.is_negative() as u16)).into();
                let regime_mask = match regime.is_negative() {
                    true => shr(!($ones >> 1), len + 1),
                    false => ($ones ^ shr($ones, len)) >> 1,
                };
                let mantissa_mask = shr(mantissa, len + $es + 2);
                let fraction = shl(mantissa, len + $es + 2);
                let (exp_mask, fraction) = match $bits < $es + len + 2 {
                    true => (shr(exp, $es + len + 2 - $bits), shl(exp, $es + len + 2)),
                    false => (shl(exp, $bits - len - 2 - $es), fraction),
                };
                ret |= regime_mask | exp_mask | mantissa_mask;
                match (
                    ret == $ones,
                    ret == ($ones >> 1),
                    fraction.cmp(&!($ones >> 1)),
                ) {
                    (true, _, _) | (_, true, _) | (_, _, ::core::cmp::Ordering::Less) => (),
                    (_, _, ::core::cmp::Ordering::Greater) => ret += !($ones << 1),
                    (_, _, ::core::cmp::Ordering::Equal) => ret += (ret & !($ones << 1)),
                };
                Self(if sign { ret.wrapping_neg() } else { ret })
            }
        }

        impl PartialOrd for $name {
            #[inline]
            fn partial_cmp(&self, other: &$name) -> Option<::core::cmp::Ordering> {
                Some(self.cmp(&other))
            }
        }

        impl Ord for $name {
            #[inline]
            fn cmp(&self, other: &$name) -> ::core::cmp::Ordering {
                match (self.is_nar(), other.is_nar()) {
                    (true, true) => return ::core::cmp::Ordering::Equal,
                    (true, false) => return ::core::cmp::Ordering::Less,
                    (false, true) => return ::core::cmp::Ordering::Greater,
                    _ => (),
                }
                match (self.is_negative(), other.is_negative()) {
                    (false, true) => ::core::cmp::Ordering::Greater,
                    (true, false) => ::core::cmp::Ordering::Less,
                    _ => self.0.cmp(&other.0),
                }
            }
        }

        impl ::core::ops::Neg for $name {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self(self.0.wrapping_neg())
            }
        }

        impl<T> ::core::ops::Add<T> for $name
        where
            T: Into<$name>,
        {
            type Output = $name;
            fn add(self, other: T) -> $name {
                let other = other.into();
                if self.is_nar() || other.is_nar() {
                    return Self::NAR;
                }
                if self.is_zero() {
                    return other;
                }
                if other.is_zero() {
                    return self;
                }
                let (lhs, rhs, sign) = {
                    let (l, r) = (self.abs(), other.abs());
                    match (l > r, self.is_negative(), other.is_negative()) {
                        (true, true, true) => (l, r, true),
                        (true, true, false) => (l, r, true),
                        (true, false, true) => (l, r, false),
                        (true, false, false) => (l, r, false),
                        (false, true, true) => (r, l, true),
                        (false, false, true) => (r, l, true),
                        (false, true, false) => (r, l, false),
                        (false, false, false) => (r, l, false),
                    }
                };
                let (lhs, rhs) = (lhs.decode().unwrap(), rhs.decode().unwrap());
                let exp_lhs = ((lhs.1 as i32) << $es) + lhs.2.to_le_bytes()[0] as i32;
                let exp_rhs = ((rhs.1 as i32) << $es) + rhs.2.to_le_bytes()[0] as i32;
                let shift = (exp_lhs - exp_rhs) as u32;
                let mantissa_lhs = (lhs.3 >> 2) | (Self::NAR.0 >> 1);
                let mantissa_rhs = ((rhs.3 >> 1) | Self::NAR.0)
                    .checked_shr(shift + 1)
                    .unwrap_or($zeros);
                let mut mantissa = match self.is_negative() == other.is_negative() {
                    true => mantissa_lhs + mantissa_rhs,
                    false => mantissa_lhs - mantissa_rhs,
                };
                let leading_zeros = mantissa.leading_zeros();
                let scaling_factor = 1 - leading_zeros as i32;
                mantissa <<= leading_zeros as usize + 1;
                let (regime, exp) = Self::regime(exp_lhs + scaling_factor);
                Self::with(sign, regime, exp, mantissa)
            }
        }

        impl<T> ::core::ops::Sub<T> for $name
        where
            T: Into<$name>,
        {
            type Output = $name;
            fn sub(self, other: T) -> $name {
                self.add(-(other.into()))
            }
        }

        impl<T> ::core::ops::Mul<T> for $name
        where
            T: Into<$name>,
        {
            type Output = $name;
            fn mul(self, other: T) -> $name {
                let other = other.into();
                let sign = self.is_negative() != other.is_negative();
                if self.is_nar() || other.is_nar() {
                    return Self::NAR;
                }
                if self.is_zero() || other.is_zero() {
                    return Self::ZERO;
                }
                let (lhs, rhs) = (self.decode().unwrap(), other.decode().unwrap());
                let exp_lhs = ((lhs.1 as i32) << $es) + lhs.2.to_le_bytes()[0] as i32;
                let exp_rhs = ((rhs.1 as i32) << $es) + rhs.2.to_le_bytes()[0] as i32;
                let mantissa_lhs = $guard::from((lhs.3 >> 2) | (Self::NAR.0 >> 1));
                let mantissa_rhs = $guard::from((rhs.3 >> 2) | (Self::NAR.0 >> 1));
                let mut mantissa = mantissa_lhs * mantissa_rhs;
                let shift = mantissa.leading_zeros() + 1;
                let scaling_factor = 4 - shift as i32;
                mantissa <<= shift as usize;
                let mantissa = {
                    let mut x = [0u8; $bits / 8];
                    let bytes = mantissa.to_le_bytes();
                    for i in 0..($bits / 8) {
                        x[i] = bytes[$bits / 8 + i]
                    }
                    $internal::from_le_bytes(x)
                };
                let (regime, exp) = Self::regime(exp_lhs + exp_rhs + scaling_factor);
                Self::with(sign, regime, exp, mantissa)
            }
        }

        impl ::core::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let &$name(ref data) = self;
                write!(f, "{:?}", data)?;
                Ok(())
            }
        }

        impl From<f32> for $name {
            fn from(init: f32) -> $name {
                if init == 0. {
                    return Self::ZERO;
                }
                if init.is_infinite() || init.is_nan() {
                    return Self::NAR;
                }
                let bits = (if init.is_sign_negative() { -init } else { init }).to_bits();
                let mut mantissa = [0u8; $bits / 8];
                let _ = (bits << 9)
                    .to_be_bytes()
                    .iter()
                    .enumerate()
                    .filter(|&(i, _)| i < $bits / 8)
                    .map(|(i, e)| mantissa[i] = *e)
                    .collect::<()>();
                let (mantissa, init_exp) = match init.is_normal() {
                    true => (
                        $internal::from_be_bytes(mantissa),
                        (bits >> 23) as i16 - 127,
                    ),
                    false => {
                        let m = $internal::from_be_bytes(mantissa);
                        let shift = m.leading_zeros() + 1;
                        (m << shift as usize, -126 - shift as i16)
                    }
                };
                let (regime, exp) = Self::regime(init_exp);
                Self::with(init.is_sign_negative(), regime, exp, mantissa)
            }
        }
    };
}

construct_posit!(Posit8, 8, 0, u8, 0, ::core::u8::MAX, 0x80, u16);
construct_posit!(Posit16, 16, 1, u16, 0, ::core::u16::MAX, 0x8000, u32);
construct_posit!(Posit32, 32, 2, u32, 0, ::core::u32::MAX, 0x8000_0000, u64);
construct_posit!(
    Posit64,
    64,
    3,
    u64,
    0,
    ::core::u64::MAX,
    0x8000_0000_0000_0000,
    u128
);
construct_posit!(
    Posi128,
    128,
    4,
    u128,
    0,
    ::core::u128::MAX,
    0x8000_0000_0000_0000_0000_0000_0000_0000,
    u256
);
construct_posit!(
    Posit256,
    256,
    5,
    u256,
    u256::ZERO,
    u256::MAX,
    u256::from_inner([0, 0, 0, 0x8000_0000_0000_0000]),
    u512
);
construct_posit!(
    Posit512,
    512,
    6,
    u512,
    u512::ZERO,
    u512::MAX,
    u512::from_inner([0, 0, 0, 0, 0, 0, 0, 0x8000_0000_0000_0000]),
    u1024
);

#[cfg(test)]
mod tests {
    #![allow(unused)]

    use super::*;

    construct_posit!(Posit8Es1, 8, 1, u8, 0, 0xff, 0x80, u16);

    #[test]
    fn posit_test() {
        assert_eq!(Posit16::from(1.).into_inner(), 0b0100_0000_0000_0000);
        assert_eq!(Posit16::from(1.125).into_inner(), 0b0100_0010_0000_0000);
        assert_eq!(Posit16::from(3.25).into_inner(), 0b0101_1010_0000_0000);
        assert_eq!(Posit16::from(4.).into_inner(), 0b0110_0000_0000_0000);
        assert_eq!(Posit16::from(8.).into_inner(), 0b0110_1000_0000_0000);
        assert_eq!(Posit16::from(1024.).into_inner(), 0b0111_1110_0000_0000);
        assert_eq!(Posit16::from(-10.).into_inner(), 0b1001_0110_0000_0000);
        assert_eq!(Posit16::from(-7. / 16.).into_inner(), 0b1101_0100_0000_0000);
        assert_eq!(Posit16::from(-256.).into_inner(), 0b1000_0100_0000_0000);
        assert_eq!(Posit16::from(0.).into_inner(), 0b0000_0000_0000_0000);
        assert_eq!(Posit16::from(-0.).into_inner(), 0b0000_0000_0000_0000);
    }

    #[test]
    fn posit_from_subnormal_test() {
        let sub = f32::from_bits(0b0000_0000_0000_1000 << 16); //2 ^ (-130)
        assert!(!sub.is_normal());
        // With es = 3, regime is -17 and exp is 6. -17 * 8 + 6 = 130
        assert_eq!(
            Posit64::from(sub).into_inner(),
            0b0000_0000_0000_0000_0011_1000 << 40
        );
    }

    #[test]
    fn posit8es1_test() {
        assert_eq!(Posit8Es1::from(1.).into_inner(), 0b0100_0000);
        assert_eq!(Posit8Es1::from(1.125).into_inner(), 0b0100_0010);
        assert_eq!(Posit8Es1::from(3.25).into_inner(), 0b0101_1010);
        assert_eq!(Posit8Es1::from(4.).into_inner(), 0b0110_0000);
        assert_eq!(Posit8Es1::from(8.).into_inner(), 0b0110_1000);
        assert_eq!(Posit8Es1::from(1024.).into_inner(), 0b0111_1110);
        assert_eq!(Posit8Es1::from(-10.).into_inner(), 0b1001_0110);
        assert_eq!(Posit8Es1::from(-7. / 16.).into_inner(), 0b1101_0100);
        assert_eq!(Posit8Es1::from(-256.).into_inner(), 0b1000_0100);
    }

    #[test]
    fn posit32_test() {
        assert_eq!(Posit32::from(1.).into_inner(), 0b0100_0000 << 24);
    }

    #[test]
    fn posit256_test() {
        assert_eq!(
            Posit256::from(1.).into_inner(),
            u256::from(0b0100_0000u64) << 248
        );
        assert_eq!(
            Posit256::from(1.125).into_inner(),
            u256::from(0b0100_0000_0010_0000u64) << 240
        );
    }

    #[test]
    fn posit8_es1_round_test() {
        assert_eq!(Posit8Es1::from(0.9999), Posit8Es1::from(1.));
        assert_eq!(Posit8Es1::from(73. / 64.), Posit8Es1::from(18. / 16.));
        assert_eq!(Posit8Es1::from(74. / 64.), Posit8Es1::from(18. / 16.));
        assert_eq!(Posit8Es1::from(75. / 64.), Posit8Es1::from(19. / 16.));
        assert_eq!(
            Posit8Es1::with(true, 1, 1, 0b0111_1111),
            Posit8Es1::with(true, 1, 1, 0b1000_0000),
        );
        assert_eq!(
            Posit8Es1::with(true, 1, 0, 0b1111_1111),
            Posit8Es1::with(true, 1, 1, 0b0000_0000),
        );
        assert_eq!(
            Posit8Es1::with(true, 1, 1, 0b1111_1111),
            Posit8Es1::with(true, 2, 0, 0b0000_0000),
        );
    }

    #[test]
    fn posit256_nar_test() {
        assert_eq!(Posit256::from(::core::f32::INFINITY), Posit256::NAR);
        assert_eq!(Posit256::from(::core::f32::NEG_INFINITY), Posit256::NAR);
        assert_eq!(Posit256::from(::core::f32::NAN), Posit256::NAR);
    }

    #[test]
    fn posit_neg_test() {
        assert_eq!(
            (-Posit256::from(1.)).into_inner(),
            Posit256::from(-1.).into_inner(),
        );
        assert_eq!(
            (-Posit256::from(0.)).into_inner(),
            Posit256::from(0.).into_inner(),
        );
    }

    #[test]
    fn posit_is_nar_test() {
        assert!(Posit256::from(::core::f32::INFINITY).is_nar());
        assert!(Posit256::from(::core::f32::NEG_INFINITY).is_nar());
        assert!(Posit256::from(::core::f32::NAN).is_nar());
        assert!(!(Posit256::ZERO.is_nar()));
        assert!(!(Posit256::from(1.).is_nar()));
    }

    #[test]
    fn posit_is_negative_test() {
        assert!(!(Posit256::from(::core::f32::INFINITY).is_negative()));
        assert!(!(Posit256::from(::core::f32::NEG_INFINITY).is_negative()));
        assert!(!(Posit256::from(::core::f32::NAN).is_negative()));
        assert!(!(Posit256::from(0.)).is_negative());
        assert!(!(Posit256::from(3.)).is_negative());
        assert!(Posit256::from(-2.).is_negative());
    }

    #[test]
    fn posit_is_positive_test() {
        assert!(!(Posit256::from(::core::f32::INFINITY).is_positive()));
        assert!(!(Posit256::from(::core::f32::NEG_INFINITY).is_positive()));
        assert!(!(Posit256::from(::core::f32::NAN).is_positive()));
        assert!(!(Posit256::from(0.)).is_positive());
        assert!(Posit256::from(3.).is_positive());
        assert!(!(Posit256::from(-2.).is_positive()));
    }

    #[test]
    fn posit_is_zero_test() {
        assert!(!(Posit256::from(::core::f32::INFINITY).is_zero()));
        assert!(!(Posit256::from(::core::f32::NEG_INFINITY).is_zero()));
        assert!(!(Posit256::from(::core::f32::NAN).is_zero()));
        assert!(Posit256::from(0.).is_zero());
        assert!(!(Posit256::from(3.).is_zero()));
        assert!(!(Posit256::from(-2.).is_zero()));
    }

    #[test]
    fn posit_decode_test() {
        assert_eq!(Posit8Es1::with(false, 1, 1, 0x80), Posit8Es1::from(12.));
        assert_eq!(Some((false, 1, 1, 0x80)), Posit8Es1::from(12.).decode());
        assert_eq!(Posit16::with(true, 1, 1, 0x8000), Posit16::from(-12.));
        assert_eq!(Some((true, 1, 1, 0x8000)), Posit16::from(-12.).decode());
    }

    #[test]
    fn posit_cmp_test() {
        assert!(Posit16::from(4.) < Posit16::from(5.));
        assert!(Posit16::from(0.) <= (Posit16::from(0.)));
        assert!(!(Posit16::from(0.) > (Posit16::from(0.))));
        assert!(Posit16::from(-1.) < Posit16::from(-0.9));
        assert!(Posit16::from(-2.) < Posit16::from(3.));
        assert!(Posit16::NAR < Posit16::from(0.));
        assert!(Posit16::NAR < Posit16::from(3.));
        assert!(Posit16::NAR < Posit16::from(-0.2));
        assert!(Posit16::NAR <= (Posit16::NAR));
        assert!(!(Posit16::NAR > (Posit16::NAR)));
    }

    #[test]
    fn posit_add_test() {
        assert_eq!(Posit16::from(-1.) + Posit16::from(-2.), Posit16::from(-3.));
        assert_eq!(
            Posit16::from(-1.) + Posit16::from(2.25),
            Posit16::from(1.25)
        );
        assert_eq!(Posit16::from(1.) + Posit16::from(2.), Posit16::from(3.));
        assert_eq!(
            Posit16::from(16.) + Posit16::from(-64.),
            Posit16::from(-48.)
        );
        assert_eq!(
            Posit16::from(2.125) + Posit16::from(3.5),
            Posit16::from(5.625)
        );
        assert_eq!(Posit16::from(1.3) + Posit16::from(3.0), Posit16::from(4.3));
        assert_eq!(
            Posit16::from(1. / 3.) + Posit16::from(1. / 3.),
            Posit16::from(2. / 3.)
        );
        assert_eq!(Posit16::from(4.) + Posit16::from(0.), Posit16::from(4.));
        assert_eq!(Posit16::from(0.) + Posit16::from(3.), Posit16::from(3.));
        assert_eq!(Posit16::from(0.) + Posit16::from(0.), Posit16::from(0.));
        assert_eq!(Posit16::NAR + Posit16::from(3.), Posit16::NAR);
        assert_eq!(Posit16::from(0.) + Posit16::NAR, Posit16::NAR);
    }

    #[test]
    fn posit_sub_test() {
        assert_eq!(Posit16::from(-1.) - Posit16::from(-2.), Posit16::from(1.));
        assert_eq!(
            Posit16::from(-1.) - Posit16::from(2.25),
            Posit16::from(-3.25)
        );
        assert_eq!(
            Posit16::from(6.) - Posit16::from(-4.25),
            Posit16::from(10.25)
        );
        assert_eq!(Posit16::from(1.) - Posit16::from(2.), Posit16::from(-1.));
        assert_eq!(
            Posit16::from(2.125) - Posit16::from(3.5),
            Posit16::from(-1.375)
        );
    }

    #[test]
    fn posit_mul_test() {
        assert_eq!(Posit16::from(2.) * Posit16::from(3.), Posit16::from(6.));
        assert_eq!(
            Posit16::from(1.25) * Posit16::from(3.5),
            Posit16::from(4.375)
        );
        assert_eq!(
            Posit16::from(1. / 3.) * Posit16::from(1. / 3.),
            Posit16::from(1. / 9.)
        );
        assert_eq!(Posit16::from(-0.5) * Posit16::from(8.), Posit16::from(-4.));
        assert_eq!(
            Posit16::from(-16.) * Posit16::from(-16.),
            Posit16::from(256.)
        );
        assert_eq!(Posit16::from(7.) * Posit16::from(0.), Posit16::from(0.));
        assert_eq!(Posit16::NAR * Posit16::from(3.), Posit16::NAR);
        assert_eq!(Posit16::from(0.) * Posit16::NAR, Posit16::NAR);
    }
}
