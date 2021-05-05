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

use syn::{Path, Lit};
use syn::parse::{Parse, Result, ParseBuffer};
use syn::ext::IdentExt;
use syn::punctuated::Punctuated;
use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;

use crate::{ArgValue, Error};

/// Drop-in replacement for [`syn::NestedMeta`], which allows to parse
/// attributes which can have arguments made of either literal, path or
/// [`MetaArgNameValue`] expressions.
pub struct MetaArgList {
    /// List of arguments
    pub list: Punctuated<MetaArg, Token![,]>,
}

impl Parse for MetaArgList {
    fn parse(input: &ParseBuffer) -> Result<Self> {
        let content;
        parenthesized!(content in input);
        let list = Punctuated::parse_terminated(&content)?;
        Ok(MetaArgList { list })
    }
}

impl ToTokens for MetaArgList {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        (quote! { ( list ) }).to_tokens(tokens);
    }
}

/// Drop-in replacement for [`syn::NestedMeta`], which allows to parse
/// attributes which can have arguments made of either literal, path or
/// [`MetaArgNameValue`] expressions.
pub enum MetaArg {
    /// Attribute argument in form of literal
    Literal(Lit),

    /// Attribute argument in form of a path
    Path(Path),

    /// Attribute argument in form of `name = value` expression, where value
    /// can be any [`ArgValue`]-representable data
    NameValue(MetaArgNameValue),
}

impl Parse for MetaArg {
    fn parse(input: &ParseBuffer) -> Result<Self> {
        if input.peek2(Token![=]) {
            input.parse().map(MetaArg::NameValue)
        } else if input.peek(Ident::peek_any)
            || input.peek(Token![::]) && input.peek3(Ident::peek_any)
        {
            input.parse().map(MetaArg::Path)
        } else {
            input.parse().map(MetaArg::Literal)
        }
    }
}

impl ToTokens for MetaArg {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            MetaArg::Literal(lit) => lit.to_tokens(tokens),
            MetaArg::Path(path) => path.to_tokens(tokens),
            MetaArg::NameValue(meta) => meta.to_tokens(tokens),
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
        let path: Path = input.parse()?;
        Ok(MetaArgNameValue {
            name: path.get_ident().ok_or(Error::ArgNameMustBeIdent)?.clone(),
            eq_token: input.parse()?,
            value: input.parse()?,
        })
    }
}

impl ToTokens for MetaArgNameValue {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.name.to_tokens(tokens);
        self.eq_token.to_tokens(tokens);
        self.value.to_tokens(tokens);
    }
}

impl Parse for ArgValue {
    fn parse(input: &ParseBuffer) -> Result<Self> {
        if input.peek(Lit) {
            input.parse().map(ArgValue::Literal)
        } else {
            input.parse().map(ArgValue::Type)
        }
    }
}

impl ToTokens for ArgValue {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            ArgValue::Literal(lit) => lit.to_tokens(tokens),
            ArgValue::Type(ty) => ty.to_tokens(tokens),
            ArgValue::None => quote! { ! }.to_tokens(tokens),
        }
    }
}
