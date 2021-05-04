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
use syn::{Path, Lit};
use proc_macro2::Ident;

use crate::ArgValue;

/// Drop-in replacement for [`syn::NestedMeta`], which allows to parse
/// attributes which can have arguments made of either literal, path or
/// [`MetaArgNameValue`] expressions.
pub enum MetaArgs {
    /// Attribute argument in form of literal
    Literal(Lit),

    /// Attribute argument in form of a path
    Path(Path),

    /// Attribute argument in form of `name = value` expression, where value
    /// can be any [`ArgValue`]-representable data
    NameValue(MetaArgNameValue),
}

impl Parse for MetaArgs {
    fn parse(input: &ParseBuffer) -> Result<Self> {
        if let Ok(lit) = Lit::parse(input) {
            Ok(MetaArgs::Literal(lit))
        } else if let Ok(path) = Path::parse(input) {
            Ok(MetaArgs::Path(path))
        } else if let Ok(meta) = MetaArgNameValue::parse(input) {
            Ok(MetaArgs::NameValue(meta))
        } else {
            Err(syn::Error::new(
                input.span(),
                "Attribute argument must be a rust literal, path or a `name=value` expression",
            ))
        }
    }
}

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
