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
use crate::error::PositDecodeError;

macro_rules! construct_posit {
    ($name:ident, $bits:expr, $es:expr, $internal:ident, $zeros: expr, $ones: expr, $nar: expr, $guard:ident, $guard_zero:expr, $guard_max: expr) => {
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

            fn exp(regime: i16, exp: $internal) -> i32 {
                ((regime as i32) << $es) + exp.to_le_bytes()[0] as i32
            }

            pub fn from_bits(bits: $internal) -> Self {
                Self(bits)
            }

            pub fn decode(&self) -> Result<(bool, i16, $internal, $internal), PositDecodeError> {
                if self.is_zero() {
                    return Err(PositDecodeError::Zero);
                }
                if self.is_nar() {
                    return Err(PositDecodeError::NaR);
                }
                let sign = self.is_negative();
                let input = self.abs().0 << 1;
                let (regime, input) = match ((!input).leading_zeros(), input.leading_zeros()) {
                    (0, zeros) => (
                        -(zeros as i16),
                        input.checked_shl(zeros as u32 + 1).unwrap_or($zeros),
                    ),
                    (ones, _) => (
                        (ones - 1) as i16,
                        input.checked_shl(ones as u32 + 1).unwrap_or($zeros),
                    ),
                };
                let exp = input.checked_shr($bits - $es).unwrap_or($zeros);
                let mantissa = input.checked_shl($es).unwrap_or($zeros);
                Ok((sign, regime, exp, mantissa))
            }

            pub fn encode(sign: bool, regime: i16, exp: $internal, mantissa: $internal) -> Self {
                Self::_encode(
                    sign,
                    regime,
                    $guard::from(exp),
                    $guard::from(mantissa) << $bits,
                )
            }

            fn _encode(sign: bool, regime: i16, exp: $guard, mantissa: $guard) -> Self {
                let shl = |x: $guard, shift| x.checked_shl(shift).unwrap_or($guard_zero);
                let shr = |x: $guard, shift| x.checked_shr(shift).unwrap_or($guard_zero);
                let mut res = $guard_zero;
                let len: u32 = (regime.abs() as u16 + (!regime.is_negative() as u16)).into();
                let regime_mask = match regime.is_negative() {
                    true => shr(!($guard_max >> 1), len + 1),
                    false => ($guard_max ^ shr($guard_max, len)) >> 1,
                };
                let mantissa_mask = shr(mantissa, len + $es + 2);
                let exp_mask = shl(exp, $bits * 2 - len - 2 - $es);
                res |= regime_mask | exp_mask | mantissa_mask;
                let (mut high, low) = {
                    let mut h = [0u8; $bits / 8];
                    let mut l = [0u8; $bits / 8];
                    let bytes = res.to_le_bytes();
                    for i in 0..($bits / 8) {
                        h[i] = bytes[$bits / 8 + i]
                    }
                    for i in 0..($bits / 8) {
                        l[i] = bytes[i]
                    }
                    ($internal::from_le_bytes(h), $internal::from_le_bytes(l))
                };
                match (high == ($ones >> 1), low.cmp(&$nar)) {
                    (true, _) | (_, ::core::cmp::Ordering::Less) => (),
                    (_, ::core::cmp::Ordering::Greater) => high += !($ones << 1),
                    (_, ::core::cmp::Ordering::Equal) => high += (high & !($ones << 1)),
                };
                Self(if sign { high.wrapping_neg() } else { high })
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
                let (lhs, rhs) = match (lhs.decode(), rhs.decode()) {
                    (Err(PositDecodeError::NaR), _) | (_, Err(PositDecodeError::NaR)) => {
                        return Self::NAR
                    }
                    (Err(PositDecodeError::Zero), _) => return (if sign { -rhs } else { rhs }),
                    (_, Err(PositDecodeError::Zero)) => return (if sign { -lhs } else { lhs }),
                    (Ok(l), Ok(r)) => (l, r),
                };
                let is_add = self.is_negative() == other.is_negative();
                if !is_add && lhs == rhs {
                    return Self::ZERO;
                }
                let exp_lhs = Self::exp(lhs.1, lhs.2);
                let exp_rhs = Self::exp(rhs.1, rhs.2);
                let shift = (exp_lhs - exp_rhs) as u32;
                let mantissa_lhs = ($guard::from(lhs.3) << ($bits - 2)) | (!($guard_max >> 1) >> 1);
                let mantissa_rhs = (($guard::from(rhs.3) << ($bits - 2))
                    | (!($guard_max >> 1) >> 1))
                    .checked_shr(shift)
                    .unwrap_or($guard_zero);
                let mantissa = match self.is_negative() == other.is_negative() {
                    true => mantissa_lhs + mantissa_rhs,
                    false => mantissa_lhs - mantissa_rhs,
                };
                let leading_zeros = mantissa.leading_zeros();
                let scaling_factor = 1 - leading_zeros as i32;
                let mantissa = mantissa
                    .checked_shl(leading_zeros as u32 + 1)
                    .unwrap_or($guard_zero);
                let (regime, exp) = Self::regime(exp_lhs + scaling_factor);
                Self::_encode(sign, regime, exp.into(), mantissa)
            }
        }

        impl<T> ::core::ops::Sub<T> for $name
        where
            T: Into<$name>,
        {
            type Output = $name;
            fn sub(self, other: T) -> $name {
                self + (-(other.into()))
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
                let (lhs, rhs) = match (self.decode(), other.decode()) {
                    (Err(PositDecodeError::NaR), _) | (_, Err(PositDecodeError::NaR)) => {
                        return Self::NAR
                    }
                    (Err(PositDecodeError::Zero), _) | (_, Err(PositDecodeError::Zero)) => {
                        return Self::ZERO
                    }
                    (Ok(l), Ok(r)) => (l, r),
                };
                let exp_lhs = Self::exp(lhs.1, lhs.2);
                let exp_rhs = Self::exp(rhs.1, rhs.2);
                let mantissa_lhs = $guard::from((lhs.3 >> 2) | (Self::NAR.0 >> 1));
                let mantissa_rhs = $guard::from((rhs.3 >> 2) | (Self::NAR.0 >> 1));
                let mut mantissa = mantissa_lhs * mantissa_rhs;
                let shift = mantissa.leading_zeros();
                let scaling_factor = 3 - shift as i32;
                mantissa <<= shift as usize;
                mantissa <<= 1;
                let (regime, exp) = Self::regime(exp_lhs + exp_rhs + scaling_factor);
                Self::_encode(sign, regime, exp.into(), mantissa)
            }
        }

        impl<T> ::core::ops::Div<T> for $name
        where
            T: Into<$name>,
        {
            type Output = $name;
            fn div(self, other: T) -> $name {
                let other = other.into();
                let sign = self.is_negative() != other.is_negative();
                let (lhs, rhs) = match (self.decode(), other.decode()) {
                    (Err(PositDecodeError::NaR), _) | (_, Err(PositDecodeError::NaR)) => {
                        return Self::NAR
                    }
                    (_, Err(PositDecodeError::Zero)) => return Self::NAR,
                    (Err(PositDecodeError::Zero), _) => return Self::ZERO,
                    (Ok(l), Ok(r)) => (l, r),
                };
                let exp_lhs = Self::exp(lhs.1, lhs.2);
                let exp_rhs = Self::exp(rhs.1, rhs.2);
                let mut mantissa_lhs = $guard::from((lhs.3 >> 1) | Self::NAR.0);
                let mut mantissa_rhs = $guard::from((rhs.3 >> 1) | Self::NAR.0);
                let cut_lhs = mantissa_lhs.leading_zeros();
                let cut_rhs = mantissa_rhs.trailing_zeros();
                mantissa_lhs <<= cut_lhs as usize;
                mantissa_rhs >>= cut_rhs as usize;
                let mantissa = mantissa_lhs / mantissa_rhs;
                let rem = mantissa_lhs % mantissa_rhs;
                let cut_rem = rem.leading_zeros();
                let rem_mantissa = match rem != $guard_zero {
                    true => (rem << cut_rem as usize) / mantissa_rhs,
                    false => rem,
                };
                let shift = mantissa.leading_zeros();
                let scaling_factor =
                    $bits as i32 * 2 - 1 - shift as i32 - (cut_lhs + cut_rhs) as i32;
                let mantissa = mantissa
                    .checked_shl(shift as u32 + 1)
                    .unwrap_or($guard_zero);
                let rem_mantissa = rem_mantissa
                    .checked_shr(cut_rem - shift - 1)
                    .unwrap_or($guard_zero);
                let (regime, exp) = Self::regime(exp_lhs - exp_rhs + scaling_factor);
                Self::_encode(sign, regime, exp.into(), mantissa | rem_mantissa)
            }
        }

        impl ::core::ops::AddAssign for $name {
            #[inline]
            fn add_assign(&mut self, other: Self) {
                *self = *self + other
            }
        }

        impl ::core::ops::SubAssign for $name {
            #[inline]
            fn sub_assign(&mut self, other: Self) {
                *self = *self - other
            }
        }

        impl ::core::ops::MulAssign for $name {
            #[inline]
            fn mul_assign(&mut self, other: Self) {
                *self = *self * other
            }
        }

        impl ::core::ops::DivAssign for $name {
            #[inline]
            fn div_assign(&mut self, other: Self) {
                *self = *self / other
            }
        }

        impl ::core::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let &$name(ref data) = self;
                write!(f, "{:?}", data)?;
                Ok(())
            }
        }

        impl From<$name> for f32 {
            fn from(init: $name) -> f32 {
                let (sign, regime, exp, mantissa): (bool, i16, $internal, $internal) =
                    match init.decode() {
                        Err(PositDecodeError::Zero) => return 0.,
                        Err(PositDecodeError::NaR) => return ::core::f32::NAN,
                        Ok(v) => v,
                    };
                let sign = if sign { 0x80000000u32 } else { 0u32 };
                let exp = $name::exp(regime, exp);
                let (exp, mantissa) = match (exp > 127, exp < -149, exp < -126) {
                    (true, _, _) => return ::core::f32::MAX,
                    (_, true, _) => return ::core::f32::MIN,
                    (_, _, true) => (0u32, ((mantissa >> 1) | $nar) >> (-127 - exp) as usize),
                    _ => (((exp + 127) as u32) << 23, mantissa),
                };
                let mut m = [0u8; 4];
                let _ = mantissa
                    .to_be_bytes()
                    .iter()
                    .enumerate()
                    .filter(|&(i, _)| i < 4)
                    .map(|(i, e)| m[i] = *e)
                    .collect::<()>();
                f32::from_bits(sign | exp | u32::from_be_bytes(m) >> 9)
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
                let mut mantissa = [0u8; $bits * 2 / 8];
                let _ = (bits << 9)
                    .to_be_bytes()
                    .iter()
                    .enumerate()
                    .filter(|&(i, _)| i < $bits * 2 / 8)
                    .map(|(i, e)| mantissa[i] = *e)
                    .collect::<()>();
                let (mantissa, init_exp) = match init.is_normal() {
                    true => ($guard::from_be_bytes(mantissa), (bits >> 23) as i16 - 127),
                    false => {
                        let m = $guard::from_be_bytes(mantissa);
                        let shift = m.leading_zeros() + 1;
                        (m << shift as usize, -126 - shift as i16)
                    }
                };
                let (regime, exp) = Self::regime(init_exp);
                Self::_encode(init.is_sign_negative(), regime, exp.into(), mantissa)
            }
        }

        impl From<$name> for f64 {
            fn from(init: $name) -> f64 {
                let (sign, regime, exp, mantissa): (bool, i16, $internal, $internal) =
                    match init.decode() {
                        Err(PositDecodeError::Zero) => return 0.,
                        Err(PositDecodeError::NaR) => return ::core::f64::NAN,
                        Ok(v) => v,
                    };
                let sign = if sign { 0x80000000_00000000u64 } else { 0u64 };
                let exp = $name::exp(regime, exp);
                let (exp, mantissa) = match (exp > 1023, exp < -1074, exp < -1022) {
                    (true, _, _) => return ::core::f64::MAX,
                    (_, true, _) => return ::core::f64::MIN,
                    (_, _, true) => (0u64, ((mantissa >> 1) | $nar) >> (-1023 - exp) as usize),
                    _ => (((exp + 1023) as u64) << 52, mantissa),
                };
                let mut m = [0u8; 8];
                let _ = mantissa
                    .to_be_bytes()
                    .iter()
                    .enumerate()
                    .filter(|&(i, _)| i < 8)
                    .map(|(i, e)| m[i] = *e)
                    .collect::<()>();
                f64::from_bits(sign | exp | u64::from_be_bytes(m) >> 12)
            }
        }

        impl From<f64> for $name {
            fn from(init: f64) -> $name {
                if init == 0. {
                    return Self::ZERO;
                }
                if init.is_infinite() || init.is_nan() {
                    return Self::NAR;
                }
                let bits = (if init.is_sign_negative() { -init } else { init }).to_bits();
                let mut mantissa = [0u8; $bits * 2 / 8];
                let _ = (bits << 12)
                    .to_be_bytes()
                    .iter()
                    .enumerate()
                    .filter(|&(i, _)| i < $bits * 2 / 8)
                    .map(|(i, e)| mantissa[i] = *e)
                    .collect::<()>();
                let (mantissa, init_exp) = match init.is_normal() {
                    true => ($guard::from_be_bytes(mantissa), (bits >> 52) as i16 - 1023),
                    false => {
                        let m = $guard::from_be_bytes(mantissa);
                        let shift = m.leading_zeros() + 1;
                        (m << shift as usize, -1022 - shift as i16)
                    }
                };
                let (regime, exp) = Self::regime(init_exp);
                Self::_encode(init.is_sign_negative(), regime, exp.into(), mantissa)
            }
        }
    };
}

