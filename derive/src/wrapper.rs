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

use proc_macro2::{TokenStream as TokenStream2};
use syn::{
    DeriveInput, Result, Data, Error, Fields, Index, Meta, MetaList, Path, NestedMeta,
    spanned::Spanned, Type,
};

use crate::util::get_amplify_crate;

const NAME: &str = "wrapper";
const EXAMPLE: &str = r#"#[wrapper(LowerHex, Add)]"#;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Wrapper {
    // Formatting
    FromStr,
    Display,
    Debug,
    Octal,
    LowerHex,
    UpperHex,
    LowerExp,
    UpperExp,
    // References
    Deref,
    BorrowSlice,
    // Indexes
    Index,
    IndexRange,
    IndexFull,
    IndexFrom,
    IndexTo,
    IndexInclusive,
    IndexToInclusive,
    // Arithmetics
    Neg,
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    // Booleans
    Not,
    Shl,
    Shr,
    BitAnd,
    BitOr,
    BitXor,
    // Group operations
    Hex,
    Exp,
    NumberFmt,
    RangeOps,
    MathOps,
    BoolOps,
    BitOps,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum WrapperMut {
    // References
    DerefMut,
    BorrowSliceMut,
    // Indexes
    IndexMut,
    IndexRangeMut,
    IndexFullMut,
    IndexFromMut,
    IndexToMut,
    IndexInclusiveMut,
    IndexToInclusiveMut,
    // Arithmetics
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    RemAssign,
    ShlAssign,
    ShrAssign,
    // Booleans
    BitAndAssign,
    BitOrAssign,
    BitXorAssign,
    // Group operations
    RangeMut,
    MathAssign,
    BoolAssign,
    BitAssign,
}

pub trait FromPath: Sized + Copy {
    const IDENT: &'static str;
    const DERIVE: &'static str;
    fn from_path(path: &Path) -> Result<Option<Self>>;
    fn populate(self, list: &mut Vec<Self>);
}

impl FromPath for Wrapper {
    const IDENT: &'static str = "wrapper";
    const DERIVE: &'static str = "Wrapper";

    fn from_path(path: &Path) -> Result<Option<Self>> {
        path.segments.first().map_or(
            Err(attr_err!(
                path.span(),
                NAME,
                "must contain at least one identifier",
                EXAMPLE
            )),
            |segment| {
                Ok(match segment.ident.to_string().as_str() {
                    "FromStr" => Some(Wrapper::FromStr),
                    "Display" => Some(Wrapper::Display),
                    "Debug" => Some(Wrapper::Debug),
                    "Octal" => Some(Wrapper::Octal),
                    "LowerHex" => Some(Wrapper::LowerHex),
                    "UpperHex" => Some(Wrapper::UpperHex),
                    "LowerExp" => Some(Wrapper::LowerExp),
                    "UpperExp" => Some(Wrapper::UpperExp),
                    "Deref" => Some(Wrapper::Deref),
                    "BorrowSlice" => Some(Wrapper::BorrowSlice),
                    "Index" => Some(Wrapper::Index),
                    "IndexRange" => Some(Wrapper::IndexRange),
                    "IndexFull" => Some(Wrapper::IndexFull),
                    "IndexFrom" => Some(Wrapper::IndexFrom),
                    "IndexTo" => Some(Wrapper::IndexTo),
                    "IndexInclusive" => Some(Wrapper::IndexInclusive),
                    "IndexToInclusive" => Some(Wrapper::IndexToInclusive),
                    "Add" => Some(Wrapper::Add),
                    "Neg" => Some(Wrapper::Neg),
                    "Not" => Some(Wrapper::Not),
                    "Sub" => Some(Wrapper::Sub),
                    "Mul" => Some(Wrapper::Mul),
                    "Div" => Some(Wrapper::Div),
                    "Rem" => Some(Wrapper::Rem),
                    "Shl" => Some(Wrapper::Shl),
                    "Shr" => Some(Wrapper::Shr),
                    "BitAnd" => Some(Wrapper::BitAnd),
                    "BitOr" => Some(Wrapper::BitOr),
                    "BitXor" => Some(Wrapper::BitXor),

                    "Hex" => Some(Wrapper::Hex),
                    "Exp" => Some(Wrapper::Exp),
                    "NumberFmt" => Some(Wrapper::NumberFmt),
                    "RangeOps" => Some(Wrapper::RangeOps),
                    "MathOps" => Some(Wrapper::MathOps),
                    "BoolOps" => Some(Wrapper::BoolOps),
                    "BitOps" => Some(Wrapper::BitOps),
                    _ => None,
                })
            },
        )
    }

