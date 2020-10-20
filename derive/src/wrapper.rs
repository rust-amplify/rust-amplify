// Rust language amplification derive library providing multiple generic trait
// implementations, type wrappers, derive macros and other language enhancements
//
// Written in 2019-2020 by
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

use syn::export::{TokenStream2};
use syn::{
    DeriveInput, Result, Data, Error, Fields, Index, Meta, MetaList, Path, NestedMeta,
    spanned::Spanned,
};

const NAME: &'static str = "wrapper";
const EXAMPLE: &'static str = r#"#[wrapper(LowerHex, Add)]"#;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum WrapperDerives {
    FromStr,
    Octal,
    LowerHex,
    UpperHex,
    LowerExp,
    UpperExp,
    Index,
    IndexMut,
    Add,
    Sub,
    Mul,
    Div,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
}

impl WrapperDerives {
    pub fn from_path(path: &Path) -> Result<Option<Self>> {
        path.segments.first().map_or(
            Err(attr_err!(
                path.span(),
                NAME,
                "must contain at least one identifier",
                EXAMPLE
            )),
            |segment| {
                Ok(match segment.ident.to_string().as_str() {
                    "FromStr" => Some(Self::FromStr),
                    "Octal" => Some(Self::Octal),
                    "LowerHex" => Some(Self::LowerHex),
                    "UpperHex" => Some(Self::UpperHex),
                    "LowerExp" => Some(Self::LowerExp),
                    "UpperExp" => Some(Self::UpperExp),
                    "Index" => Some(Self::Index),
                    "IndexMut" => Some(Self::IndexMut),
                    "Add" => Some(Self::Add),
                    "Sub" => Some(Self::Sub),
                    "Mul" => Some(Self::Mul),
                    "Div" => Some(Self::Div),
                    "AddAssign" => Some(Self::AddAssign),
                    "SubAssign" => Some(Self::SubAssign),
                    "MulAssign" => Some(Self::MulAssign),
                    "DivAssign" => Some(Self::DivAssign),
                    _ => None,
                })
            },
        )
    }

