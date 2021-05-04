// Rust language amplification derive library providing multiple generic trait
// implementations, type wrappers, derive macros and other language enhancements
//
// Written in 2019-2021 by
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

use syn::parse::{Parse, Result, ParseBuffer};
use proc_macro2::Ident;

use crate::ArgValue;

/// Drop-in replacement for [`syn::MetaNameValue`] used for parsing named
/// arguments inside attributes which name is always an [`proc_macro2::Ident`]
/// (and not [`syn::Path`]) and value can be not only a literal, but of any
/// valid rust type.
pub struct MetaArgNameValue {
    /// Argument name
    pub name: Ident,
    /// Token placeholder
    pub eq_token: Token![=],
    /// Argument value
    pub value: ArgValue,
}

impl Parse for MetaArgNameValue {
    fn parse(input: &ParseBuffer) -> Result<Self> {
        Ok(MetaArgNameValue {
            name: input.parse()?,
            eq_token: input.parse()?,
            value: input.parse()?,
        })
    }
}
