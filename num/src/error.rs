// Rust language amplification library providing multiple generic trait
// implementations, type wrappers, derive macros and other language enhancements
//
// Written in 2014 by
//     Andrew Poelstra <apoelstra@wpsoftware.net>
// Updated in 2020-2021 by
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

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
/// Error indicating that a value does not fit integer dimension
pub struct OverflowError {
    /// Integer bit size
    pub max: usize,
    /// Value that overflows
    pub value: usize,
}

impl core::fmt::Display for OverflowError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Unable to construct bit-sized integer from a value `{}` overflowing max value `{}`",
            self.value, self.max
        )
    }
}

#[cfg(feature = "std")]
impl std::error::Error for OverflowError {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DivError {
    ZeroDiv,
    Overflow,
}

impl core::fmt::Display for DivError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            DivError::ZeroDiv => write!(f, "division by zero"),
            DivError::Overflow => write!(f, "division with overflow"),
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
/// Invalid slice length
pub struct ParseLengthError {
    /// The length of the slice de-facto
    pub actual: usize,
    /// The required length of the slice
    pub expected: usize,
}

impl core::fmt::Display for ParseLengthError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "Invalid length: got {}, expected {}",
            self.actual, self.expected
        )
    }
}
#[cfg(feature = "std")]
impl std::error::Error for ParseLengthError {}