    pub fn into_token_stream2(self, input: &DeriveInput) -> TokenStream2 {
        let impl_generics_params = input.generics.params.clone();
        let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
        let ident_name = &input.ident;

        match self {
            Self::FromStr => quote! {
                impl #impl_generics ::std::str::FromStr for #ident_name #ty_generics #where_clause
                {
                    type Err = <<Self as ::amplify::Wrapper>::Inner as ::std::str::FromStr>::Err;

                    fn from_str(s: &str) -> Result<Self, Self::Err> {
                        Ok(Self::from_inner(
                            <Self as ::amplify::Wrapper>::Inner::from_str(s)?,
                        ))
                    }
                }
            },
            Self::Octal => quote! {
                impl #impl_generics ::std::fmt::Octal for #ident_name #ty_generics #where_clause
                {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        ::std::fmt::Octal::fmt(self.as_inner(), f)
                    }
                }
            },
            Self::LowerHex => quote! {
                impl #impl_generics ::std::fmt::LowerHex for #ident_name #ty_generics #where_clause
                {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        ::std::fmt::LowerHex::fmt(self.as_inner(), f)
                    }
                }
            },
            Self::UpperHex => quote! {
                impl #impl_generics ::std::fmt::UpperHex for #ident_name #ty_generics #where_clause
                {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        ::std::fmt::UpperHex::fmt(self.as_inner(), f)
                    }
                }
            },
            Self::LowerExp => quote! {
                impl #impl_generics ::std::fmt::LowerExp for #ident_name #ty_generics #where_clause
                {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        ::std::fmt::LowerExp::fmt(self.as_inner(), f)
                    }
                }
            },
            Self::UpperExp => quote! {
                impl #impl_generics ::std::fmt::UpperExp for #ident_name #ty_generics #where_clause
                {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        ::std::fmt::UpperExp::fmt(self.as_inner(), f)
                    }
                }
            },
            Self::Index => quote! {
                impl <_IndexType, #impl_generics_params> ::core::ops::Index<_IndexType> for #ident_name #ty_generics #where_clause
                {
                    type Output = <<Self as ::amplify::Wrapper>::Inner as ::core::ops::Index<_IndexType>>::Output;

                    fn index(&self, index: _IndexType) -> &Self::Output {
                        self.as_inner().index(index)
                    }
                }
            },
            Self::IndexMut => quote! {
                impl <_IndexType, #impl_generics_params> ::core::ops::IndexMut<_IndexType> for #ident_name #ty_generics #where_clause
                {
                    fn index_mut(&mut self, index: _IndexType) -> &mut Self::Output {
                        self.as_inner_mut().index_mut(index)
                    }
                }
            },
            Self::Add => quote! {
                impl #impl_generics ::core::ops::Add for #ident_name #ty_generics #where_clause
                {
                    type Output = Self;

                    fn add(self, rhs: Self) -> Self {
                        Self::from_inner(::core::ops::Add::add(self.into_inner(), rhs.into_inner()))
                    }
                }
            },
            Self::Sub => quote! {
                impl #impl_generics ::core::ops::Sub for #ident_name #ty_generics #where_clause
                {
                    type Output = Self;

                    fn sub(self, rhs: Self) -> Self {
                        Self::from_inner(::core::ops::Sub::sub(self.into_inner(), rhs.into_inner()))
                    }
                }
            },
            Self::Mul => quote! {
                impl #impl_generics ::core::ops::Mul for #ident_name #ty_generics #where_clause
                {
                    type Output = Self;

                    fn mul(self, rhs: Self) -> Self {
                        Self::from_inner(::core::ops::Mul::mul(self.into_inner(), rhs.into_inner()))
                    }
                }
            },
            Self::Div => quote! {
                impl #impl_generics ::core::ops::Div for #ident_name #ty_generics #where_clause
                {
                    type Output = Self;

                    fn div(self, rhs: Self) -> Self {
                        Self::from_inner(::core::ops::Div::div(self.into_inner(), rhs.into_inner()))
                    }
                }
            },
            Self::AddAssign => quote! {
                impl #impl_generics ::core::ops::AddAssign for #ident_name #ty_generics #where_clause
                {
                    fn add_assign(&mut self, rhs: Self) {
                        ::core::ops::AddAssign::add_assign(self.as_inner_mut(), rhs.into_inner())
                    }
                }
            },
            Self::SubAssign => quote! {
                impl #impl_generics ::core::ops::SubAssign for #ident_name #ty_generics #where_clause
                {
                    fn sub_assign(&mut self, rhs: Self) {
                        ::core::ops::SubAssign::sub_assign(self.as_inner_mut(), rhs.into_inner())
                    }
                }
            },
            Self::MulAssign => quote! {
                impl #impl_generics ::core::ops::MulAssign for #ident_name #ty_generics #where_clause
                {
                    fn mul_assign(&mut self, rhs: Self) {
                        ::core::ops::MulAssign::mul_assign(self.as_inner_mut(), rhs.into_inner())
                    }
                }
            },
            Self::DivAssign => quote! {
                impl #impl_generics ::core::ops::DivAssign for #ident_name #ty_generics #where_clause
                {
                    fn div_assign(&mut self, rhs: Self) {
                        ::core::ops::DivAssign::div_assign(self.as_inner_mut(), rhs.into_inner())
                    }
                }
            },
        }
    }
}

