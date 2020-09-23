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
    Attribute, Data, DataStruct, DeriveInput, Error, Field, Fields, Ident, Index, Lit, Member,
    Meta, MetaList, NestedMeta, Path, Result, Type, TypeSlice,
};

pub(crate) fn inner(input: DeriveInput) -> Result<TokenStream2> {
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let ident_name = &input.ident;

    Ok(quote! {
        impl #impl_generics ::amplify::AsAny for #ident_name #ty_generics #where_clause {
           fn as_any(&self) -> &dyn ::core::any::Any {
                self as &dyn ::core::any::Any
            }
        }
    })
}
