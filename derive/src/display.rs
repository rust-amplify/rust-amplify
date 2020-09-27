// Rust language amplification derive library providing multiple generic trait
// implementations, type wrappers, derive macros and other language enhancements
//
// Written in 2019-2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//     Elichai Turkel <elichai.turkel@gmail.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

use amplify::proc_macro::attr_named_value;
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
            "Deriving `DisplayEnum` can be done only with enums, not with structs",
        )),
        Data::Enum(ref data) => inner_enum(&input, data),
        //strict_encode_inner_enum(&input, &data),
        Data::Union(_) => Err(Error::new_spanned(
            &input,
            "Deriving `DisplayEnum` can be done only with enums, not with unions",
        )),
    }
}

fn inner_enum(input: &DeriveInput, data: &DataEnum) -> Result<TokenStream2> {
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let ident_name = &input.ident;

    let name = "display";

    let mut display = vec![];
    let example = r#"#[display="String for display"]"#;

    for v in &data.variants {
        let type_name = &v.ident;
        let type_str = format!("{}", type_name);

        let display_str = attr_named_value(&v.attrs, name, example)?
            .map(|lit| match lit {
                Lit::Str(display) => Ok(display),
                _ => proc_macro_err!(
                    ident_name,
                    "non-string literal for display parameter",
                    example
                ),
            })
            .map_or(Ok(None), |r| r.map(Some))?;

        match (&v.fields, &display_str) {
            (Fields::Named(_), None) => {
                display.push(quote_spanned! { v.span() =>
                    Self::#type_name { .. } => f.write_str(concat!(#type_str, " { .. }")),
                });
            }
            (Fields::Unnamed(_), None) => {
                display.push(quote_spanned! { v.span() =>
                    Self::#type_name(..) => f.write_str(concat!(#type_str, "(..)")),
                });
            }
            (Fields::Unit, None) => {
                display.push(quote_spanned! { v.span() =>
                    Self::#type_name => f.write_str(#type_str),
                });
            }
            (Fields::Named(fields), Some(display_str)) => {
                let idents = fields
                    .named
                    .iter()
                    .map(|f| f.ident.as_ref().unwrap())
                    .collect::<Vec<_>>();
                display.push(quote_spanned! { v.span() =>
                    Self::#type_name { #( #idents, )* } => write!(f, #display_str, #( #idents = #idents, )*),
                });
            }
            (Fields::Unnamed(fields), Some(display_str)) => {
                let idents = (0..fields.unnamed.len())
                    .map(|i| Ident::new(&format!("_{}", i), v.span()))
                    .collect::<Vec<_>>();
                display.push(quote_spanned! { v.span() =>
                    Self::#type_name ( #( #idents, )* ) => write!(f, #display_str, #( #idents = #idents, )*),
                });
            }
            (Fields::Unit, Some(display_str)) => {
                display.push(quote_spanned! { v.span() =>
                    Self::#type_name => f.write_str(#display_str),
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
