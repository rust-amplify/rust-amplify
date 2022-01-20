// Rust language amplification library providing multiple generic trait
// implementations, type wrappers, derive macros and other language enhancements
//
// Written in 2014 by
//     Andrew Poelstra <apoelstra@wpsoftware.net>
// Refactored & fixed in 2021 by
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

use crate::error::ParseLengthError;
use crate::divrem::DivRem;

macro_rules! construct_bigint {
    ($name:ident, $n_words:expr) => {
        /// Large integer type
        ///
        /// The type is composed of little-endian ordered 64-bit words, which represents
        /// its inner representation.
        #[allow(non_camel_case_types)]
        #[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
        pub struct $name([u64; $n_words]);

        impl $name {
            #[inline]
            /// Converts the object to a raw pointer
            pub fn as_ptr(&self) -> *const u64 {
                let &$name(ref dat) = self;
                dat.as_ptr()
            }

            #[inline]
            /// Converts the object to a mutable raw pointer
            pub fn as_mut_ptr(&mut self) -> *mut u64 {
                let &mut $name(ref mut dat) = self;
                dat.as_mut_ptr()
            }

            #[inline]
            /// Returns the underlying array of words constituting large integer
            pub fn as_inner(&self) -> &[u64; $n_words] {
                &self.0
            }

            #[inline]
            /// Returns the underlying array of words constituting large integer
            pub fn into_inner(self) -> [u64; $n_words] {
                self.0
            }

            #[inline]
            /// Constructs integer type from the underlying array of words.
            pub fn from_inner(array: [u64; $n_words]) -> Self {
                Self(array)
            }
        }

        impl $name {
            /// Zero value
            pub const ZERO: $name = $name([0u64; $n_words]);

            /// Value for `1`
            pub const ONE: $name = $name({
                let mut one = [0u64; $n_words];
                one[0] = 1u64;
                one
            });

            /// Bit dimension
            pub const BITS: u32 = $n_words * 64;

            /// Length of the integer in bytes
            pub const BYTES: u8 = $n_words * 8;

            /// Length of the inner representation in 64-bit words
            pub const INNER_LEN: u8 = $n_words;

            /// Returns whether specific bit number is set to `1` or not
            #[inline]
            pub fn bit(&self, index: usize) -> bool {
                let &$name(ref arr) = self;
                arr[index / 64] & (1 << (index % 64)) != 0
            }

            /// Returns lower 32 bits of the number as `u32`
            #[inline]
            pub fn low_u32(&self) -> u32 {
                let &$name(ref arr) = self;
                (arr[0] & ::core::u32::MAX as u64) as u32
            }

            /// Returns lower 64 bits of the number as `u32`
            #[inline]
            pub fn low_u64(&self) -> u64 {
                let &$name(ref arr) = self;
                arr[0] as u64
            }

            /// Return the least number of bits needed to represent the number
            #[inline]
            pub fn bits_required(&self) -> usize {
                let &$name(ref arr) = self;
                if self.is_negative() {
                    for i in 1..$n_words {
                        if arr[$n_words - i] != ::core::u64::MAX {
                            return (0x40 * ($n_words - i + 1)) + 1
                                - (!arr[$n_words - i]).leading_zeros() as usize;
                        }
                    }
                    0x40 + 1 - (!arr[0]).leading_zeros() as usize
                } else {
                    for i in 1..$n_words {
                        if arr[$n_words - i] > 0 {
                            return (0x40 * ($n_words - i + 1))
                                - arr[$n_words - i].leading_zeros() as usize;
                        }
                    }
                    0x40 - arr[0].leading_zeros() as usize
                }
            }

            #[inline]
            pub fn is_zero(&self) -> bool {
                self[..] == [0; $n_words]
            }

            #[inline]
            pub fn abs(self) -> $name {
                if !self.is_negative() {
                    return self;
                }
                (!self).wrapping_add($name::ONE)
            }

            /// Creates the integer value from a byte array using big-endian
            /// encoding
            pub fn from_be_bytes(bytes: [u8; $n_words * 8]) -> $name {
                Self::_from_be_slice(&bytes)
            }

            /// Creates the integer value from a byte slice using big-endian
            /// encoding
            pub fn from_be_slice(bytes: &[u8]) -> Result<$name, ParseLengthError> {
                if bytes.len() != $n_words * 8 {
                    Err(ParseLengthError {
                        actual: bytes.len(),
                        expected: $n_words * 8,
                    })
                } else {
                    Ok(Self::_from_be_slice(bytes))
                }
            }

            /// Creates the integer value from a byte array using little-endian
            /// encoding
            pub fn from_le_bytes(bytes: [u8; $n_words * 8]) -> $name {
                Self::_from_le_slice(&bytes)
            }

            /// Creates the integer value from a byte slice using little-endian
            /// encoding
            pub fn from_le_slice(bytes: &[u8]) -> Result<$name, ParseLengthError> {
                if bytes.len() != $n_words * 8 {
                    Err(ParseLengthError {
                        actual: bytes.len(),
                        expected: $n_words * 8,
                    })
                } else {
                    Ok(Self::_from_le_slice(bytes))
                }
            }

            fn _from_be_slice(bytes: &[u8]) -> $name {
                let mut slice = [0u64; $n_words];
                slice
                    .iter_mut()
                    .rev()
                    .zip(bytes.chunks(8).into_iter().map(|s| {
                        let mut b = [0u8; 8];
                        b.copy_from_slice(s);
                        b
                    }))
                    .for_each(|(word, bytes)| *word = u64::from_be_bytes(bytes));
                $name(slice)
            }

            fn _from_le_slice(bytes: &[u8]) -> $name {
                let mut slice = [0u64; $n_words];
                slice
                    .iter_mut()
                    .zip(bytes.chunks(8).into_iter().map(|s| {
                        let mut b = [0u8; 8];
                        b.copy_from_slice(s);
                        b
                    }))
                    .for_each(|(word, bytes)| *word = u64::from_le_bytes(bytes));
                $name(slice)
            }

            /// Convert the integer into a byte array using big-endian encoding
            pub fn to_be_bytes(self) -> [u8; $n_words * 8] {
                let mut res = [0; $n_words * 8];
                for i in 0..$n_words {
                    let start = i * 8;
                    res[start..start + 8]
                        .copy_from_slice(&self.0[$n_words - (i + 1)].to_be_bytes());
                }
                res
            }

            /// Convert a integer into a byte array using little-endian encoding
            pub fn to_le_bytes(self) -> [u8; $n_words * 8] {
                let mut res = [0; $n_words * 8];
                for i in 0..$n_words {
                    let start = i * 8;
                    res[start..start + 8].copy_from_slice(&self.0[i].to_le_bytes());
                }
                res
            }
        }

        impl DivRem for $name {
            // divmod like operation, returns (quotient, remainder)
            #[inline]
            fn div_rem(self, other: Self) -> (Self, Self) {
                let mut sub_copy = self;
                let mut shift_copy = other;
                let mut ret = [0u64; $n_words];

                let my_bits = self.bits_required();
                let your_bits = other.bits_required();

                // Check for division by 0
                assert!(your_bits != 0);

                // Early return in case we are dividing by a larger number than us
                if my_bits < your_bits {
                    return ($name(ret), sub_copy);
                }

                if sub_copy.is_negative() || shift_copy.is_negative() {
                    assert!(
                        sub_copy != $name::MIN || my_bits != $name::BITS as usize,
                        "attempt to divide with overflow"
                    );
                }
                sub_copy = sub_copy.abs();
                shift_copy = shift_copy.abs();

                // Bitwise long division
                let mut shift = my_bits - your_bits;
                shift_copy <<= shift;
                loop {
                    if sub_copy >= shift_copy {
                        ret[shift / 64] |= 1 << (shift % 64);
                        sub_copy -= shift_copy;
                    }
                    shift_copy >>= 1;
                    if shift == 0 {
                        break;
                    }
                    shift -= 1;
                }

                let ret = match self.is_negative() == other.is_negative() {
                    true => $name(ret),
                    false => -$name(ret),
                };
                sub_copy = match self.is_negative() {
                    true => -sub_copy,
                    false => sub_copy,
                };
                (ret, sub_copy)
            }
            // same operation as in div_rem, not panicking when
            #[inline]
            fn div_rem_checked(self, other: Self) -> Option<(Self, Self)> {
                //quotient and remainder will always be smaller than self so they're going to be in bounds
                match other {
                    Self::ZERO => None,
                    _ => Some(self.div_rem(other)),
                }
            }
        }

        impl From<u8> for $name {
            fn from(init: u8) -> $name {
                let mut ret = [0; $n_words];
                ret[0] = init as u64;
                $name(ret)
            }
        }

        impl From<u16> for $name {
            fn from(init: u16) -> $name {
                let mut ret = [0; $n_words];
                ret[0] = init as u64;
                $name(ret)
            }
        }

        impl From<u32> for $name {
            fn from(init: u32) -> $name {
                let mut ret = [0; $n_words];
                ret[0] = init as u64;
                $name(ret)
            }
        }

        impl From<u64> for $name {
            fn from(init: u64) -> $name {
                let mut ret = [0; $n_words];
                ret[0] = init;
                $name(ret)
            }
        }

        impl From<u128> for $name {
            fn from(init: u128) -> $name {
                let mut ret = [0; $n_words * 8];
                for (pos, byte) in init.to_le_bytes().iter().enumerate() {
                    ret[pos] = *byte;
                }
                $name::from_le_bytes(ret)
            }
        }

        impl<'a> ::core::convert::TryFrom<&'a [u64]> for $name {
            type Error = $crate::error::ParseLengthError;
            fn try_from(data: &'a [u64]) -> Result<$name, Self::Error> {
                if data.len() != $n_words {
                    Err($crate::error::ParseLengthError {
                        actual: data.len(),
                        expected: $n_words,
                    })
                } else {
                    let mut bytes = [0u64; $n_words];
                    bytes.copy_from_slice(data);
                    Ok(Self::from_inner(bytes))
                }
            }
        }
        impl ::core::ops::Index<usize> for $name {
            type Output = u64;

            #[inline]
            fn index(&self, index: usize) -> &u64 {
                &self.0[index]
            }
        }

        impl ::core::ops::Index<::core::ops::Range<usize>> for $name {
            type Output = [u64];

            #[inline]
            fn index(&self, index: ::core::ops::Range<usize>) -> &[u64] {
                &self.0[index]
            }
        }

        impl ::core::ops::Index<::core::ops::RangeTo<usize>> for $name {
            type Output = [u64];

            #[inline]
            fn index(&self, index: ::core::ops::RangeTo<usize>) -> &[u64] {
                &self.0[index]
            }
        }

        impl ::core::ops::Index<::core::ops::RangeFrom<usize>> for $name {
            type Output = [u64];

            #[inline]
            fn index(&self, index: ::core::ops::RangeFrom<usize>) -> &[u64] {
                &self.0[index]
            }
        }

        impl ::core::ops::Index<::core::ops::RangeFull> for $name {
            type Output = [u64];

            #[inline]
            fn index(&self, _: ::core::ops::RangeFull) -> &[u64] {
                &self.0[..]
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
                match (self.is_negative(), other.is_negative()) {
                    (false, true) => ::core::cmp::Ordering::Greater,
                    (true, false) => ::core::cmp::Ordering::Less,
                    _ => {
                        // We need to manually implement ordering because we use little-endian
                        // and the auto derive is a lexicographic ordering(i.e. memcmp)
                        // which with numbers is equivilant to big-endian
                        for i in 0..$n_words {
                            if self[$n_words - 1 - i] < other[$n_words - 1 - i] {
                                return ::core::cmp::Ordering::Less;
                            }
                            if self[$n_words - 1 - i] > other[$n_words - 1 - i] {
                                return ::core::cmp::Ordering::Greater;
                            }
                        }
                        ::core::cmp::Ordering::Equal
                    }
                }
            }
        }

        impl ::core::ops::Neg for $name {
            type Output = Self;
            fn neg(self) -> Self::Output {
                assert!(
                    $name::MIN != $name([::core::u64::MAX; $n_words]),
                    "attempt to negate unsigned number"
                );
                assert!(
                    self != $name::MIN,
                    "attempt to negate the minimum value, which would overflow"
                );
                (!self).wrapping_add($name::ONE)
            }
        }

        impl $name {
            /// Checked integer addition. Computes `self + rhs`, returning `None` if
            /// overflow occurred.
            pub fn checked_add<T>(self, other: T) -> Option<$name>
            where
                T: Into<$name>,
            {
                let (res, flag) = self.overflowing_add(other);
                if flag {
                    None
                } else {
                    Some(res)
                }
            }

            /// Saturating integer addition. Computes `self + rhs`, saturating at the
            /// numeric bounds instead of overflowing.
            pub fn saturating_add<T>(self, other: T) -> $name
            where
                T: Into<$name>,
            {
                let (res, flag) = self.overflowing_add(other);
                if flag {
                    Self::MAX
                } else {
                    res
                }
            }

            /// Calculates `self + rhs`
            ///
            /// Returns a tuple of the addition along with a boolean indicating whether
            /// an arithmetic overflow would occur. If an overflow would have occurred
            /// then the wrapped value is returned.
            pub fn overflowing_add<T>(self, other: T) -> ($name, bool)
            where
                T: Into<$name>,
            {
                let $name(ref me) = self;
                let other = other.into();
                let $name(ref you) = other;
                let mut ret = [0u64; $n_words];
                let mut carry = 0u64;
                for i in 0..$n_words {
                    let (res, flag) = me[i].overflowing_add(carry);
                    carry = flag as u64;
                    let (res, flag) = res.overflowing_add(you[i]);
                    carry += flag as u64;
                    ret[i] = res;
                }
                let ret = Self(ret);
                (
                    ret,
                    (self.is_negative() == other.is_negative())
                        && (self.is_negative() != ret.is_negative()),
                )
            }

            /// Wrapping (modular) addition. Computes `self + rhs`, wrapping around at
            /// the boundary of the type.
            pub fn wrapping_add<T>(self, other: T) -> $name
            where
                T: Into<$name>,
            {
                self.overflowing_add(other).0
            }

            /// Checked integer subtraction. Computes `self - rhs`, returning `None` if
            /// overflow occurred.
            pub fn checked_sub<T>(self, other: T) -> Option<$name>
            where
                T: Into<$name>,
            {
                let (res, flag) = self.overflowing_sub(other);
                if flag {
                    None
                } else {
                    Some(res)
                }
            }

            /// Saturating integer subtraction. Computes `self - rhs`, saturating at the
            /// numeric bounds instead of overflowing.
            pub fn saturating_sub<T>(self, other: T) -> $name
            where
                T: Into<$name>,
            {
                let (res, flag) = self.overflowing_sub(other);
                if flag {
                    Self::MAX
                } else {
                    res
                }
            }

            /// Calculates `self - rhs`
            ///
            /// Returns a tuple of the subtraction along with a boolean indicating
            /// whether an arithmetic overflow would occur. If an overflow would
            /// have occurred then the wrapped value is returned.
            pub fn overflowing_sub<T>(self, other: T) -> ($name, bool)
            where
                T: Into<$name>,
            {
                self.overflowing_add((!other.into()).wrapping_add($name::ONE))
            }

            /// Wrapping (modular) subtraction. Computes `self - rhs`, wrapping around
            /// at the boundary of the type.
            pub fn wrapping_sub<T>(self, other: T) -> $name
            where
                T: Into<$name>,
            {
                self.overflowing_sub(other).0
            }

            /// Checked integer multiplication. Computes `self * rhs`, returning `None`
            /// if overflow occurred.
            pub fn checked_mul<T>(self, other: T) -> Option<$name>
            where
                T: Into<$name>,
            {
                let (res, flag) = self.overflowing_mul(other);
                if flag {
                    None
                } else {
                    Some(res)
                }
            }

            /// Saturating integer multiplication. Computes `self * rhs`, saturating at
            /// the numeric bounds instead of overflowing.
            pub fn saturating_mul<T>(self, other: T) -> $name
            where
                T: Into<$name>,
            {
                let (res, flag) = self.overflowing_mul(other);
                if flag {
                    Self::MAX
                } else {
                    res
                }
            }

            /// Wrapping (modular) multiplication. Computes `self * rhs`, wrapping
            /// around at the boundary of the type.
            pub fn wrapping_mul<T>(self, other: T) -> $name
            where
                T: Into<$name>,
            {
                self.overflowing_mul(other).0
            }

            /// Checked shift left. Computes self << rhs,
            /// returning None if rhs is larger than or equal to the number of bits in self.
            pub fn checked_shl(self, rhs: u32) -> Option<$name> {
                match rhs < Self::BITS {
                    true => Some(self << (rhs as usize)),
                    false => None,
                }
            }

            /// Checked shift right. Computes self >> rhs,
            /// returning None if rhs is larger than or equal to the number of bits in self.
            pub fn checked_shr(self, rhs: u32) -> Option<$name> {
                match rhs < Self::BITS {
                    true => Some(self >> (rhs as usize)),
                    false => None,
                }
            }

            /// Wrapping (modular) negation. Computes -self,
            /// wrapping around at the boundary of the type.
            /// Since unsigned types do not have negative equivalents
            /// all applications of this function will wrap (except for -0).
            /// For values smaller than the corresponding signed type's maximum
            /// the result is the same as casting the corresponding signed value.
            /// Any larger values are equivalent to MAX + 1 - (val - MAX - 1)
            /// where MAX is the corresponding signed type's maximum.
            pub fn wrapping_neg(self) -> $name {
                !self + Self::ONE
            }
        }

        impl<T> ::core::ops::Add<T> for $name
        where
            T: Into<$name>,
        {
            type Output = $name;

            fn add(self, other: T) -> $name {
                let (res, flag) = self.overflowing_add(other);
                assert!(!flag, "attempt to add with overflow");
                res
            }
        }
        impl<T> ::core::ops::AddAssign<T> for $name
        where
            T: Into<$name>,
        {
            #[inline]
            fn add_assign(&mut self, rhs: T) {
                self.0 = (*self + rhs).0
            }
        }

        impl<T> ::core::ops::Sub<T> for $name
        where
            T: Into<$name>,
        {
            type Output = $name;

            #[inline]
            fn sub(self, other: T) -> $name {
                let (res, flag) = self.overflowing_sub(other);
                assert!(!flag, "attempt to subtract with overflow");
                res
            }
        }
        impl<T> ::core::ops::SubAssign<T> for $name
        where
            T: Into<$name>,
        {
            #[inline]
            fn sub_assign(&mut self, rhs: T) {
                self.0 = (*self - rhs).0
            }
        }

        impl<T> ::core::ops::Mul<T> for $name
        where
            T: Into<$name>,
        {
            type Output = $name;

            fn mul(self, other: T) -> $name {
                let (res, flag) = self.overflowing_mul(other);
                assert!(!flag, "attempt to mul with overflow");
                res
            }
        }
        impl<T> ::core::ops::MulAssign<T> for $name
        where
            T: Into<$name>,
        {
            #[inline]
            fn mul_assign(&mut self, rhs: T) {
                self.0 = (*self * rhs).0
            }
        }

        impl<T> ::core::ops::Div<T> for $name
        where
            T: Into<$name>,
        {
            type Output = $name;

            fn div(self, other: T) -> $name {
                self.div_rem(other.into()).0
            }
        }
        impl<T> ::core::ops::DivAssign<T> for $name
        where
            T: Into<$name>,
        {
            #[inline]
            fn div_assign(&mut self, rhs: T) {
                self.0 = (*self / rhs).0
            }
        }

        impl<T> ::core::ops::Rem<T> for $name
        where
            T: Into<$name>,
        {
            type Output = $name;

            fn rem(self, other: T) -> $name {
                self.div_rem(other.into()).1
            }
        }
        impl<T> ::core::ops::RemAssign<T> for $name
        where
            T: Into<$name>,
        {
            #[inline]
            fn rem_assign(&mut self, rhs: T) {
                self.0 = (*self % rhs).0
            }
        }

        impl<T> ::core::ops::BitAnd<T> for $name
        where
            T: Into<$name>,
        {
            type Output = $name;

            #[inline]
            fn bitand(self, other: T) -> $name {
                let $name(ref arr1) = self;
                let $name(ref arr2) = other.into();
                let mut ret = [0u64; $n_words];
                for i in 0..$n_words {
                    ret[i] = arr1[i] & arr2[i];
                }
                $name(ret)
            }
        }
        impl<T> ::core::ops::BitAndAssign<T> for $name
        where
            T: Into<$name>,
        {
            #[inline]
            fn bitand_assign(&mut self, rhs: T) {
                self.0 = (*self & rhs).0
            }
        }

        impl<T> ::core::ops::BitXor<T> for $name
        where
            T: Into<$name>,
        {
            type Output = $name;

            #[inline]
            fn bitxor(self, other: T) -> $name {
                let $name(ref arr1) = self;
                let $name(ref arr2) = other.into();
                let mut ret = [0u64; $n_words];
                for i in 0..$n_words {
                    ret[i] = arr1[i] ^ arr2[i];
                }
                $name(ret)
            }
        }
        impl<T> ::core::ops::BitXorAssign<T> for $name
        where
            T: Into<$name>,
        {
            #[inline]
            fn bitxor_assign(&mut self, rhs: T) {
                self.0 = (*self ^ rhs).0
            }
        }

        impl<T> ::core::ops::BitOr<T> for $name
        where
            T: Into<$name>,
        {
            type Output = $name;

            #[inline]
            fn bitor(self, other: T) -> $name {
                let $name(ref arr1) = self;
                let $name(ref arr2) = other.into();
                let mut ret = [0u64; $n_words];
                for i in 0..$n_words {
                    ret[i] = arr1[i] | arr2[i];
                }
                $name(ret)
            }
        }
        impl<T> ::core::ops::BitOrAssign<T> for $name
        where
            T: Into<$name>,
        {
            #[inline]
            fn bitor_assign(&mut self, rhs: T) {
                self.0 = (*self | rhs).0
            }
        }

        impl ::core::ops::Shl<usize> for $name {
            type Output = $name;

            fn shl(self, shift: usize) -> $name {
                let $name(ref original) = self;
                let mut ret = [0u64; $n_words];
                let word_shift = shift / 64;
                let bit_shift = shift % 64;
                for i in 0..$n_words {
                    // Shift
                    if bit_shift < 64 && i + word_shift < $n_words {
                        ret[i + word_shift] += original[i] << bit_shift;
                    }
                    // Carry
                    if bit_shift > 0 && i + word_shift + 1 < $n_words {
                        ret[i + word_shift + 1] += original[i] >> (64 - bit_shift);
                    }
                }
                $name(ret)
            }
        }
        impl ::core::ops::ShlAssign<usize> for $name {
            #[inline]
            fn shl_assign(&mut self, rhs: usize) {
                self.0 = (*self << rhs).0
            }
        }

        impl ::core::ops::Shr<usize> for $name {
            type Output = $name;

            fn shr(self, shift: usize) -> $name {
                let $name(ref original) = self;
                let mut ret = [0u64; $n_words];
                let word_shift = shift / 64;
                let bit_shift = shift % 64;
                for i in word_shift..$n_words {
                    // Shift
                    ret[i - word_shift] += original[i] >> bit_shift;
                    // Carry
                    if bit_shift > 0 && i < $n_words - 1 {
                        ret[i - word_shift] += original[i + 1] << (64 - bit_shift);
                    }
                }
                if self.is_negative() {
                    ret[$n_words - 1] |= 0x8000_0000_0000_0000
                }
                $name(ret)
            }
        }

        impl ::core::ops::ShrAssign<usize> for $name {
            #[inline]
            fn shr_assign(&mut self, rhs: usize) {
                self.0 = (*self >> rhs).0
            }
        }

        impl ::core::ops::Not for $name {
            type Output = $name;

            #[inline]
            fn not(self) -> $name {
                let $name(ref arr) = self;
                let mut ret = [0u64; $n_words];
                for i in 0..$n_words {
                    ret[i] = !arr[i];
                }
                $name(ret)
            }
        }

        impl ::core::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let &$name(ref data) = self;
                write!(f, "0x")?;
                for ch in data.iter().rev() {
                    write!(f, "{:016x}", ch)?;
                }
                Ok(())
            }
        }

        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Debug::fmt(self, f)
            }
        }

        #[cfg(feature = "alloc")]
        impl ::core::fmt::UpperHex for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> Result<(), ::core::fmt::Error> {
                use alloc::string::String;
                use alloc::format;

                let mut hex = String::new();
                for chunk in self.0.iter().rev().skip_while(|x| **x == 0) {
                    if hex.is_empty() {
                        hex.push_str(&format!("{:X}", chunk));
                    } else {
                        hex.push_str(&format!("{:0>16X}", chunk));
                    }
                }
                if hex.is_empty() {
                    hex.push_str("0");
                }

                let mut prefix = if f.alternate() {
                    String::from("0x")
                } else {
                    String::new()
                };
                if let Some(width) = f.width() {
                    if f.sign_aware_zero_pad() {
                        let missing_width =
                            width.saturating_sub(prefix.len()).saturating_sub(hex.len());
                        prefix.push_str(&"0".repeat(missing_width));
                    }
                }

                prefix.push_str(&hex);
                f.pad(&prefix)
            }
        }

        #[cfg(feature = "alloc")]
        impl ::core::fmt::LowerHex for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> Result<(), ::core::fmt::Error> {
                use alloc::string::String;
                use alloc::format;

                let mut hex = String::new();
                for chunk in self.0.iter().rev().skip_while(|x| **x == 0) {
                    if hex.is_empty() {
                        hex.push_str(&format!("{:x}", chunk));
                    } else {
                        hex.push_str(&format!("{:0>16x}", chunk));
                    }
                }
                if hex.is_empty() {
                    hex.push_str("0");
                }

                let mut prefix = if f.alternate() {
                    String::from("0x")
                } else {
                    String::new()
                };
                if let Some(width) = f.width() {
                    if f.sign_aware_zero_pad() {
                        let missing_width =
                            width.saturating_sub(prefix.len()).saturating_sub(hex.len());
                        prefix.push_str(&"0".repeat(missing_width));
                    }
                }

                prefix.push_str(&hex);
                f.pad(&prefix)
            }
        }

        #[cfg(feature = "alloc")]
        impl ::core::fmt::Octal for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> Result<(), ::core::fmt::Error> {
                use alloc::string::String;
                use alloc::format;

                let mut octal = String::new();
                for chunk in self.0.iter().rev().skip_while(|x| **x == 0) {
                    if octal.is_empty() {
                        octal.push_str(&format!("{:o}", chunk));
                    } else {
                        octal.push_str(&format!("{:0>22o}", chunk));
                    }
                }
                if octal.is_empty() {
                    octal.push_str("0");
                }

                let mut prefix = if f.alternate() {
                    String::from("0o")
                } else {
                    String::new()
                };
                if let Some(width) = f.width() {
                    if f.sign_aware_zero_pad() {
                        let missing_width = width
                            .saturating_sub(prefix.len())
                            .saturating_sub(octal.len());
                        prefix.push_str(&"0".repeat(missing_width));
                    }
                }

                prefix.push_str(&octal);
                f.pad(&prefix)
            }
        }

        #[cfg(feature = "alloc")]
        impl ::core::fmt::Binary for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> Result<(), ::core::fmt::Error> {
                use alloc::string::String;
                use alloc::format;

                let mut binary = String::new();
                for chunk in self.0.iter().rev().skip_while(|x| **x == 0) {
                    if binary.is_empty() {
                        binary.push_str(&format!("{:b}", chunk));
                    } else {
                        binary.push_str(&format!("{:0>64b}", chunk));
                    }
                }
                if binary.is_empty() {
                    binary.push_str("0");
                }

                let mut prefix = if f.alternate() {
                    String::from("0b")
                } else {
                    String::new()
                };
                if let Some(width) = f.width() {
                    if f.sign_aware_zero_pad() {
                        let missing_width = width
                            .saturating_sub(prefix.len())
                            .saturating_sub(binary.len());
                        prefix.push_str(&"0".repeat(missing_width));
                    }
                }

                prefix.push_str(&binary);
                f.pad(&prefix)
            }
        }

        #[cfg(feature = "serde")]
        impl $crate::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: $crate::serde::Serializer,
            {
                use $crate::hex::ToHex;
                let bytes = self.to_be_bytes();
                if serializer.is_human_readable() {
                    serializer.serialize_str(&bytes.to_hex())
                } else {
                    serializer.serialize_bytes(&bytes)
                }
            }
        }

        #[cfg(feature = "serde")]
        impl<'de> $crate::serde::Deserialize<'de> for $name {
            fn deserialize<D: $crate::serde::Deserializer<'de>>(
                deserializer: D,
            ) -> Result<Self, D::Error> {
                use ::std::fmt;
                use $crate::hex::FromHex;
                use $crate::serde::de;
                struct Visitor;
                impl<'de> de::Visitor<'de> for Visitor {
                    type Value = $name;

                    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(
                            f,
                            "{} bytes or a hex string with {} characters",
                            $n_words * 8,
                            $n_words * 8 * 2
                        )
                    }

                    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        let bytes = Vec::from_hex(s)
                            .map_err(|_| de::Error::invalid_value(de::Unexpected::Str(s), &self))?;
                        $name::from_be_slice(&bytes)
                            .map_err(|_| de::Error::invalid_length(bytes.len() * 2, &self))
                    }

                    fn visit_bytes<E>(self, bytes: &[u8]) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        $name::from_be_slice(bytes)
                            .map_err(|_| de::Error::invalid_length(bytes.len(), &self))
                    }
                }

                if deserializer.is_human_readable() {
                    deserializer.deserialize_str(Visitor)
                } else {
                    deserializer.deserialize_bytes(Visitor)
                }
            }
        }
    };
}