    fn populate(self, list: &mut Vec<Self>) {
        let ext = match self {
            Wrapper::Hex => &[Wrapper::LowerHex, Wrapper::UpperHex] as &[_],
            Wrapper::Exp => &[Wrapper::LowerExp, Wrapper::UpperExp] as &[_],
            Wrapper::NumberFmt => &[
                Wrapper::LowerHex,
                Wrapper::UpperHex,
                Wrapper::LowerExp,
                Wrapper::UpperExp,
                Wrapper::Octal,
            ] as &[_],
            Wrapper::RangeOps => &[
                Wrapper::IndexRange,
                Wrapper::IndexFrom,
                Wrapper::IndexTo,
                Wrapper::IndexInclusive,
                Wrapper::IndexToInclusive,
                Wrapper::IndexFull,
            ] as &[_],
            Wrapper::MathOps => &[
                Wrapper::Neg,
                Wrapper::Add,
                Wrapper::Sub,
                Wrapper::Mul,
                Wrapper::Div,
                Wrapper::Rem,
            ] as &[_],
            Wrapper::BoolOps => &[
                Wrapper::Not,
                Wrapper::BitAnd,
                Wrapper::BitOr,
                Wrapper::BitXor,
            ] as &[_],
            Wrapper::BitOps => &[
                Wrapper::Not,
                Wrapper::BitAnd,
                Wrapper::BitOr,
                Wrapper::BitXor,
                Wrapper::Shl,
                Wrapper::Shr,
            ] as &[_],
            x => {
                list.push(x);
                &[] as &[_]
            }
        };
        list.extend(ext);
    }
}

