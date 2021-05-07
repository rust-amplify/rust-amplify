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

use ::std::io;
use ::std::fmt::{Display, Formatter, self};

use crate::Wrapper;

/// Copyable & cloneable I/O error type represented by the error kind function.
///
/// Available only when both `std` and `derive` features are present.
///
/// # Example
/// ```compile_fail
/// #[macro_use]
/// extern crate amplify_derive;
/// use amplify::IoError;
///
/// #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, From, Debug, Display, Error)]
/// enum Error {
///     #[from(::std::io::Error)]
///     #[display(inner)]
///     Io(IoError),
/// }
/// ```
#[derive(Wrapper, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Error, From)]
#[wrapper(Debug)]
#[amplify_crate(crate)]
pub struct IoError(io::ErrorKind);

impl Display for IoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let err = io::Error::from(*self.as_inner());
        Display::fmt(&err, f)
    }
}

impl From<io::Error> for IoError {
    fn from(err: io::Error) -> Self {
        IoError::from_inner(err.kind())
    }
}

impl From<IoError> for io::Error {
    fn from(err: IoError) -> Self {
        io::Error::from(err.into_inner())
    }
}
