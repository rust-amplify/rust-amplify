// Rust language amplification library providing multiple generic trait
// implementations, type wrappers, derive macros and other language enhancements
//
// Written in 2019-2022 by
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

use std::cmp::Ordering;
use std::io;
use std::fmt::{Debug, Display, Formatter, self};
use std::error::Error as StdError;
use std::hash::{Hash, Hasher};

/// Copyable & cloneable I/O error type represented by the error kind function.
///
/// Available only when both `std` and `derive` features are present.
///
/// # Example
/// ```
/// use amplify::{IoError, Error, Display, From};
///
/// #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, From, Debug, Display, Error)]
/// enum Error {
///     #[from(std::io::Error)]
///     #[display(inner)]
///     Io(IoError),
/// }
/// ```
#[derive(Error)]
pub struct IoError {
    kind: io::ErrorKind,
    display: String,
    debug: String,
    details: Option<Box<dyn StdError + Send + Sync>>,
}

impl IoError {
    /// Returns [`io::ErrorKind`] of this error.
    pub fn kind(&self) -> io::ErrorKind {
        self.kind
    }
}

impl Clone for IoError {
    fn clone(&self) -> Self {
        Self {
            kind: self.kind,
            display: self.display.clone(),
            debug: self.debug.clone(),
            details: None,
        }
    }
}

impl PartialEq for IoError {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.debug == other.debug
    }
}

impl Eq for IoError {}

impl PartialOrd for IoError {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for IoError {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.kind.cmp(&other.kind) {
            Ordering::Equal => self.debug.cmp(&other.debug),
            ordering => ordering,
        }
    }
}

impl Hash for IoError {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.debug.as_bytes())
    }
}

impl Display for IoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let err = io::Error::from(self.clone());
        Display::fmt(&err, f)
    }
}

impl Debug for IoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let err = io::Error::from(self.clone());
        Debug::fmt(&err, f)
    }
}

impl From<io::Error> for IoError {
    fn from(err: io::Error) -> Self {
        IoError {
            kind: err.kind(),
            display: err.to_string(),
            debug: format!("{:?}", err),
            details: err.into_inner(),
        }
    }
}

impl From<io::ErrorKind> for IoError {
    fn from(kind: io::ErrorKind) -> Self {
        IoError {
            kind,
            display: kind.to_string(),
            debug: format!("{:?}", kind),
            details: None,
        }
    }
}

impl From<IoError> for io::Error {
    fn from(err: IoError) -> Self {
        match err.details {
            Some(details) => io::Error::new(err.kind, details),
            None => io::Error::from(err.kind),
        }
    }
}