impl Wrapper {
    pub fn into_token_stream2(self, input: &DeriveInput) -> TokenStream2 {
        let impl_generics_params = input.generics.params.clone();
        let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
        let ident_name = &input.ident;
        let amplify_crate = get_amplify_crate(input);

        match self {
            Wrapper::FromStr => quote! {
                impl #impl_generics ::core::str::FromStr for #ident_name #ty_generics #where_clause
                {
                    type Err = <<Self as #amplify_crate::Wrapper>::Inner as ::core::str::FromStr>::Err;

                    #[inline]
                    fn from_str(s: &str) -> Result<Self, Self::Err> {
                        use ::core::str::FromStr;
                        use #amplify_crate::Wrapper;
                        Ok(Wrapper::from_inner(
                            <Self as #amplify_crate::Wrapper>::Inner::from_str(s)?,
                        ))
                    }
                }
            },
            Wrapper::Display => quote! {
                impl #impl_generics ::core::fmt::Display for #ident_name #ty_generics #where_clause
                {
                    #[inline]
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        use #amplify_crate::Wrapper;
                        ::core::fmt::Display::fmt(Wrapper::as_inner(self), f)
                    }
                }
            },
            Wrapper::Debug => quote! {
                impl #impl_generics ::core::fmt::Debug for #ident_name #ty_generics #where_clause
                {
                    #[inline]
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        use #amplify_crate::Wrapper;
                        ::core::fmt::Debug::fmt(Wrapper::as_inner(self), f)
                    }
                }
            },
            Wrapper::Octal => quote! {
                impl #impl_generics ::core::fmt::Octal for #ident_name #ty_generics #where_clause
                {
                    #[inline]
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        use #amplify_crate::Wrapper;
                        ::core::fmt::Octal::fmt(Wrapper::as_inner(self), f)
                    }
                }
            },
            Wrapper::LowerHex => quote! {
                impl #impl_generics ::core::fmt::LowerHex for #ident_name #ty_generics #where_clause
                {
                    #[inline]
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        use #amplify_crate::Wrapper;
                        ::core::fmt::LowerHex::fmt(Wrapper::as_inner(self), f)
                    }
                }
            },
            Wrapper::UpperHex => quote! {
                impl #impl_generics ::core::fmt::UpperHex for #ident_name #ty_generics #where_clause
                {
                    #[inline]
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        use #amplify_crate::Wrapper;
                        ::core::fmt::UpperHex::fmt(Wrapper::as_inner(self), f)
                    }
                }
            },
            Wrapper::LowerExp => quote! {
                impl #impl_generics ::core::fmt::LowerExp for #ident_name #ty_generics #where_clause
                {
                    #[inline]
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        use #amplify_crate::Wrapper;
                        ::core::fmt::LowerExp::fmt(Wrapper::as_inner(self), f)
                    }
                }
            },
            Wrapper::UpperExp => quote! {
                impl #impl_generics ::core::fmt::UpperExp for #ident_name #ty_generics #where_clause
                {
                    #[inline]
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        use #amplify_crate::Wrapper;
                        ::core::fmt::UpperExp::fmt(Wrapper::as_inner(self), f)
                    }
                }
            },
            Wrapper::Deref => quote! {
                impl #impl_generics ::core::ops::Deref for #ident_name #ty_generics #where_clause
                {
                    type Target = <Self as #amplify_crate::Wrapper>::Inner;
                    #[inline]
                    fn deref(&self) -> &Self::Target {
                        use #amplify_crate::Wrapper;
                        Wrapper::as_inner(self)
                    }
                }
            },
            Wrapper::BorrowSlice => quote! {
                impl #impl_generics ::core::borrow::Borrow<[u8]> for #ident_name #ty_generics #where_clause
                {
                    #[inline]
                    fn borrow(&self) -> &[u8] {
                        use #amplify_crate::Wrapper;
                        ::core::borrow::Borrow::<[u8]>::borrow(Wrapper::as_inner(self))
                    }
                }
            },
            Wrapper::Index => {
                let where_clause = match where_clause {
                    None => quote! { where },
                    Some(_) => quote! { #where_clause },
                };
                quote! {
                    impl <#impl_generics_params> ::core::ops::Index<usize> for #ident_name #ty_generics #where_clause
                    {
                        type Output = <<Self as #amplify_crate::Wrapper>::Inner as ::core::ops::Index<usize>>::Output;

                        #[inline]
                        fn index(&self, index: usize) -> &Self::Output {
                            use #amplify_crate::Wrapper;
                            Wrapper::as_inner(self).index(index)
                        }
                    }
                }
            }
            Wrapper::IndexRange => {
                quote! {
                    impl <#impl_generics_params> ::core::ops::Index<::core::ops::Range<usize>> for #ident_name #ty_generics #where_clause
                    {
                        type Output = <<Self as #amplify_crate::Wrapper>::Inner as ::core::ops::Index<::core::ops::Range<usize>>>::Output;

                        #[inline]
                        fn index(&self, index: ::core::ops::Range<usize>) -> &Self::Output {
                            use #amplify_crate::Wrapper;
                            Wrapper::as_inner(self).index(index)
                        }
                    }
                }
            }
            Wrapper::IndexFrom => {
                quote! {
                    impl <#impl_generics_params> ::core::ops::Index<::core::ops::RangeFrom<usize>> for #ident_name #ty_generics #where_clause
                    {
                        type Output = <<Self as #amplify_crate::Wrapper>::Inner as ::core::ops::Index<::core::ops::RangeFrom<usize>>>::Output;

                        #[inline]
                        fn index(&self, index: ::core::ops::RangeFrom<usize>) -> &Self::Output {
                            use #amplify_crate::Wrapper;
                            Wrapper::as_inner(self).index(index)
                        }
                    }
                }
            }
            Wrapper::IndexTo => {
                quote! {
                    impl <#impl_generics_params> ::core::ops::Index<::core::ops::RangeTo<usize>> for #ident_name #ty_generics #where_clause
                    {
                        type Output = <<Self as #amplify_crate::Wrapper>::Inner as ::core::ops::Index<::core::ops::RangeTo<usize>>>::Output;

                        #[inline]
                        fn index(&self, index: ::core::ops::RangeTo<usize>) -> &Self::Output {
                            use #amplify_crate::Wrapper;
                            Wrapper::as_inner(self).index(index)
                        }
                    }
                }
            }
            Wrapper::IndexInclusive => {
                quote! {
                    impl <#impl_generics_params> ::core::ops::Index<::core::ops::RangeInclusive<usize>> for #ident_name #ty_generics #where_clause
                    {
                        type Output = <<Self as #amplify_crate::Wrapper>::Inner as ::core::ops::Index<::core::ops::RangeInclusive<usize>>>::Output;

                        #[inline]
                        fn index(&self, index: ::core::ops::RangeInclusive<usize>) -> &Self::Output {
                            use #amplify_crate::Wrapper;
                            Wrapper::as_inner(self).index(index)
                        }
                    }
                }
            }
            Wrapper::IndexToInclusive => {
                quote! {
                    impl <#impl_generics_params> ::core::ops::Index<::core::ops::RangeToInclusive<usize>> for #ident_name #ty_generics #where_clause
                    {
                        type Output = <<Self as #amplify_crate::Wrapper>::Inner as ::core::ops::Index<::core::ops::RangeInclusive<usize>>>::Output;

                        #[inline]
                        fn index(&self, index: ::core::ops::RangeToInclusive<usize>) -> &Self::Output {
                            use #amplify_crate::Wrapper;
                            Wrapper::as_inner(self).index(index)
                        }
                    }
                }
            }
            Wrapper::IndexFull => {
                quote! {
                    impl <#impl_generics_params> ::core::ops::Index<::core::ops::RangeFull> for #ident_name #ty_generics #where_clause
                    {
                        type Output = <<Self as #amplify_crate::Wrapper>::Inner as ::core::ops::Index<::core::ops::RangeFull>>::Output;

                        #[inline]
                        fn index(&self, index: ::core::ops::RangeFull) -> &Self::Output {
                            use #amplify_crate::Wrapper;
                            Wrapper::as_inner(self).index(index)
                        }
                    }
                }
            }
            Wrapper::Neg => quote! {
                impl #impl_generics ::core::ops::Neg for #ident_name #ty_generics #where_clause
                {
                    type Output = Self;

                    #[inline]
                    fn neg(self) -> Self {
                        use #amplify_crate::Wrapper;
                        Wrapper::from_inner(::core::ops::Neg::neg(Wrapper::into_inner(self)))
                    }
                }
            },
            Wrapper::Not => quote! {
                impl #impl_generics ::core::ops::Not for #ident_name #ty_generics #where_clause
                {
                    type Output = Self;

                    #[inline]
                    fn not(self) -> Self {
                        use #amplify_crate::Wrapper;
                        Wrapper::from_inner(::core::ops::Not::not(Wrapper::into_inner(self)))
                    }
                }
            },
            Wrapper::Add => quote! {
                impl #impl_generics ::core::ops::Add for #ident_name #ty_generics #where_clause
                {
                    type Output = Self;

                    #[inline]
                    fn add(self, rhs: Self) -> Self {
                        use #amplify_crate::Wrapper;
                        Wrapper::from_inner(::core::ops::Add::add(Wrapper::into_inner(self), rhs.into_inner()))
                    }
                }
            },
            Wrapper::Sub => quote! {
                impl #impl_generics ::core::ops::Sub for #ident_name #ty_generics #where_clause
                {
                    type Output = Self;

                    #[inline]
                    fn sub(self, rhs: Self) -> Self {
                        use #amplify_crate::Wrapper;
                        Wrapper::from_inner(::core::ops::Sub::sub(Wrapper::into_inner(self), rhs.into_inner()))
                    }
                }
            },
            Wrapper::Mul => quote! {
                impl #impl_generics ::core::ops::Mul for #ident_name #ty_generics #where_clause
                {
                    type Output = Self;

                    #[inline]
                    fn mul(self, rhs: Self) -> Self {
                        use #amplify_crate::Wrapper;
                        Wrapper::from_inner(::core::ops::Mul::mul(Wrapper::into_inner(self), rhs.into_inner()))
                    }
                }
            },
            Wrapper::Div => quote! {
                impl #impl_generics ::core::ops::Div for #ident_name #ty_generics #where_clause
                {
                    type Output = Self;

                    #[inline]
                    fn div(self, rhs: Self) -> Self {
                        use #amplify_crate::Wrapper;
                        Wrapper::from_inner(::core::ops::Div::div(Wrapper::into_inner(self), rhs.into_inner()))
                    }
                }
            },
            Wrapper::Rem => quote! {
                impl #impl_generics ::core::ops::Rem for #ident_name #ty_generics #where_clause
                {
                    type Output = Self;

                    #[inline]
                    fn rem(self, rhs: Self) -> Self {
                        use #amplify_crate::Wrapper;
                        Wrapper::from_inner(::core::ops::Rem::rem(Wrapper::into_inner(self), rhs.into_inner()))
                    }
                }
            },
            Wrapper::Shl => quote! {
                impl #impl_generics ::core::ops::Shl for #ident_name #ty_generics #where_clause
                {
                    type Output = Self;

                    #[inline]
                    fn shl(self, rhs: Self) -> Self {
                        use #amplify_crate::Wrapper;
                        Wrapper::from_inner(::core::ops::Shl::shl(Wrapper::into_inner(self), rhs.into_inner()))
                    }
                }
            },
            Wrapper::Shr => quote! {
                impl #impl_generics ::core::ops::Shr for #ident_name #ty_generics #where_clause
                {
                    type Output = Self;

                    #[inline]
                    fn shr(self, rhs: Self) -> Self {
                        use #amplify_crate::Wrapper;
                        Wrapper::from_inner(::core::ops::Shr::shr(Wrapper::into_inner(self), rhs.into_inner()))
                    }
                }
            },
            Wrapper::BitAnd => quote! {
                impl #impl_generics ::core::ops::BitAnd for #ident_name #ty_generics #where_clause
                {
                    type Output = Self;

                    #[inline]
                    fn bitand(self, rhs: Self) -> Self {
                        use #amplify_crate::Wrapper;
                        Wrapper::from_inner(::core::ops::BitAnd::bitand(Wrapper::into_inner(self), rhs.into_inner()))
                    }
                }
            },
            Wrapper::BitOr => quote! {
                impl #impl_generics ::core::ops::BitOr for #ident_name #ty_generics #where_clause
                {
                    type Output = Self;

                    #[inline]
                    fn bitor(self, rhs: Self) -> Self {
                        use #amplify_crate::Wrapper;
                        Wrapper::from_inner(::core::ops::BitOr::bitor(Wrapper::into_inner(self), rhs.into_inner()))
                    }
                }
            },
            Wrapper::BitXor => quote! {
                impl #impl_generics ::core::ops::BitXor for #ident_name #ty_generics #where_clause
                {
                    type Output = Self;

                    #[inline]
                    fn bitxor(self, rhs: Self) -> Self {
                        use #amplify_crate::Wrapper;
                        Wrapper::from_inner(::core::ops::BitXor::bitxor(Wrapper::into_inner(self), rhs.into_inner()))
                    }
                }
            },
            _ => unreachable!(),
        }
    }
}