construct_posit!(
    Posit8,
    8,
    0,
    u8,
    0,
    ::core::u8::MAX,
    0x80,
    u16,
    0,
    ::core::u16::MAX
);
construct_posit!(
    Posit16,
    16,
    1,
    u16,
    0,
    ::core::u16::MAX,
    0x8000,
    u32,
    0,
    ::core::u32::MAX
);
construct_posit!(
    Posit32,
    32,
    2,
    u32,
    0,
    ::core::u32::MAX,
    0x8000_0000,
    u64,
    0,
    ::core::u64::MAX
);
construct_posit!(
    Posit64,
    64,
    3,
    u64,
    0,
    ::core::u64::MAX,
    0x8000_0000_0000_0000,
    u128,
    0,
    ::core::u128::MAX
);
construct_posit!(
    Posi128,
    128,
    4,
    u128,
    0,
    ::core::u128::MAX,
    0x8000_0000_0000_0000_0000_0000_0000_0000,
    u256,
    u256::ZERO,
    u256::MAX
);
construct_posit!(
    Posit256,
    256,
    5,
    u256,
    u256::ZERO,
    u256::MAX,
    u256::from_inner([0, 0, 0, 0x8000_0000_0000_0000]),
    u512,
    u512::ZERO,
    u512::MAX
);
construct_posit!(
    Posit512,
    512,
    6,
    u512,
    u512::ZERO,
    u512::MAX,
    u512::from_inner([0, 0, 0, 0, 0, 0, 0, 0x8000_0000_0000_0000]),
    u1024,
    u1024::ZERO,
    u1024::MAX
);