pub(crate) fn inner(input: DeriveInput) -> Result<TokenStream2> {
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let ident_name = &input.ident;

    let data = match input.data {
        Data::Struct(ref data) => data,
        Data::Enum(_) => Err(Error::new_spanned(
            &input,
            "Deriving wrapper is not supported in enums",
        ))?,
        //strict_encode_inner_enum(&input, &data),
        Data::Union(_) => Err(Error::new_spanned(
            &input,
            "Deriving wrapper is not supported in unions",
        ))?,
    };

    let mut wrappers = vec![];
    const WRAPPER_DERIVE_ERR: &'static str = "Wrapper attributes must be in a form of type list";
    for attr in input
        .attrs
        .iter()
        .filter(|attr| attr.path.is_ident("wrapper"))
    {
        match attr
            .parse_meta()
            .map_err(|_| attr_err!(attr, WRAPPER_DERIVE_ERR))?
        {
            Meta::List(MetaList { nested, .. }) => {
                for meta in nested {
                    match meta {
                        NestedMeta::Meta(Meta::Path(path)) => {
                            wrappers.push(
                                WrapperDerives::from_path(&path)?
                                    .ok_or(attr_err!(path, "Unrecognized wrapper parameter"))?,
                            );
                        }
                        _ => Err(attr_err!(meta, WRAPPER_DERIVE_ERR))?,
                    }
                }
            }
            _ => Err(attr_err!(attr, WRAPPER_DERIVE_ERR))?,
        }
    }

    let field;
    let mut from;
    match data.fields {
        Fields::Named(ref fields) => {
            let mut source = None;
            from = fields.named[0].ty.clone();
            for field in &fields.named {
                for attr in &field.attrs {
                    if attr.path.is_ident("wrap") {
                        if source.is_some() {
                            Err(Error::new_spanned(
                                attr,
                                "Only a single field may be wrapped",
                            ))?;
                        }
                        source = Some(field.ident.clone().expect("we know it's named"));
                        from = field.ty.clone();
                    }
                }
            }
            if source.is_none() && fields.named.len() > 1 {
                Err(Error::new_spanned(
                    fields,
                    "When the structure has multiple fields you must point out \
                     the one you will wrap by using `#[wrap]` attribute",
                ))?
            }
            let source =
                source.unwrap_or(fields.named[0].ident.clone().expect("we know it's named"));
            field = quote! { #source };
        }
        Fields::Unnamed(ref fields) => {
            let mut source = None;
            from = fields.unnamed[0].ty.clone();
            for (index, field) in fields.unnamed.iter().enumerate() {
                for attr in &field.attrs {
                    if attr.path.is_ident("wrap") {
                        if source.is_some() {
                            Err(Error::new_spanned(
                                attr,
                                "Only a single field may be wrapped",
                            ))?;
                        }
                        let i = Index::from(index);
                        source = Some(quote! { #i });
                        from = field.ty.clone();
                    }
                }
            }
            if source.is_none() && fields.unnamed.len() > 1 {
                Err(Error::new_spanned(
                    fields,
                    "When the structure has multiple fields you must point out \
                     the one you will wrap by using `#[wrap]` attribute",
                ))?
            }
            field = source.unwrap_or(quote! { 0 });
        }
        Fields::Unit => {
            return Err(Error::new_spanned(
                &input,
                "Deriving wrapper is meaningless for unit structs",
            ))
        }
    };

    let wrapper_derive = wrappers.iter().map(|w| w.into_token_stream2(&input));

    Ok(quote! {
        impl #impl_generics ::amplify::Wrapper for #ident_name #ty_generics #where_clause {
            type Inner = #from;

            #[inline]
            fn from_inner(inner: Self::Inner) -> Self {
                Self::from(inner)
            }

            #[inline]
            fn as_inner(&self) -> &Self::Inner {
                &self.#field
            }

            #[inline]
            fn as_inner_mut(&mut self) -> &mut Self::Inner {
                &mut self.#field
            }

            #[inline]
            fn into_inner(self) -> Self::Inner {
                self.#field
            }
        }

        impl #impl_generics ::core::convert::AsRef<<#ident_name #impl_generics as ::amplify::Wrapper>::Inner> for #ident_name #ty_generics #where_clause {
            #[inline]
            fn as_ref(&self) -> &<Self as ::amplify::Wrapper>::Inner {
                self.as_inner()
            }
        }

        impl #impl_generics ::core::convert::AsMut<<#ident_name #impl_generics as ::amplify::Wrapper>::Inner> for #ident_name #ty_generics #where_clause {
            #[inline]
            fn as_mut(&mut self) -> &mut <Self as ::amplify::Wrapper>::Inner {
                self.as_inner_mut()
            }
        }

        impl #impl_generics ::core::borrow::Borrow<<#ident_name #impl_generics as ::amplify::Wrapper>::Inner> for #ident_name #ty_generics #where_clause {
            #[inline]
            fn borrow(&self) -> &<Self as ::amplify::Wrapper>::Inner {
                self.as_inner()
            }
        }

        impl #impl_generics ::core::borrow::BorrowMut<<#ident_name #impl_generics as ::amplify::Wrapper>::Inner> for #ident_name #ty_generics #where_clause {
            #[inline]
            fn borrow_mut(&mut self) -> &mut <Self as ::amplify::Wrapper>::Inner {
                self.as_inner_mut()
            }
        }

        impl #impl_generics ::core::ops::Deref for #ident_name #ty_generics #where_clause {
            type Target = <Self as ::amplify::Wrapper>::Inner;
            #[inline]
            fn deref(&self) -> &Self::Target {
                self.as_inner()
            }
        }

        impl #impl_generics ::core::ops::DerefMut for #ident_name #ty_generics #where_clause {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.as_inner_mut()
            }
        }

        #( #wrapper_derive )*
    })
}