impl FromPath for WrapperMut {
    const IDENT: &'static str = "wrapper_mut";
    const DERIVE: &'static str = "WrapperMut";

    fn from_path(path: &Path) -> Result<Option<Self>> {
        path.segments.first().map_or(
            Err(attr_err!(
                path.span(),
                NAME,
                "must contain at least one identifier",
                EXAMPLE
            )),
            |segment| {
                Ok(match segment.ident.to_string().as_str() {
                    "DerefMut" => Some(WrapperMut::DerefMut),
                    "BorrowSliceMut" => Some(WrapperMut::BorrowSliceMut),
                    "IndexMut" => Some(WrapperMut::IndexMut),
                    "IndexRangeMut" => Some(WrapperMut::IndexRangeMut),
                    "IndexFullMut" => Some(WrapperMut::IndexFullMut),
                    "IndexFromMut" => Some(WrapperMut::IndexFromMut),
                    "IndexToMut" => Some(WrapperMut::IndexToMut),
                    "IndexInclusiveMut" => Some(WrapperMut::IndexInclusiveMut),
                    "IndexToInclusiveMut" => Some(WrapperMut::IndexToInclusiveMut),
                    "AddAssign" => Some(WrapperMut::AddAssign),
                    "SubAssign" => Some(WrapperMut::SubAssign),
                    "MulAssign" => Some(WrapperMut::MulAssign),
                    "DivAssign" => Some(WrapperMut::DivAssign),
                    "RemAssign" => Some(WrapperMut::RemAssign),
                    "ShlAssign" => Some(WrapperMut::ShlAssign),
                    "ShrAssign" => Some(WrapperMut::ShrAssign),
                    "BitAndAssign" => Some(WrapperMut::BitAndAssign),
                    "BitOrAssign" => Some(WrapperMut::BitOrAssign),
                    "BitXorAssign" => Some(WrapperMut::BitXorAssign),

                    "RangeMut" => Some(WrapperMut::RangeMut),
                    "MathAssign" => Some(WrapperMut::MathAssign),
                    "BoolAssign" => Some(WrapperMut::BoolAssign),
                    "BitAssign" => Some(WrapperMut::BitAssign),
                    _ => None,
                })
            },
        )
    }

