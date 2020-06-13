// Rust language extension library providing multiple generic trait
// implementations, type wrappers, derive macros and other "language warps"
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

use std::any::Any;

// TODO: We can't do a default implementation with current rust compiler
//       limitations, but we can do a derive macro for an automatic
//       implementation of the trait, which is trivial
pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}