macro_rules! construct_signed_bigint_methods {
    ( $ name: ident, $ n_words: expr ) => {
        impl From<i8> for $name {
            fn from(init: i8) -> $name {
                let bytes = init.to_le_bytes();
                let mut ret = [if init.is_negative() {
                    ::core::u8::MAX
                } else {
                    0
                }; $n_words * 8];
                for i in 0..bytes.len() {
                    ret[i] = bytes[i]
                }
                $name::from_le_bytes(ret)
            }
        }

        impl From<i16> for $name {
            fn from(init: i16) -> $name {
                let bytes = init.to_le_bytes();
                let mut ret = [if init.is_negative() {
                    ::core::u8::MAX
                } else {
                    0
                }; $n_words * 8];
                for i in 0..bytes.len() {
                    ret[i] = bytes[i]
                }
                $name::from_le_bytes(ret)
            }
        }

        impl From<i32> for $name {
            fn from(init: i32) -> $name {
                let bytes = init.to_le_bytes();
                let mut ret = [if init.is_negative() {
                    ::core::u8::MAX
                } else {
                    0
                }; $n_words * 8];
                for i in 0..bytes.len() {
                    ret[i] = bytes[i]
                }
                $name::from_le_bytes(ret)
            }
        }

        impl From<i64> for $name {
            fn from(init: i64) -> $name {
                let bytes = init.to_le_bytes();
                let mut ret = [if init.is_negative() {
                    ::core::u8::MAX
                } else {
                    0
                }; $n_words * 8];
                for i in 0..bytes.len() {
                    ret[i] = bytes[i]
                }
                $name::from_le_bytes(ret)
            }
        }

        impl From<i128> for $name {
            fn from(init: i128) -> $name {
                let bytes = init.to_le_bytes();
                let mut ret = [if init.is_negative() {
                    ::core::u8::MAX
                } else {
                    0
                }; $n_words * 8];
                for i in 0..bytes.len() {
                    ret[i] = bytes[i]
                }
                $name::from_le_bytes(ret)
            }
        }

        impl $name {
            /// Minimum value
            pub const MIN: $name = {
                let mut min = [0u64; $n_words];
                min[$n_words - 1] = 0x8000_0000_0000_0000;
                $name(min)
            };

            /// Maximum value
            pub const MAX: $name = {
                let mut max = [::core::u64::MAX; $n_words];
                max[$n_words - 1] = ::core::u64::MAX >> 1;
                $name(max)
            };

            #[inline]
            pub fn is_positive(&self) -> bool {
                !self.is_zero()
                    && self[($name::INNER_LEN - 1) as usize] & 0x8000_0000_0000_0000 == 0
            }

            #[inline]
            pub fn is_negative(&self) -> bool {
                !self.is_zero() && !self.is_positive()
            }

            /// Calculates `self * rhs`
            ///
            /// Returns a tuple of the multiplication along with a boolean indicating
            /// whether an arithmetic overflow would occur. If an overflow would
            /// have occurred then the wrapped value is returned.
            pub fn overflowing_mul<T>(self, other: T) -> ($name, bool)
            where
                T: Into<$name>,
            {
                let sub = if self != $name::MIN { -self } else { self };
                let mut p_high = $name::ZERO;
                let mut p_low = other.into();
                let mut prev = false;
                for _i in 0..$name::BITS {
                    let p_low_trailing_bit = (p_low[0] & 1) != 0;
                    p_high = match (p_low_trailing_bit, prev) {
                        (false, true) => p_high.wrapping_add(self),
                        (true, false) => p_high.wrapping_add(sub),
                        _ => p_high,
                    };
                    prev = p_low_trailing_bit;
                    p_low >>= 1;
                    p_low = match p_high[0] & 1 {
                        0 => p_low & $name::MAX,
                        _ => p_low | $name::MIN,
                    };
                    p_high >>= 1;
                }
                let negative_overflow =
                    p_low.is_negative() && p_high != $name([::core::u64::MAX; $n_words]);
                let positive_overflow = !p_low.is_negative() && p_high != $name::ZERO;
                (p_low, negative_overflow || positive_overflow)
            }
        }
    };
}