    fn populate(self, list: &mut Vec<Self>) {
        let exp = match self {
            WrapperMut::RangeMut => &[
                WrapperMut::IndexRangeMut,
                WrapperMut::IndexFromMut,
                WrapperMut::IndexToMut,
                WrapperMut::IndexInclusiveMut,
                WrapperMut::IndexToInclusiveMut,
                WrapperMut::IndexFullMut,
            ] as &[_],
            WrapperMut::MathAssign => &[
                WrapperMut::AddAssign,
                WrapperMut::SubAssign,
                WrapperMut::MulAssign,
                WrapperMut::DivAssign,
                WrapperMut::RemAssign,
            ] as &[_],
            WrapperMut::BoolAssign => &[
                WrapperMut::BitAndAssign,
                WrapperMut::BitOrAssign,
                WrapperMut::BitXorAssign,
            ] as &[_],
            WrapperMut::BitAssign => &[
                WrapperMut::BitAndAssign,
                WrapperMut::BitOrAssign,
                WrapperMut::BitXorAssign,
                WrapperMut::ShlAssign,
                WrapperMut::ShrAssign,
            ],
            x => {
                list.push(x);
                &[] as &[_]
            }
        };
        list.extend(exp)
    }
}

impl WrapperMut {
    pub fn into_token_stream2(self, input: &DeriveInput) -> TokenStream2 {
        let impl_generics_params = input.generics.params.clone();
        let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
        let ident_name = &input.ident;
        let amplify_crate = get_amplify_crate(input);

        match self {
            WrapperMut::DerefMut => quote! {
                impl #impl_generics ::core::ops::DerefMut for #ident_name #ty_generics #where_clause
                {
                    #[inline]
                    fn deref_mut(&mut self) -> &mut Self::Target {
                        use #amplify_crate::WrapperMut;
                        WrapperMut::as_inner_mut(self)
                    }
                }
            },
            WrapperMut::BorrowSliceMut => quote! {
                impl #impl_generics ::core::borrow::BorrowMut<[u8]> for #ident_name #ty_generics #where_clause
                {
                    #[inline]
                    fn borrow_mut(&self) -> &mut [u8] {
                        use #amplify_crate::WrapperMut;
                        ::core::borrow::BorrowMut::<[u8]>::borrow_mut(WrapperMut::as_inner_mut(self))
                    }
                }
            },
            WrapperMut::IndexMut => {
                let where_clause = match where_clause {
                    None => quote! { where },
                    Some(_) => quote! { #where_clause },
                };
                quote! {
                    impl <#impl_generics_params> ::core::ops::IndexMut<usize> for #ident_name #ty_generics #where_clause
                    {
                        #[inline]
                        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                            use #amplify_crate::WrapperMut;
                            WrapperMut::as_inner_mut(self).index_mut(index)
                        }
                    }
                }
            }
            WrapperMut::IndexRangeMut => {
                quote! {
                    impl <#impl_generics_params> ::core::ops::IndexMut<::core::ops::Range<usize>> for #ident_name #ty_generics #where_clause
                    {
                        #[inline]
                        fn index_mut(&mut self, index: ::core::ops::Range<usize>) -> &mut Self::Output {
                            use #amplify_crate::WrapperMut;
                            WrapperMut::as_inner_mut(self).index_mut(index)
                        }
                    }
                }
            }
            WrapperMut::IndexFromMut => {
                quote! {
                    impl <#impl_generics_params> ::core::ops::IndexMut<::core::ops::RangeFrom<usize>> for #ident_name #ty_generics #where_clause
                    {
                        #[inline]
                        fn index_mut(&mut self, index: ::core::ops::RangeFrom<usize>) -> &mut Self::Output {
                            use #amplify_crate::WrapperMut;
                            WrapperMut::as_inner_mut(self).index_mut(index)
                        }
                    }
                }
            }
            WrapperMut::IndexToMut => {
                quote! {
                    impl <#impl_generics_params> ::core::ops::IndexMut<::core::ops::RangeTo<usize>> for #ident_name #ty_generics #where_clause
                    {
                        #[inline]
                        fn index_mut(&mut self, index: ::core::ops::RangeTo<usize>) -> &mut Self::Output {
                            use #amplify_crate::WrapperMut;
                            WrapperMut::as_inner_mut(self).index_mut(index)
                        }
                    }
                }
            }
            WrapperMut::IndexInclusiveMut => {
                quote! {
                    impl <#impl_generics_params> ::core::ops::IndexMut<::core::ops::RangeInclusive<usize>> for #ident_name #ty_generics #where_clause
                    {
                        #[inline]
                        fn index_mut(&mut self, index: ::core::ops::RangeInclusive<usize>) -> &mut Self::Output {
                            use #amplify_crate::WrapperMut;
                            WrapperMut::as_inner_mut(self).index_mut(index)
                        }
                    }
                }
            }
            WrapperMut::IndexToInclusiveMut => {
                quote! {
                    impl <#impl_generics_params> ::core::ops::IndexMut<::core::ops::RangeToInclusive<usize>> for #ident_name #ty_generics #where_clause
                    {
                        #[inline]
                        fn index_mut(&mut self, index: ::core::ops::RangeToInclusive<usize>) -> &mut Self::Output {
                            use #amplify_crate::WrapperMut;
                            WrapperMut::as_inner_mut(self).index_mut(index)
                        }
                    }
                }
            }
            WrapperMut::IndexFullMut => {
                quote! {
                    impl <#impl_generics_params> ::core::ops::IndexMut<::core::ops::RangeFull> for #ident_name #ty_generics #where_clause
                    {
                        #[inline]
                        fn index_mut(&mut self, index: ::core::ops::RangeFull) -> &mut Self::Output {
                            use #amplify_crate::WrapperMut;
                            WrapperMut::as_inner_mut(self).index_mut(index)
                        }
                    }
                }
            }
            WrapperMut::AddAssign => quote! {
                impl #impl_generics ::core::ops::AddAssign for #ident_name #ty_generics #where_clause
                {
                    #[inline]
                    fn add_assign(&mut self, rhs: Self) {
                        use #amplify_crate::{Wrapper, WrapperMut};
                        ::core::ops::AddAssign::add_assign(WrapperMut::as_inner_mut(self), rhs.into_inner())
                    }
                }
            },
            WrapperMut::SubAssign => quote! {
                impl #impl_generics ::core::ops::SubAssign for #ident_name #ty_generics #where_clause
                {
                    #[inline]
                    fn sub_assign(&mut self, rhs: Self) {
                        use #amplify_crate::{Wrapper, WrapperMut};
                        ::core::ops::SubAssign::sub_assign(WrapperMut::as_inner_mut(self), rhs.into_inner())
                    }
                }
            },
            WrapperMut::MulAssign => quote! {
                impl #impl_generics ::core::ops::MulAssign for #ident_name #ty_generics #where_clause
                {
                    #[inline]
                    fn mul_assign(&mut self, rhs: Self) {
                        use #amplify_crate::{Wrapper, WrapperMut};
                        ::core::ops::MulAssign::mul_assign(WrapperMut::as_inner_mut(self), rhs.into_inner())
                    }
                }
            },
            WrapperMut::DivAssign => quote! {
                impl #impl_generics ::core::ops::DivAssign for #ident_name #ty_generics #where_clause
                {
                    #[inline]
                    fn div_assign(&mut self, rhs: Self) {
                        use #amplify_crate::{Wrapper, WrapperMut};
                        ::core::ops::DivAssign::div_assign(WrapperMut::as_inner_mut(self), rhs.into_inner())
                    }
                }
            },
            WrapperMut::RemAssign => quote! {4
                impl #impl_generics ::core::ops::RemAssign for #ident_name #ty_generics #where_clause
                {
                    #[inline]
                    fn rem_assign(&mut self, rhs: Self) {
                        use #amplify_crate::{Wrapper, WrapperMut};
                        ::core::ops::RemAssign::rem_assign(WrapperMut::as_inner_mut(self), rhs.into_inner())
                    }
                }
            },
            WrapperMut::ShlAssign => quote! {
                impl #impl_generics ::core::ops::ShlAssign for #ident_name #ty_generics #where_clause
                {
                    #[inline]
                    fn shl_assign(&mut self, rhs: Self) {
                        use #amplify_crate::{Wrapper, WrapperMut};
                        ::core::ops::ShlAssign::shl_assign(WrapperMut::as_inner_mut(self), rhs.into_inner())
                    }
                }
            },
            WrapperMut::ShrAssign => quote! {
                impl #impl_generics ::core::ops::ShrAssign for #ident_name #ty_generics #where_clause
                {
                    #[inline]
                    fn shr_assign(&mut self, rhs: Self) {
                        use #amplify_crate::{Wrapper, WrapperMut};
                        ::core::ops::ShrAssign::shr_assign(WrapperMut::as_inner_mut(self), rhs.into_inner())
                    }
                }
            },
            WrapperMut::BitAndAssign => quote! {
                impl #impl_generics ::core::ops::BitAndAssign for #ident_name #ty_generics #where_clause
                {
                    #[inline]
                    fn bitand_assign(&mut self, rhs: Self) {
                        use #amplify_crate::{Wrapper, WrapperMut};
                        ::core::ops::BitAndAssign::bitand_assign(WrapperMut::as_inner_mut(self), rhs.into_inner())
                    }
                }
            },
            WrapperMut::BitOrAssign => quote! {
                impl #impl_generics ::core::ops::BitOrAssign for #ident_name #ty_generics #where_clause
                {
                    #[inline]
                    fn bitor_assign(&mut self, rhs: Self) {
                        use #amplify_crate::{Wrapper, WrapperMut};
                        ::core::ops::BitOrAssign::bitor_assign(WrapperMut::as_inner_mut(self), rhs.into_inner())
                    }
                }
            },
            WrapperMut::BitXorAssign => quote! {
                impl #impl_generics ::core::ops::BitXorAssign for #ident_name #ty_generics #where_clause
                {
                    #[inline]
                    fn bitxor_assign(&mut self, rhs: Self) {
                        use #amplify_crate::{Wrapper, WrapperMut};
                        ::core::ops::BitXorAssign::bitxor_assign(WrapperMut::as_inner_mut(self), rhs.into_inner())
                    }
                }
            },
            _ => unreachable!(),
        }
    }
}