#[cfg(test)]
mod tests {
    #![allow(unused)]

    use super::*;

    construct_posit!(Posit8Es1, 8, 1, u8, 0, 0xff, 0x80, u16, 0, 0xffff);

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
        assert_eq!(f32::from(Posit64::from(sub)), sub);
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
            Posit8Es1::encode(true, 1, 1, 0b0111_1111u8),
            Posit8Es1::encode(true, 1, 1, 0b1000_0000u8),
        );
        assert_eq!(
            Posit8Es1::encode(true, 1, 0, 0b1111_1111u8),
            Posit8Es1::encode(true, 1, 1, 0b0000_0000u8),
        );
        assert_eq!(
            Posit8Es1::encode(true, 1, 1, 0b1111_1111u8),
            Posit8Es1::encode(true, 2, 0, 0b0000_0000u8),
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
        assert_eq!(Posit8Es1::encode(false, 1, 1, 0x80u8), Posit8Es1::from(12.));
        assert_eq!(Ok((false, 1, 1, 0x80)), Posit8Es1::from(12.).decode());
        assert_eq!(Posit16::encode(true, 1, 1, 0x8000u16), Posit16::from(-12.));
        assert_eq!(Ok((true, 1, 1, 0x8000)), Posit16::from(-12.).decode());
        assert_eq!(
            Ok((false, 0, 0, 0b00001000)),
            Posit8::from(1.03125).decode()
        );
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
        assert_eq!(Posit8::from(10.) + Posit8::from(1.0935), Posit8::from(12.));
        assert_eq!(Posit8::from(10.) + Posit8::from(-10.), Posit8::from(0.));
        assert_eq!(
            Posit16::from(
                f64::from(Posit16::from_bits(32769)) + f64::from(Posit16::from_bits(1457))
            ),
            Posit16::from_bits(32769)
        );
    }

    #[test]
    fn posit_add_assign_test() {
        let mut x = Posit8::from(1.0);
        x += Posit8::from(2.0);
        assert_eq!(x, Posit8::from(3.0))
    }

    #[test]
    fn posit_sub_assign_test() {
        let mut x = Posit8::from(1.0);
        x -= Posit8::from(2.0);
        assert_eq!(x, Posit8::from(-1.0))
    }

    #[test]
    fn posit_mul_assign_test() {
        let mut x = Posit8::from(2.0);
        x *= Posit8::from(3.0);
        assert_eq!(x, Posit8::from(6.0))
    }

    #[test]
    fn posit_div_assign_test() {
        let mut x = Posit8::from(1.0);
        x /= Posit8::from(2.0);
        assert_eq!(x, Posit8::from(0.5))
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
        assert_eq!(Posit16::from(-0.5) * Posit16::from(8.), Posit16::from(-4.));
        assert_eq!(
            Posit16::from(-16.) * Posit16::from(-16.),
            Posit16::from(256.)
        );
        assert_eq!(Posit16::from(7.) * Posit16::from(0.), Posit16::from(0.));
        assert_eq!(Posit16::NAR * Posit16::from(3.), Posit16::NAR);
        assert_eq!(Posit16::from(0.) * Posit16::NAR, Posit16::NAR);
    }

    #[test]
    fn posit_div_test() {
        assert_eq!(Posit16::from(1.) / Posit16::from(2.), Posit16::from(0.5));
        assert_eq!(Posit16::from(1.) / Posit16::from(8.), Posit16::from(0.125));
        assert_eq!(Posit32::from(1.) / Posit32::from(8.), Posit32::from(0.125));
        assert_eq!(
            Posit32::from(1.) / Posit32::from(64.),
            Posit32::from(1. / 64.)
        );
        assert_eq!(Posit8::from(1.75) / Posit8::from(1.), Posit8::from(1.75));
        assert_eq!(
            Posit8::from(-3.) / Posit8::from(1.15625),
            Posit8::from(-2.625)
        );
    }

    fn rand_posit8(fun: fn(Posit8, Posit8, f32, f32) -> (Posit8, f32)) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        for _ in 0..100000 {
            let x: u8 = rng.gen();
            let y: u8 = rng.gen();
            let (p, f) = fun(
                Posit8::from_bits(x),
                Posit8::from_bits(y),
                f32::from(Posit8::from_bits(x)),
                f32::from(Posit8::from_bits(y)),
            );
            assert_eq!(p, Posit8::from(f));
        }
    }

    fn rand_posit16(fun: fn(Posit16, Posit16, f64, f64) -> (Posit16, f64)) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        for _ in 0..100000 {
            let x: u16 = rng.gen();
            let y: u16 = rng.gen();
            let (p, f) = fun(
                Posit16::from_bits(x),
                Posit16::from_bits(y),
                f64::from(Posit16::from_bits(x)),
                f64::from(Posit16::from_bits(y)),
            );
            assert_eq!(p, Posit16::from(f));
        }
    }

    #[test]
    fn posit8_add() {
        rand_posit8(|p_a, p_b, f_a, f_b| (p_a + p_b, f_a + f_b));
    }

    #[test]
    fn posit8_sub() {
        rand_posit8(|p_a, p_b, f_a, f_b| (p_a - p_b, f_a - f_b));
    }

    #[test]
    fn posit8_mul() {
        rand_posit8(|p_a, p_b, f_a, f_b| (p_a * p_b, f_a * f_b));
    }

    #[test]
    fn posit8_div() {
        rand_posit8(|p_a, p_b, f_a, f_b| (p_a / p_b, f_a / f_b));
    }

    #[test]
    fn posit16_add() {
        rand_posit16(|p_a, p_b, f_a, f_b| (p_a + p_b, f_a + f_b));
    }

    #[test]
    fn posit16_sub() {
        rand_posit16(|p_a, p_b, f_a, f_b| (p_a - p_b, f_a - f_b));
    }

    #[test]
    fn posit16_mul() {
        rand_posit16(|p_a, p_b, f_a, f_b| (p_a * p_b, f_a * f_b));
    }

    #[test]
    fn posit16_div() {
        rand_posit16(|p_a, p_b, f_a, f_b| (p_a / p_b, f_a / f_b));
    }
}