macro_rules! construct_unsigned_bigint_methods {
    ( $ name: ident, $ n_words: expr ) => {
        impl $name {
            /// Minimum value
            pub const MIN: $name = $name([0u64; $n_words]);

            /// Maximum value
            pub const MAX: $name = $name([::core::u64::MAX; $n_words]);

            #[inline]
            pub fn is_positive(&self) -> bool {
                !self.is_zero()
            }

            #[inline]
            pub fn is_negative(&self) -> bool {
                false
            }

            /// Calculates `self * rhs`
            ///
            /// Returns a tuple of the multiplication along with a boolean indicating
            /// whether an arithmetic overflow would occur. If an overflow would
            /// have occurred then the wrapped value is returned.
            pub fn overflowing_mul<T>(self, other: T) -> ($name, bool)
            where
                T: Into<$name>,
            {
                let $name(ref me) = self;
                let $name(ref you) = other.into();
                let mut ret = [0u64; $n_words];
                let mut overflow = false;
                for i in 0..$n_words {
                    let mut carry = 0u64;
                    for j in 0..$n_words {
                        if i + j >= $n_words {
                            if me[i] > 0 && you[j] > 0 {
                                overflow = true
                            }
                            continue;
                        }
                        let prev_carry = carry;
                        let res = me[i] as u128 * you[j] as u128;
                        carry = (res >> 64) as u64;
                        let mul = (res & ::core::u64::MAX as u128) as u64;
                        let (res, flag) = ret[i + j].overflowing_add(mul);
                        carry += flag as u64;
                        ret[i + j] = res;
                        let (res, flag) = ret[i + j].overflowing_add(prev_carry);
                        carry += flag as u64;
                        ret[i + j] = res;
                    }
                    if carry > 0 {
                        overflow = true
                    }
                }
                (Self(ret), overflow)
            }
        }
    };
}