pub(crate) fn inner(input: DeriveInput) -> Result<TokenStream2> {
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let ident_name = &input.ident;
    let amplify_crate = get_amplify_crate(&input);

    let (field, from) = get_params(&input)?;

    let wrappers = get_wrappers::<Wrapper>(&input)?;
    let wrapper_derive = wrappers.iter().map(|w| w.into_token_stream2(&input));

    Ok(quote! {
        impl #impl_generics #amplify_crate::Wrapper for #ident_name #ty_generics #where_clause {
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
            fn into_inner(self) -> Self::Inner {
                self.#field
            }
        }

        impl #impl_generics ::core::convert::From<#ident_name #ty_generics> for #from #where_clause {
            #[inline]
            fn from(wrapped: #ident_name #ty_generics) -> Self {
                use #amplify_crate::Wrapper;
                Wrapper::into_inner(wrapped)
            }
        }

        impl #impl_generics ::core::convert::AsRef<<#ident_name #impl_generics as #amplify_crate::Wrapper>::Inner> for #ident_name #ty_generics #where_clause {
            #[inline]
            fn as_ref(&self) -> &<Self as #amplify_crate::Wrapper>::Inner {
                use #amplify_crate::Wrapper;
                Wrapper::as_inner(self)
            }
        }

        impl #impl_generics ::core::borrow::Borrow<<#ident_name #impl_generics as #amplify_crate::Wrapper>::Inner> for #ident_name #ty_generics #where_clause {
            #[inline]
            fn borrow(&self) -> &<Self as #amplify_crate::Wrapper>::Inner {
                use #amplify_crate::Wrapper;
                Wrapper::as_inner(self)
            }
        }

        #( #wrapper_derive )*
    })
}

