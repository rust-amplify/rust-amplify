// Rust language amplification derive library providing multiple generic trait
// implementations, type wrappers, derive macros and other language enhancements
//
// Written in 2019-2020 by
//     Elichai Turkel <elichai.turkel@gmail.com>
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

use syn::export::{Span, ToTokens, TokenStream, TokenStream2};
use syn::spanned::Spanned;
use syn::{
    Attribute, Data, DataEnum, DataStruct, DeriveInput, Error, Field, Fields, Ident, Index, Lit,
    Member, Meta, MetaList, NestedMeta, Path, Result, Type, TypeSlice,
};

pub(crate) fn inner(input: DeriveInput) -> Result<TokenStream2> {
    match input.data {
        Data::Struct(ref data) => Err(Error::new_spanned(
            &input,
            "Deriving `Display` can be done only with enums, not with structs",
        )),
        Data::Enum(ref data) => inner_enum(&input, data),
        //strict_encode_inner_enum(&input, &data),
        Data::Union(_) => Err(Error::new_spanned(
            &input,
            "Deriving `Display` can be done only with enums, not with unions",
        )),
    }
}

fn inner_enum(input: &DeriveInput, data: &DataEnum) -> Result<TokenStream2> {
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let ident_name = &input.ident;

    let name = "display";

    let mut display = vec![];

    for v in &data.variants {
        let type_name = &v.ident;
        let type_str = format!("\"{}\"", type_name);

        match &v.fields {
            Fields::Named(_) => {
                return Err(Error::new(
                    v.span(),
                    "`DeriveEnum` does not support enums with named variants",
                ))
            }
            Fields::Unnamed(args) => {
                return Err(Error::new(
                    v.span(),
                    "`DeriveEnum` does not support enums with unnamed variants",
                ))
            }
            Fields::Unit => {
                display.push(quote_spanned! { v.span() =>
                    Self::#type_name => f.write_str(#type_str),
                });
            }
        }
    }

    Ok(quote! {
        impl #impl_generics ::std::fmt::Display for #ident_name #ty_generics #where_clause {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                match self {
                    #( #display )*
                }
            }
        }
    })
}
