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

use crate::u256;

macro_rules! construct_posit {
    ($name:ident, $bits:expr, $es:expr, $internal:ident, $zeros: expr, $ones: expr) => {
        #[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
        pub struct $name($internal);

        impl $name {
            #[inline]
            pub fn as_inner(&self) -> &$internal {
                &self.0
            }

            #[inline]
            pub fn into_inner(self) -> $internal {
                self.0
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
                let (exp_mask, fraction) = match $bits - len - 2 < $es {
                    true => (shr(exp, $es - ($bits - len - 2)), shl(exp, $es + len + 2)),
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

        impl ::core::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let &$name(ref data) = self;
                write!(f, "{:?}", data)?;
                Ok(())
            }
        }

        impl From<f32> for $name {
            fn from(init: f32) -> $name {
                let bits = (if init.is_sign_negative() { -init } else { init }).to_bits();
                let init_exp = (bits >> 23) as i16 - 127;
                let regime = init_exp >> $es;
                let exp = (init_exp - (regime << $es)) as u8;
                let mut mantissa = [0u8; $bits / 8];
                let _ = (bits << 9)
                    .to_be_bytes()
                    .iter()
                    .enumerate()
                    .filter(|&(i, _)| i < $bits / 8)
                    .map(|(i, e)| mantissa[i] = *e)
                    .collect::<()>();
                Self::with(
                    init.is_sign_negative(),
                    regime,
                    exp.into(),
                    $internal::from_be_bytes(mantissa),
                )
            }
        }
    };
}

construct_posit!(Posit16, 16, 1, u16, 0, 0xffff);
construct_posit!(Posit32, 32, 2, u32, 0, 0xffff_ffff);
construct_posit!(Posit256, 256, 8, u256, u256::ZERO, u256::MAX);

#[cfg(test)]
mod tests {
    #![allow(unused)]

    use super::*;

    construct_posit!(Posit8Es1, 8, 1, u8, 0, 0xff);

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
            u256::from(0b0100_0000_0000_0100u64) << 240
        );
    }

    #[test]
    fn posit8Es1_round_test() {
        assert_eq!(Posit8Es1::from(0.9999), Posit8Es1::from(1.));
        assert_eq!(Posit8Es1::from(73. / 64.), Posit8Es1::from(18. / 16.));
        assert_eq!(Posit8Es1::from(74. / 64.), Posit8Es1::from(18. / 16.));
        assert_eq!(Posit8Es1::from(75. / 64.), Posit8Es1::from(19. / 16.));
    }
}