pub(crate) fn inner_mut(input: DeriveInput) -> Result<TokenStream2> {
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let ident_name = &input.ident;
    let amplify_crate = get_amplify_crate(&input);

    let (field, _from) = get_params(&input)?;

    let wrappers = get_wrappers::<WrapperMut>(&input)?;
    let wrapper_derive = wrappers.iter().map(|w| w.into_token_stream2(&input));

    Ok(quote! {
        impl #impl_generics #amplify_crate::WrapperMut for #ident_name #ty_generics #where_clause {
            #[inline]
            fn as_inner_mut(&mut self) -> &mut <Self as #amplify_crate::Wrapper>::Inner {
                &mut self.#field
            }
        }

        impl #impl_generics ::core::convert::AsMut<<#ident_name #impl_generics as #amplify_crate::Wrapper>::Inner> for #ident_name #ty_generics #where_clause {
            #[inline]
            fn as_mut(&mut self) -> &mut <Self as #amplify_crate::Wrapper>::Inner {
                use #amplify_crate::Wrapper;
                WrapperMut::as_inner_mut(self)
            }
        }

        impl #impl_generics ::core::borrow::BorrowMut<<#ident_name #impl_generics as #amplify_crate::Wrapper>::Inner> for #ident_name #ty_generics #where_clause {
            #[inline]
            fn borrow_mut(&mut self) -> &mut <Self as #amplify_crate::Wrapper>::Inner {
                use #amplify_crate::Wrapper;
                WrapperMut::as_inner_mut(self)
            }
        }

        #( #wrapper_derive )*
    })
}