construct_bigint!(i256, 4);
construct_bigint!(i512, 8);
construct_bigint!(i1024, 16);
construct_bigint!(u256, 4);
construct_bigint!(u512, 8);
construct_bigint!(u1024, 16);

construct_unsigned_bigint_methods!(u256, 4);
construct_unsigned_bigint_methods!(u512, 8);
construct_unsigned_bigint_methods!(u1024, 16);
construct_signed_bigint_methods!(i256, 4);
construct_signed_bigint_methods!(i512, 8);
construct_signed_bigint_methods!(i1024, 16);

#[cfg(test)]
mod tests {
    #![allow(unused)]

    use super::*;

    construct_bigint!(Uint128, 2);
    construct_unsigned_bigint_methods!(Uint128, 2);

    #[test]
    fn u256_bits_test() {
        assert_eq!(u256::from(255u64).bits_required(), 8);
        assert_eq!(u256::from(256u64).bits_required(), 9);
        assert_eq!(u256::from(300u64).bits_required(), 9);
        assert_eq!(u256::from(60000u64).bits_required(), 16);
        assert_eq!(u256::from(70000u64).bits_required(), 17);

        // Try to read the following lines out loud quickly
        let mut shl = u256::from(70000u64);
        shl = shl << 100;
        assert_eq!(shl.bits_required(), 117);
        shl = shl << 100;
        assert_eq!(shl.bits_required(), 217);
        shl = shl << 100;
        assert_eq!(shl.bits_required(), 0);

        // Bit set check
        assert!(!u256::from(10u64).bit(0));
        assert!(u256::from(10u64).bit(1));
        assert!(!u256::from(10u64).bit(2));
        assert!(u256::from(10u64).bit(3));
        assert!(!u256::from(10u64).bit(4));
    }

