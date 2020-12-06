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
    Debug,
    Octal,
    LowerHex,
    UpperHex,
    LowerExp,
    UpperExp,
    Index,
    IndexMut,
    IndexRange,
    IndexFull,
    IndexFrom,
    IndexTo,
    IndexInclusive,
    Neg,
    Not,
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Shl,
    Shr,
    BitAnd,
    BitOr,
    BitXor,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    RemAssign,
    ShlAssign,
    ShrAssign,
    BitAndAssign,
    BitOrAssign,
    BitXorAssign,
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
                    "Debug" => Some(Self::Debug),
                    "Octal" => Some(Self::Octal),
                    "LowerHex" => Some(Self::LowerHex),
                    "UpperHex" => Some(Self::UpperHex),
                    "LowerExp" => Some(Self::LowerExp),
                    "UpperExp" => Some(Self::UpperExp),
                    "Index" => Some(Self::Index),
                    "IndexMut" => Some(Self::IndexMut),
                    "IndexRange" => Some(Self::IndexRange),
                    "IndexFull" => Some(Self::IndexFull),
                    "IndexFrom" => Some(Self::IndexFrom),
                    "IndexTo" => Some(Self::IndexTo),
                    "IndexInclusive" => Some(Self::IndexInclusive),
                    "Add" => Some(Self::Add),
                    "Neg" => Some(Self::Neg),
                    "Not" => Some(Self::Not),
                    "Sub" => Some(Self::Sub),
                    "Mul" => Some(Self::Mul),
                    "Div" => Some(Self::Div),
                    "Rem" => Some(Self::Rem),
                    "Shl" => Some(Self::Shl),
                    "Shr" => Some(Self::Shr),
                    "BitAnd" => Some(Self::BitAnd),
                    "BitOr" => Some(Self::BitOr),
                    "BitXor" => Some(Self::BitXor),
                    "AddAssign" => Some(Self::AddAssign),
                    "SubAssign" => Some(Self::SubAssign),
                    "MulAssign" => Some(Self::MulAssign),
                    "DivAssign" => Some(Self::DivAssign),
                    "RemAssign" => Some(Self::RemAssign),
                    "ShlAssign" => Some(Self::ShlAssign),
                    "ShrAssign" => Some(Self::ShrAssign),
                    "BitAndAssign" => Some(Self::BitAndAssign),
                    "BitOrAssign" => Some(Self::BitOrAssign),
                    "BitXorAssign" => Some(Self::BitXorAssign),
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

                    #[inline]
                    fn from_str(s: &str) -> Result<Self, Self::Err> {
                        use ::std::str::FromStr;
                        Ok(Self::from_inner(
                            <Self as ::amplify::Wrapper>::Inner::from_str(s)?,
                        ))
                    }
                }
            },
            Self::Debug => quote! {
                impl #impl_generics ::std::fmt::Debug for #ident_name #ty_generics #where_clause
                {
                    #[inline]
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        ::std::fmt::Debug::fmt(self.as_inner(), f)
                    }
                }
            },
            Self::Octal => quote! {
                impl #impl_generics ::std::fmt::Octal for #ident_name #ty_generics #where_clause
                {
                    #[inline]
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        ::std::fmt::Octal::fmt(self.as_inner(), f)
                    }
                }
            },
            Self::LowerHex => quote! {
                impl #impl_generics ::std::fmt::LowerHex for #ident_name #ty_generics #where_clause
                {
                    #[inline]
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        use ::amplify::Wrapper;
                        ::std::fmt::LowerHex::fmt(self.as_inner(), f)
                    }
                }
            },
            Self::UpperHex => quote! {
                impl #impl_generics ::std::fmt::UpperHex for #ident_name #ty_generics #where_clause
                {
                    #[inline]
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        use ::amplify::Wrapper;
                        ::std::fmt::UpperHex::fmt(self.as_inner(), f)
                    }
                }
            },
            Self::LowerExp => quote! {
                impl #impl_generics ::std::fmt::LowerExp for #ident_name #ty_generics #where_clause
                {
                    #[inline]
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        use ::amplify::Wrapper;
                        ::std::fmt::LowerExp::fmt(self.as_inner(), f)
                    }
                }
            },
            Self::UpperExp => quote! {
                impl #impl_generics ::std::fmt::UpperExp for #ident_name #ty_generics #where_clause
                {
                    #[inline]
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        use ::amplify::Wrapper;
                        ::std::fmt::UpperExp::fmt(self.as_inner(), f)
                    }
                }
            },
            Self::Index => {
                let where_clause = match where_clause {
                    None => quote! { where },
                    Some(_) => quote! { #where_clause },
                };
                quote! {
                    impl <#impl_generics_params, _IndexType> ::core::ops::Index<_IndexType> for #ident_name #ty_generics #where_clause
                        _IndexType: ::core::slice::SliceIndex<<Self as ::amplify::Wrapper>::Inner>
                    {
                        type Output = <<Self as ::amplify::Wrapper>::Inner as ::core::ops::Index<_IndexType>>::Output;

                        #[inline]
                        fn index(&self, index: _IndexType) -> &Self::Output {
                            use ::amplify::Wrapper;
                            self.as_inner().index(index)
                        }
                    }
                }
            }
            Self::IndexMut => {
                let where_clause = match where_clause {
                    None => quote! { where },
                    Some(_) => quote! { #where_clause },
                };
                quote! {
                    impl <#impl_generics_params, _IndexType> ::core::ops::IndexMut<_IndexType> for #ident_name #ty_generics #where_clause
                        _IndexType: ::core::slice::SliceIndex<<Self as ::amplify::Wrapper>::Inner>
                    {
                        #[inline]
                        fn index_mut(&mut self, index: _IndexType) -> &mut Self::Output {
                            use ::amplify::Wrapper;
                            self.as_inner_mut().index_mut(index)
                        }
                    }
                }
            }
            Self::IndexRange => {
                quote! {
                    impl <#impl_generics_params> ::core::ops::Index<::core::ops::Range<usize>> for #ident_name #ty_generics #where_clause
                    {
                        type Output = <<Self as ::amplify::Wrapper>::Inner as ::core::ops::Index<::core::ops::Range<usize>>>::Output;

                        #[inline]
                        fn index(&self, index: ::core::ops::Range<usize>) -> &Self::Output {
                            use ::amplify::Wrapper;
                            self.as_inner().index(index)
                        }
                    }
                }
            }
            Self::IndexFrom => {
                quote! {
                    impl <#impl_generics_params> ::core::ops::Index<::core::ops::RangeFrom<usize>> for #ident_name #ty_generics #where_clause
                    {
                        type Output = <<Self as ::amplify::Wrapper>::Inner as ::core::ops::Index<::core::ops::RangeFrom<usize>>>::Output;

                        #[inline]
                        fn index(&self, index: ::core::ops::RangeFrom<usize>) -> &Self::Output {
                            use ::amplify::Wrapper;
                            self.as_inner().index(index)
                        }
                    }
                }
            }
            Self::IndexTo => {
                quote! {
                    impl <#impl_generics_params> ::core::ops::Index<::core::ops::RangeTo<usize>> for #ident_name #ty_generics #where_clause
                    {
                        type Output = <<Self as ::amplify::Wrapper>::Inner as ::core::ops::Index<::core::ops::RangeTo<usize>>>::Output;

                        #[inline]
                        fn index(&self, index: ::core::ops::RangeTo<usize>) -> &Self::Output {
                            use ::amplify::Wrapper;
                            self.as_inner().index(index)
                        }
                    }
                }
            }
            Self::IndexInclusive => {
                quote! {
                    impl <#impl_generics_params> ::core::ops::Index<::core::ops::RangeInclusive<usize>> for #ident_name #ty_generics #where_clause
                    {
                        type Output = <<Self as ::amplify::Wrapper>::Inner as ::core::ops::Index<::core::ops::RangeInclusive<usize>>>::Output;

                        #[inline]
                        fn index(&self, index: ::core::ops::RangeInclusive<usize>) -> &Self::Output {
                            use ::amplify::Wrapper;
                            self.as_inner().index(index)
                        }
                    }
                }
            }
            Self::IndexFull => {
                quote! {
                    impl <#impl_generics_params> ::core::ops::Index<::core::ops::RangeFull> for #ident_name #ty_generics #where_clause
                    {
                        type Output = <<Self as ::amplify::Wrapper>::Inner as ::core::ops::Index<::core::ops::RangeFull>>::Output;

                        #[inline]
                        fn index(&self, index: ::core::ops::RangeFull) -> &Self::Output {
                            use ::amplify::Wrapper;
                            self.as_inner().index(index)
                        }
                    }
                }
            }
            Self::Neg => quote! {
                impl #impl_generics ::core::ops::Neg for #ident_name #ty_generics #where_clause
                {
                    type Output = Self;

                    #[inline]
                    fn neg(self) -> Self {
                        use ::amplify::Wrapper;
                        Self::from_inner(::core::ops::Neg::neg(self.into_inner()))
                    }
                }
            },
            Self::Not => quote! {
                impl #impl_generics ::core::ops::Not for #ident_name #ty_generics #where_clause
                {
                    type Output = Self;

                    #[inline]
                    fn not(self) -> Self {
                        use ::amplify::Wrapper;
                        Self::from_inner(::core::ops::Not::not(self.into_inner()))
                    }
                }
            },
            Self::Add => quote! {
                impl #impl_generics ::core::ops::Add for #ident_name #ty_generics #where_clause
                {
                    type Output = Self;

                    #[inline]
                    fn add(self, rhs: Self) -> Self {
                        use ::amplify::Wrapper;
                        Self::from_inner(::core::ops::Add::add(self.into_inner(), rhs.into_inner()))
                    }
                }
            },
            Self::Sub => quote! {
                impl #impl_generics ::core::ops::Sub for #ident_name #ty_generics #where_clause
                {
                    type Output = Self;

                    #[inline]
                    fn sub(self, rhs: Self) -> Self {
                        use ::amplify::Wrapper;
                        Self::from_inner(::core::ops::Sub::sub(self.into_inner(), rhs.into_inner()))
                    }
                }
            },
            Self::Mul => quote! {
                impl #impl_generics ::core::ops::Mul for #ident_name #ty_generics #where_clause
                {
                    type Output = Self;

                    #[inline]
                    fn mul(self, rhs: Self) -> Self {
                        use ::amplify::Wrapper;
                        Self::from_inner(::core::ops::Mul::mul(self.into_inner(), rhs.into_inner()))
                    }
                }
            },
            Self::Div => quote! {
                impl #impl_generics ::core::ops::Div for #ident_name #ty_generics #where_clause
                {
                    type Output = Self;

                    #[inline]
                    fn div(self, rhs: Self) -> Self {
                        use ::amplify::Wrapper;
                        Self::from_inner(::core::ops::Div::div(self.into_inner(), rhs.into_inner()))
                    }
                }
            },
            Self::Rem => quote! {
                impl #impl_generics ::core::ops::Rem for #ident_name #ty_generics #where_clause
                {
                    type Output = Self;

                    #[inline]
                    fn rem(self, rhs: Self) -> Self {
                        use ::amplify::Wrapper;
                        Self::from_inner(::core::ops::Rem::rem(self.into_inner(), rhs.into_inner()))
                    }
                }
            },
            Self::Shl => quote! {
                impl #impl_generics ::core::ops::Shl for #ident_name #ty_generics #where_clause
                {
                    type Output = Self;

                    #[inline]
                    fn shl(self, rhs: Self) -> Self {
                        use ::amplify::Wrapper;
                        Self::from_inner(::core::ops::Shl::shl(self.into_inner(), rhs.into_inner()))
                    }
                }
            },
            Self::Shr => quote! {
                impl #impl_generics ::core::ops::Shr for #ident_name #ty_generics #where_clause
                {
                    type Output = Self;

                    #[inline]
                    fn shr(self, rhs: Self) -> Self {
                        use ::amplify::Wrapper;
                        Self::from_inner(::core::ops::Shr::shr(self.into_inner(), rhs.into_inner()))
                    }
                }
            },
            Self::BitAnd => quote! {
                impl #impl_generics ::core::ops::BitAnd for #ident_name #ty_generics #where_clause
                {
                    type Output = Self;

                    #[inline]
                    fn bitand(self, rhs: Self) -> Self {
                        use ::amplify::Wrapper;
                        Self::from_inner(::core::ops::BitAnd::bitand(self.into_inner(), rhs.into_inner()))
                    }
                }
            },
            Self::BitOr => quote! {
                impl #impl_generics ::core::ops::BitOr for #ident_name #ty_generics #where_clause
                {
                    type Output = Self;

                    #[inline]
                    fn bitor(self, rhs: Self) -> Self {
                        use ::amplify::Wrapper;
                        Self::from_inner(::core::ops::BitOr::bitor(self.into_inner(), rhs.into_inner()))
                    }
                }
            },
            Self::BitXor => quote! {
                impl #impl_generics ::core::ops::BitXor for #ident_name #ty_generics #where_clause
                {
                    type Output = Self;

                    #[inline]
                    fn bitxor(self, rhs: Self) -> Self {
                        use ::amplify::Wrapper;
                        Self::from_inner(::core::ops::BitXor::bitxor(self.into_inner(), rhs.into_inner()))
                    }
                }
            },
            Self::AddAssign => quote! {
                impl #impl_generics ::core::ops::AddAssign for #ident_name #ty_generics #where_clause
                {
                    #[inline]
                    fn add_assign(&mut self, rhs: Self) {
                        use ::amplify::Wrapper;
                        ::core::ops::AddAssign::add_assign(self.as_inner_mut(), rhs.into_inner())
                    }
                }
            },
            Self::SubAssign => quote! {
                impl #impl_generics ::core::ops::SubAssign for #ident_name #ty_generics #where_clause
                {
                    #[inline]
                    fn sub_assign(&mut self, rhs: Self) {
                        use ::amplify::Wrapper;
                        ::core::ops::SubAssign::sub_assign(self.as_inner_mut(), rhs.into_inner())
                    }
                }
            },
            Self::MulAssign => quote! {
                impl #impl_generics ::core::ops::MulAssign for #ident_name #ty_generics #where_clause
                {
                    #[inline]
                    fn mul_assign(&mut self, rhs: Self) {
                        use ::amplify::Wrapper;
                        ::core::ops::MulAssign::mul_assign(self.as_inner_mut(), rhs.into_inner())
                    }
                }
            },
            Self::DivAssign => quote! {
                impl #impl_generics ::core::ops::DivAssign for #ident_name #ty_generics #where_clause
                {
                    #[inline]
                    fn div_assign(&mut self, rhs: Self) {
                        use ::amplify::Wrapper;
                        ::core::ops::DivAssign::div_assign(self.as_inner_mut(), rhs.into_inner())
                    }
                }
            },
            Self::RemAssign => quote! {4
                impl #impl_generics ::core::ops::RemAssign for #ident_name #ty_generics #where_clause
                {
                    #[inline]
                    fn rem_assign(&mut self, rhs: Self) {
                        use ::amplify::Wrapper;
                        ::core::ops::RemAssign::rem_assign(self.as_inner_mut(), rhs.into_inner())
                    }
                }
            },
            Self::ShlAssign => quote! {
                impl #impl_generics ::core::ops::ShlAssign for #ident_name #ty_generics #where_clause
                {
                    #[inline]
                    fn shl_assign(&mut self, rhs: Self) {
                        use ::amplify::Wrapper;
                        ::core::ops::ShlAssign::shl_assign(self.as_inner_mut(), rhs.into_inner())
                    }
                }
            },
            Self::ShrAssign => quote! {
                impl #impl_generics ::core::ops::ShrAssign for #ident_name #ty_generics #where_clause
                {
                    #[inline]
                    fn shr_assign(&mut self, rhs: Self) {
                        use ::amplify::Wrapper;
                        ::core::ops::ShrAssign::shr_assign(self.as_inner_mut(), rhs.into_inner())
                    }
                }
            },
            Self::BitAndAssign => quote! {
                impl #impl_generics ::core::ops::BitAndAssign for #ident_name #ty_generics #where_clause
                {
                    #[inline]
                    fn bitand_assign(&mut self, rhs: Self) {
                        use ::amplify::Wrapper;
                        ::core::ops::BitAndAssign::bitand_assign(self.as_inner_mut(), rhs.into_inner())
                    }
                }
            },
            Self::BitOrAssign => quote! {
                impl #impl_generics ::core::ops::BitOrAssign for #ident_name #ty_generics #where_clause
                {
                    #[inline]
                    fn bitor_assign(&mut self, rhs: Self) {
                        use ::amplify::Wrapper;
                        ::core::ops::BitOrAssign::bitor_assign(self.as_inner_mut(), rhs.into_inner())
                    }
                }
            },
            Self::BitXorAssign => quote! {
                impl #impl_generics ::core::ops::BitXorAssign for #ident_name #ty_generics #where_clause
                {
                    #[inline]
                    fn bitxor_assign(&mut self, rhs: Self) {
                        use ::amplify::Wrapper;
                        ::core::ops::BitXorAssign::bitxor_assign(self.as_inner_mut(), rhs.into_inner())
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
                use ::amplify::Wrapper;
                self.as_inner()
            }
        }

        impl #impl_generics ::core::convert::AsMut<<#ident_name #impl_generics as ::amplify::Wrapper>::Inner> for #ident_name #ty_generics #where_clause {
            #[inline]
            fn as_mut(&mut self) -> &mut <Self as ::amplify::Wrapper>::Inner {
                use ::amplify::Wrapper;
                self.as_inner_mut()
            }
        }

        impl #impl_generics ::core::borrow::Borrow<<#ident_name #impl_generics as ::amplify::Wrapper>::Inner> for #ident_name #ty_generics #where_clause {
            #[inline]
            fn borrow(&self) -> &<Self as ::amplify::Wrapper>::Inner {
                use ::amplify::Wrapper;
                self.as_inner()
            }
        }

        impl #impl_generics ::core::borrow::BorrowMut<<#ident_name #impl_generics as ::amplify::Wrapper>::Inner> for #ident_name #ty_generics #where_clause {
            #[inline]
            fn borrow_mut(&mut self) -> &mut <Self as ::amplify::Wrapper>::Inner {
                use ::amplify::Wrapper;
                self.as_inner_mut()
            }
        }

        impl #impl_generics ::core::ops::Deref for #ident_name #ty_generics #where_clause {
            type Target = <Self as ::amplify::Wrapper>::Inner;
            #[inline]
            fn deref(&self) -> &Self::Target {
                use ::amplify::Wrapper;
                self.as_inner()
            }
        }

        impl #impl_generics ::core::ops::DerefMut for #ident_name #ty_generics #where_clause {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                use ::amplify::Wrapper;
                self.as_inner_mut()
            }
        }

        #( #wrapper_derive )*
    })
}