fn get_params(input: &DeriveInput) -> Result<(TokenStream2, Type)> {
    let data = match input.data {
        Data::Struct(ref data) => data,
        Data::Enum(_) => {
            return Err(Error::new_spanned(
                &input,
                "Deriving wrapper is not supported in enums",
            ))
        }
        //strict_encode_inner_enum(&input, &data),
        Data::Union(_) => {
            return Err(Error::new_spanned(
                &input,
                "Deriving wrapper is not supported in unions",
            ))
        }
    };

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
                            return Err(Error::new_spanned(
                                attr,
                                "Only a single field may be wrapped",
                            ));
                        }
                        source = Some(field.ident.clone().expect("we know it's named"));
                        from = field.ty.clone();
                    }
                }
            }
            if source.is_none() && fields.named.len() > 1 {
                return Err(Error::new_spanned(
                    fields,
                    "When the structure has multiple fields you must point out \
                     the one you will wrap by using `#[wrap]` attribute",
                ));
            }
            let source = source
                .unwrap_or_else(|| fields.named[0].ident.clone().expect("we know it's named"));
            field = quote! { #source };
        }
        Fields::Unnamed(ref fields) => {
            let mut source = None;
            from = fields.unnamed[0].ty.clone();
            for (index, field) in fields.unnamed.iter().enumerate() {
                for attr in &field.attrs {
                    if attr.path.is_ident("wrap") {
                        if source.is_some() {
                            return Err(Error::new_spanned(
                                attr,
                                "Only a single field may be wrapped",
                            ));
                        }
                        let i = Index::from(index);
                        source = Some(quote! { #i });
                        from = field.ty.clone();
                    }
                }
            }
            if source.is_none() && fields.unnamed.len() > 1 {
                return Err(Error::new_spanned(
                    fields,
                    "When the structure has multiple fields you must point out \
                     the one you will wrap by using `#[wrap]` attribute",
                ));
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
    Ok((field, from))
}

fn get_wrappers<T: FromPath>(input: &DeriveInput) -> Result<Vec<T>> {
    let mut wrappers = vec![];
    const WRAPPER_DERIVE_ERR: &str = "Wrapper attributes must be in a form of type list";
    for attr in input
        .attrs
        .iter()
        .filter(|attr| attr.path.is_ident(T::IDENT))
    {
        match attr
            .parse_meta()
            .map_err(|_| attr_err!(attr, WRAPPER_DERIVE_ERR))?
        {
            Meta::List(MetaList { nested, .. }) => {
                for meta in nested {
                    match meta {
                        NestedMeta::Meta(Meta::Path(path)) => {
                            T::from_path(&path)?
                                .ok_or_else(|| attr_err!(path, "Unrecognized wrapper parameter"))?
                                .populate(&mut wrappers);
                        }
                        _ => return Err(attr_err!(meta, WRAPPER_DERIVE_ERR)),
                    }
                }
            }
            _ => return Err(attr_err!(attr, WRAPPER_DERIVE_ERR)),
        }
    }
    Ok(wrappers)
}