    #[test]
    fn u256_display_test() {
        assert_eq!(
            format!("{}", u256::from(0xDEADBEEFu64)),
            "0x00000000000000000000000000000000000000000000000000000000deadbeef"
        );
        assert_eq!(
            format!("{}", u256::from(::core::u64::MAX)),
            "0x000000000000000000000000000000000000000000000000ffffffffffffffff"
        );

        let max_val = u256([
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
        ]);
        assert_eq!(
            format!("{}", max_val),
            "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
        );
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn fmt_hex() {
        let one = u256::ONE;
        let mut u_256 = u256([
            0x0000000000000000,
            0xAAAAAAAABBBBBBBB,
            0x0000000111122222,
            0x0000000000000000,
        ]);

        // UpperHex
        assert_eq!(
            format!("{:X}", u_256),
            "111122222AAAAAAAABBBBBBBB0000000000000000"
        );
        assert_eq!(
            format!("{:#X}", u_256),
            "0x111122222AAAAAAAABBBBBBBB0000000000000000"
        );
        assert_eq!(format!("{:X}", u256::ZERO), "0");
        assert_eq!(format!("{:05X}", one), "00001");
        assert_eq!(format!("{:#05X}", one), "0x001");
        assert_eq!(format!("{:5X}", one), "1    ");
        assert_eq!(format!("{:#5X}", one), "0x1  ");
        assert_eq!(format!("{:w^#7X}", one), "ww0x1ww");

        // LowerHex
        assert_eq!(
            format!("{:x}", u_256),
            "111122222aaaaaaaabbbbbbbb0000000000000000"
        );
        assert_eq!(
            format!("{:#x}", u_256),
            "0x111122222aaaaaaaabbbbbbbb0000000000000000"
        );
        assert_eq!(format!("{:x}", u256::ZERO), "0");
        assert_eq!(format!("{:05x}", one), "00001");
        assert_eq!(format!("{:#05x}", one), "0x001");
        assert_eq!(format!("{:5x}", one), "1    ");
        assert_eq!(format!("{:#5x}", one), "0x1  ");
        assert_eq!(format!("{:w^#7x}", one), "ww0x1ww");
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn fmt_octal() {
        let one = u256::ONE;
        let mut u_256 = u256([
            0o0000000000000000000000,
            0o0011222222222222222222,
            0o0000000001111111111111,
            0o0000000000000000000000,
        ]);

        assert_eq!(
            format!("{:o}", u_256),
            "111111111111100112222222222222222220000000000000000000000"
        );
        assert_eq!(
            format!("{:#o}", u_256),
            "0o111111111111100112222222222222222220000000000000000000000"
        );
        assert_eq!(format!("{:o}", u256::ZERO), "0");
        assert_eq!(format!("{:05o}", one), "00001");
        assert_eq!(format!("{:#05o}", one), "0o001");
        assert_eq!(format!("{:5o}", one), "1    ");
        assert_eq!(format!("{:#5o}", one), "0o1  ");
        assert_eq!(format!("{:w^#7o}", one), "ww0o1ww");
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn fmt_binary() {
        let one = u256::ONE;
        let mut u_256 = u256([
            0b0000000000000000000000000000000000000000000000000000000000000000,
            0b0001111000011110001111000011110001111000011110001111000011110000,
            0b0000000000000000000000000000001111111111111111111111111111111111,
            0b0000000000000000000000000000000000000000000000000000000000000000,
        ]);

        assert_eq!(
            format!("{:b}", u_256),
            "111111111111111111111111111111111100011110000111100011110000111100011110000111100011110000111100000000000000000000000000000000000000000000000000000000000000000000"
        );
        assert_eq!(
            format!("{:#b}", u_256),
            "0b111111111111111111111111111111111100011110000111100011110000111100011110000111100011110000111100000000000000000000000000000000000000000000000000000000000000000000"
        );
        assert_eq!(format!("{:b}", u256::ZERO), "0");
        assert_eq!(format!("{:05b}", one), "00001");
        assert_eq!(format!("{:#05b}", one), "0b001");
        assert_eq!(format!("{:5b}", one), "1    ");
        assert_eq!(format!("{:#5b}", one), "0b1  ");
        assert_eq!(format!("{:w^#7b}", one), "ww0b1ww");
    }

    #[test]
    fn u256_comp_test() {
        let small = u256([10u64, 0, 0, 0]);
        let big = u256([0x8C8C3EE70C644118u64, 0x0209E7378231E632, 0, 0]);
        let bigger = u256([0x9C8C3EE70C644118u64, 0x0209E7378231E632, 0, 0]);
        let biggest = u256([0x5C8C3EE70C644118u64, 0x0209E7378231E632, 0, 1]);

        assert!(small < big);
        assert!(big < bigger);
        assert!(bigger < biggest);
        assert!(bigger <= biggest);
        assert!(biggest <= biggest);
        assert!(bigger >= big);
        assert!(bigger >= small);
        assert!(small <= small);
    }

    #[test]
    fn uint_from_be_bytes() {
        assert_eq!(
            Uint128::from_be_bytes([
                0x1b, 0xad, 0xca, 0xfe, 0xde, 0xad, 0xbe, 0xef, 0xde, 0xaf, 0xba, 0xbe, 0x2b, 0xed,
                0xfe, 0xed
            ]),
            Uint128([0xdeafbabe2bedfeed, 0x1badcafedeadbeef])
        );

        assert_eq!(
            u256::from_be_bytes([
                0x1b, 0xad, 0xca, 0xfe, 0xde, 0xad, 0xbe, 0xef, 0xde, 0xaf, 0xba, 0xbe, 0x2b, 0xed,
                0xfe, 0xed, 0xba, 0xad, 0xf0, 0x0d, 0xde, 0xfa, 0xce, 0xda, 0x11, 0xfe, 0xd2, 0xba,
                0xd1, 0xc0, 0xff, 0xe0
            ]),
            u256([
                0x11fed2bad1c0ffe0,
                0xbaadf00ddefaceda,
                0xdeafbabe2bedfeed,
                0x1badcafedeadbeef
            ])
        );
    }

    #[test]
    fn uint_from_le_bytes() {
        let mut be = [
            0x1b, 0xad, 0xca, 0xfe, 0xde, 0xad, 0xbe, 0xef, 0xde, 0xaf, 0xba, 0xbe, 0x2b, 0xed,
            0xfe, 0xed,
        ];
        be.reverse();
        assert_eq!(
            Uint128::from_le_bytes(be),
            Uint128([0xdeafbabe2bedfeed, 0x1badcafedeadbeef])
        );

        let mut be = [
            0x1b, 0xad, 0xca, 0xfe, 0xde, 0xad, 0xbe, 0xef, 0xde, 0xaf, 0xba, 0xbe, 0x2b, 0xed,
            0xfe, 0xed, 0xba, 0xad, 0xf0, 0x0d, 0xde, 0xfa, 0xce, 0xda, 0x11, 0xfe, 0xd2, 0xba,
            0xd1, 0xc0, 0xff, 0xe0,
        ];
        be.reverse();
        assert_eq!(
            u256::from_le_bytes(be),
            u256([
                0x11fed2bad1c0ffe0,
                0xbaadf00ddefaceda,
                0xdeafbabe2bedfeed,
                0x1badcafedeadbeef
            ])
        );
    }

    #[test]
    fn uint_to_be_bytes() {
        assert_eq!(
            Uint128([0xdeafbabe2bedfeed, 0x1badcafedeadbeef]).to_be_bytes(),
            [
                0x1b, 0xad, 0xca, 0xfe, 0xde, 0xad, 0xbe, 0xef, 0xde, 0xaf, 0xba, 0xbe, 0x2b, 0xed,
                0xfe, 0xed
            ]
        );

        assert_eq!(
            u256([
                0x11fed2bad1c0ffe0,
                0xbaadf00ddefaceda,
                0xdeafbabe2bedfeed,
                0x1badcafedeadbeef
            ])
            .to_be_bytes(),
            [
                0x1b, 0xad, 0xca, 0xfe, 0xde, 0xad, 0xbe, 0xef, 0xde, 0xaf, 0xba, 0xbe, 0x2b, 0xed,
                0xfe, 0xed, 0xba, 0xad, 0xf0, 0x0d, 0xde, 0xfa, 0xce, 0xda, 0x11, 0xfe, 0xd2, 0xba,
                0xd1, 0xc0, 0xff, 0xe0
            ]
        );
    }

    #[test]
    fn uint_to_le_bytes() {
        assert_eq!(
            Uint128([0xdeafbabe2bedfeed, 0x1badcafedeadbeef]).to_le_bytes(),
            [
                0xed, 0xfe, 0xed, 0x2b, 0xbe, 0xba, 0xaf, 0xde, 0xef, 0xbe, 0xad, 0xde, 0xfe, 0xca,
                0xad, 0x1b
            ]
        );

        assert_eq!(
            u256([
                0x11fed2bad1c0ffe0,
                0xbaadf00ddefaceda,
                0xdeafbabe2bedfeed,
                0x1badcafedeadbeef
            ])
            .to_le_bytes(),
            [
                0xe0, 0xff, 0xc0, 0xd1, 0xba, 0xd2, 0xfe, 0x11, 0xda, 0xce, 0xfa, 0xde, 0x0d, 0xf0,
                0xad, 0xba, 0xed, 0xfe, 0xed, 0x2b, 0xbe, 0xba, 0xaf, 0xde, 0xef, 0xbe, 0xad, 0xde,
                0xfe, 0xca, 0xad, 0x1b,
            ]
        );
    }

    #[test]
    fn u256_div_rem_checked() {
        let zero = u256::ZERO;
        let number_one = u256::from(0xDEADBEEFu64);
        let number_two = u256::from(::core::u64::MAX);
        let one_div_rem_two = (
            u256::from(::core::u64::MAX / 0xDEADBEEFu64),
            u256::from(::core::u64::MAX % 0xDEADBEEFu64),
        );
        let max = u256::MAX;

        // Division by zero gets not panic and gets None
        assert_eq!(u256::div_rem_checked(max, zero), None);
        assert_eq!(u256::div_rem_checked(number_two, zero), None);
        assert_eq!(u256::div_rem_checked(number_one, zero), None);

        // Division of zero gets Zero
        assert_eq!(u256::div_rem_checked(zero, max), Some((zero, zero)));
        assert_eq!(u256::div_rem_checked(zero, number_two), Some((zero, zero)));
        assert_eq!(u256::div_rem_checked(zero, number_one), Some((zero, zero)));

        // Division by another than zero not gets None
        assert_ne!(u256::div_rem_checked(max, number_one), None);
        assert_ne!(u256::div_rem_checked(number_two, number_one), None);

        // In u256 division gets the same as in u64
        assert_eq!(
            u256::div_rem_checked(number_two, number_one),
            Some(one_div_rem_two)
        );
    }

    #[test]
    fn u256_div_rem() {
        let zero = u256::ZERO;
        let number_one = u256::from(0xDEADBEEFu64);
        let number_two = u256::from(::core::u64::MAX);
        let max = u256::MAX;

        let result1 = std::panic::catch_unwind(|| u256::div_rem(max, zero));
        assert!(result1.is_err());
        let result2 = std::panic::catch_unwind(|| u256::div_rem(number_one, zero));
        assert!(result2.is_err());
        let result3 = std::panic::catch_unwind(|| u256::div_rem(number_two, zero));
        assert!(result3.is_err());
    }

    #[test]
    fn bigint_min_max() {
        assert_eq!(u256::MIN.as_inner(), &[0u64; 4]);
        assert_eq!(u512::MIN.as_inner(), &[0u64; 8]);
        assert_eq!(u1024::MIN.as_inner(), &[0u64; 16]);
        assert_eq!(u256::MAX.as_inner(), &[::core::u64::MAX; 4]);
        assert_eq!(u512::MAX.as_inner(), &[::core::u64::MAX; 8]);
        assert_eq!(u1024::MAX.as_inner(), &[::core::u64::MAX; 16]);
        assert_eq!(u256::BITS, 4 * 64);
        assert_eq!(u512::BITS, 8 * 64);
        assert_eq!(u1024::BITS, 16 * 64);
    }

    #[test]
    fn u256_arithmetic_test() {
        let init = u256::from(0xDEADBEEFDEADBEEFu64);
        let copy = init;

        let add = init + copy;
        assert_eq!(add, u256([0xBD5B7DDFBD5B7DDEu64, 1, 0, 0]));
        // Bitshifts
        let shl = add << 88;
        assert_eq!(shl, u256([0u64, 0xDFBD5B7DDE000000, 0x1BD5B7D, 0]));
        let shr = shl >> 40;
        assert_eq!(shr, u256([0x7DDE000000000000u64, 0x0001BD5B7DDFBD5B, 0, 0]));
        // Increment
        let mut incr = shr;
        incr += 1u32;
        assert_eq!(
            incr,
            u256([0x7DDE000000000001u64, 0x0001BD5B7DDFBD5B, 0, 0])
        );
        // Subtraction
        let sub = incr - init;
        assert_eq!(sub, u256([0x9F30411021524112u64, 0x0001BD5B7DDFBD5A, 0, 0]));
        // Multiplication
        let mult = sub * 300u32;
        assert_eq!(
            mult,
            u256([0x8C8C3EE70C644118u64, 0x0209E7378231E632, 0, 0])
        );
        // Division
        assert_eq!(u256::from(105u64) / u256::from(5u64), u256::from(21u64));
        let div = mult / u256::from(300u64);
        assert_eq!(div, u256([0x9F30411021524112u64, 0x0001BD5B7DDFBD5A, 0, 0]));

        assert_eq!(u256::from(105u64) % u256::from(5u64), u256::from(0u64));
        assert_eq!(
            u256::from(35498456u64) % u256::from(3435u64),
            u256::from(1166u64)
        );
        let rem_src = mult * u256::from(39842u64) + u256::from(9054u64);
        assert_eq!(rem_src % u256::from(39842u64), u256::from(9054u64));
        // TODO: bit inversion
    }

    #[test]
    fn mul_u32_test() {
        let u64_val = u256::from(0xDEADBEEFDEADBEEFu64);

        let u96_res = u64_val * 0xFFFFFFFFu32;
        let u128_res = u96_res * 0xFFFFFFFFu32;
        let u160_res = u128_res * 0xFFFFFFFFu32;
        let u192_res = u160_res * 0xFFFFFFFFu32;
        let u224_res = u192_res * 0xFFFFFFFFu32;
        let u256_res = u224_res * 0xFFFFFFFFu32;

        assert_eq!(u96_res, u256([0xffffffff21524111u64, 0xDEADBEEE, 0, 0]));
        assert_eq!(
            u128_res,
            u256([0x21524111DEADBEEFu64, 0xDEADBEEE21524110, 0, 0])
        );
        assert_eq!(
            u160_res,
            u256([0xBD5B7DDD21524111u64, 0x42A4822200000001, 0xDEADBEED, 0])
        );
        assert_eq!(
            u192_res,
            u256([
                0x63F6C333DEADBEEFu64,
                0xBD5B7DDFBD5B7DDB,
                0xDEADBEEC63F6C334,
                0
            ])
        );
        assert_eq!(
            u224_res,
            u256([
                0x7AB6FBBB21524111u64,
                0xFFFFFFFBA69B4558,
                0x854904485964BAAA,
                0xDEADBEEB
            ])
        );
        assert_eq!(
            u256_res,
            u256([
                0xA69B4555DEADBEEFu64,
                0xA69B455CD41BB662,
                0xD41BB662A69B4550,
                0xDEADBEEAA69B455C
            ])
        );
    }

    #[test]
    fn multiplication_test() {
        let u64_val = u256::from(0xDEADBEEFDEADBEEFu64);

        let u128_res = u64_val * u64_val;

        assert_eq!(
            u128_res,
            u256([0x048D1354216DA321u64, 0xC1B1CD13A4D13D46, 0, 0])
        );

        let u256_res = u128_res * u128_res;

        assert_eq!(
            u256_res,
            u256([
                0xF4E166AAD40D0A41u64,
                0xF5CF7F3618C2C886u64,
                0x4AFCFF6F0375C608u64,
                0x928D92B4D7F5DF33u64
            ])
        );
    }

    #[test]
    fn u256_extreme_bitshift_test() {
        // Shifting a u64 by 64 bits gives an undefined value, so make sure that
        // we're doing the Right Thing here
        let init = u256::from(0xDEADBEEFDEADBEEFu64);

        assert_eq!(init << 64, u256([0, 0xDEADBEEFDEADBEEF, 0, 0]));
        let add = (init << 64) + init;
        assert_eq!(add, u256([0xDEADBEEFDEADBEEF, 0xDEADBEEFDEADBEEF, 0, 0]));
        assert_eq!(
            add >> 0,
            u256([0xDEADBEEFDEADBEEF, 0xDEADBEEFDEADBEEF, 0, 0])
        );
        assert_eq!(
            add << 0,
            u256([0xDEADBEEFDEADBEEF, 0xDEADBEEFDEADBEEF, 0, 0])
        );
        assert_eq!(add >> 64, u256([0xDEADBEEFDEADBEEF, 0, 0, 0]));
        assert_eq!(
            add << 64,
            u256([0, 0xDEADBEEFDEADBEEF, 0xDEADBEEFDEADBEEF, 0])
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn u256_serde_test() {
        let check = |uint, hex| {
            let json = format!("\"{}\"", hex);
            assert_eq!(::serde_json::to_string(&uint).unwrap(), json);
            assert_eq!(::serde_json::from_str::<u256>(&json).unwrap(), uint);

            let bin_encoded = ::bincode::serialize(&uint).unwrap();
            let bin_decoded: u256 = ::bincode::deserialize(&bin_encoded).unwrap();
            assert_eq!(bin_decoded, uint);
        };

        check(
            u256::from(0u64),
            "0000000000000000000000000000000000000000000000000000000000000000",
        );
        check(
            u256::from(0xDEADBEEFu64),
            "00000000000000000000000000000000000000000000000000000000deadbeef",
        );
        check(
            u256([0xaa11, 0xbb22, 0xcc33, 0xdd44]),
            "000000000000dd44000000000000cc33000000000000bb22000000000000aa11",
        );
        check(
            u256([
                u64::max_value(),
                u64::max_value(),
                u64::max_value(),
                u64::max_value(),
            ]),
            "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        );
        check(
            u256([
                0xA69B4555DEADBEEF,
                0xA69B455CD41BB662,
                0xD41BB662A69B4550,
                0xDEADBEEAA69B455C,
            ]),
            "deadbeeaa69b455cd41bb662a69b4550a69b455cd41bb662a69b4555deadbeef",
        );

        assert!(::serde_json::from_str::<u256>(
            "\"fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffg\""
        )
        .is_err()); // invalid char
        assert!(::serde_json::from_str::<u256>(
            "\"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\""
        )
        .is_err()); // invalid length
        assert!(::serde_json::from_str::<u256>(
            "\"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\""
        )
        .is_err()); // invalid length
    }

    #[test]
    fn i256_test() {
        let x = i256::from(1);
        let y = i256::from(1);
        assert_eq!(x.checked_add(y), Some(i256::from(2)));
    }

    #[test]
    fn i256_is_positive_test() {
        assert_eq!(true, i256::from(1).is_positive());
        assert_eq!(false, i256::from(-1).is_positive());
        assert_eq!(false, i256::from(0).is_positive());
        assert_eq!(true, i256::MAX.is_positive());
        assert_eq!(false, i256::MIN.is_positive());
        assert_eq!(true, i256::MIN.is_negative());
    }

    #[test]
    fn i256_add_test() {
        assert_eq!(
            (i256::from(3), false),
            i256::from(1).overflowing_add(i256::from(2))
        );
        assert_eq!(
            (i256::from(1), false),
            i256::from(-1).overflowing_add(i256::from(2))
        );
        assert_eq!(
            (i256::from(-2), false),
            i256::from(-1).overflowing_add(i256::from(-1))
        );
        assert_eq!(
            (i256::from(0), false),
            i256::from(0).overflowing_add(i256::from(0))
        );
        assert_eq!((i256::MIN, true), i256::from(1).overflowing_add(i256::MAX));
    }

    #[test]
    fn i256_sub_test() {
        assert_eq!(
            (i256::from(-1), false),
            i256::from(1).overflowing_sub(i256::from(2))
        );
        assert_eq!(
            (i256::from(1), false),
            i256::from(3).overflowing_sub(i256::from(2))
        );
        assert_eq!(
            (i256::from(-3), false),
            i256::from(-4).overflowing_sub(i256::from(-1))
        );
        assert_eq!(
            (i256::from(0), false),
            i256::from(0).overflowing_add(i256::from(0))
        );
        assert_eq!((i256::MIN, false), i256::from(0).overflowing_sub(i256::MIN));
        assert_eq!((-i256::ONE, false), i256::MAX.overflowing_sub(i256::MIN));
        assert_eq!(
            (i256::MAX, true),
            (-i256::from(2)).overflowing_sub(i256::MAX)
        );
    }

    #[test]
    fn i256_neg_test() {
        assert_eq!(i256::from(1), -i256::from(-1));
        assert_eq!(i256::from(-1), -i256::from(1));
        assert_eq!(i256::from(0), -i256::from(0));
        assert_eq!(i256::MIN + 1, -i256::MAX);
    }

    #[test]
    #[should_panic]
    fn i256_neg_min_test() {
        assert_eq!(-i256::MIN, -i256::MIN);
    }

    #[test]
    fn i256_mul_test() {
        assert_eq!(
            (i256::from(-12), false),
            i256::from(3).overflowing_mul(i256::from(-4))
        );
        assert_eq!(
            (i256::from(6), false),
            i256::from(2).overflowing_mul(i256::from(3))
        );
        assert_eq!(
            (i256::from(30), false),
            i256::from(-6).overflowing_mul(i256::from(-5))
        );
        assert_eq!(
            (i256::from(-2), true),
            i256::MAX.overflowing_mul(i256::from(2))
        );
        assert_eq!((i256::ZERO, true), i256::MIN.overflowing_mul(i256::from(2)));
        assert_eq!((i256::ONE, true), i256::MAX.overflowing_mul(i256::MAX));
    }

    #[test]
    fn i256_arithmetic_shr_test() {
        assert_eq!(i256::from(-1), i256::from(-1) >> 1);
        assert_eq!(i256::from(-1), i256::from(-2) >> 1);
        assert_eq!(i256::from(1), i256::from(2) >> 1);
        assert_eq!(i256::from(1), i256::from(2) >> 1);
        assert_eq!(i256::from(0), i256::from(1) >> 1);
    }

    #[test]
    fn i256_bits_required_test() {
        assert_eq!(i256::from(255u64).bits_required(), 8);
        assert_eq!(i256::from(256u64).bits_required(), 9);
        assert_eq!(i256::from(300u64).bits_required(), 9);
        assert_eq!(i256::from(60000u64).bits_required(), 16);
        assert_eq!(i256::from(70000u64).bits_required(), 17);
        assert_eq!(i256::from(-128i64).bits_required(), 8);
        assert_eq!(i256::from(-129i128).bits_required(), 9);
        assert_eq!(i256::from(0i32).bits_required(), 0);
        assert_eq!(i256::from(-1i16).bits_required(), 1);
        assert_eq!(i256::from(-2i64).bits_required(), 2);
        assert_eq!(i256::MIN.bits_required(), 256);
        assert_eq!(i256::MAX.bits_required(), 255);
    }

    #[test]
    fn i256_div_test() {
        assert_eq!(
            (i256::from(3), i256::from(1)),
            i256::from(7).div_rem_checked(i256::from(2i32)).unwrap()
        );
        assert_eq!(
            (i256::from(-3), i256::from(1)),
            i256::from(7).div_rem_checked(i256::from(-2i128)).unwrap()
        );
        assert_eq!(
            (i256::from(-3), i256::from(-1)),
            i256::from(-7).div_rem_checked(i256::from(2)).unwrap()
        );
        assert_eq!(
            (i256::from(3), i256::from(-1)),
            i256::from(-7).div_rem_checked(i256::from(-2)).unwrap()
        );
        let res = std::panic::catch_unwind(|| i256::div_rem(i256::MAX, i256::ZERO));
        assert!(res.is_err());
        let res = std::panic::catch_unwind(|| i256::div_rem(i256::MIN, i256::from(-1)));
        assert!(res.is_err());
    }

    #[test]
    fn i256_cmp_test() {
        assert!(i256::ZERO < i256::ONE);
        assert!(-i256::ONE < i256::ZERO);
        assert!(i256::MIN < i256::MAX);
        assert!(i256::MIN < i256::ZERO);
        assert!(i256::from(200) < i256::from(10000000));
        assert!(i256::from(-3) < i256::from(87));
    }
}
