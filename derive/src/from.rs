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

use syn::export::{Span, TokenStream2};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    Attribute, Data, DataEnum, DataStruct, DataUnion, DeriveInput, Error,
    Field, Fields, FieldsNamed, FieldsUnnamed, Ident, Path, Result, Type,
    TypePath,
};

const NAME: &'static str = "from";
const EXAMPLE: &'static str = r#"#[from(::std::fmt::Error)]"#;

macro_rules! err {
    ( $span:expr, $msg:literal ) => {
        Err(attr_err!($span, NAME, $msg, EXAMPLE))?
    };
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum InstructionEntity {
    Default,
    Unit {
        variant: Option<Ident>,
    },
    Named {
        variant: Option<Ident>,
        field: Ident,
    },
    Unnamed {
        variant: Option<Ident>,
        index: usize,
        total: usize,
    },
}

impl InstructionEntity {
    pub fn with_fields(
        fields: &Fields,
        variant: Option<Ident>,
    ) -> Result<Self> {
        let res = match (
            fields.len(),
            variant.is_some(),
            fields,
            fields.iter().next(),
        ) {
            (0, true, ..) => InstructionEntity::Unit { variant },
            (1, _, Fields::Named(_), Some(Field { ident: Some(i), .. })) => {
                InstructionEntity::Named {
                    variant,
                    field: i.clone(),
                }
            }
            (1, _, Fields::Named(_), ..) => unreachable!(),
            (1, _, Fields::Unnamed(_), ..) => InstructionEntity::Unnamed {
                variant,
                index: 0,
                total: 1,
            },
            (_, false, ..) => InstructionEntity::Default,
            (_, true, ..) => err!(
                fields.span(),
                "allowed only for enum variants with no or \
                 a single field"
            ),
        };
        Ok(res)
    }

    pub fn with_field(
        index: usize,
        total: usize,
        field: &Field,
        variant: Option<Ident>,
    ) -> Self {
        if let Some(ref ident) = field.ident {
            InstructionEntity::Named {
                variant,
                field: ident.clone(),
            }
        } else {
            InstructionEntity::Unnamed {
                variant,
                index,
                total,
            }
        }
    }

    pub fn into_token_stream2(self) -> TokenStream2 {
        match self {
            InstructionEntity::Default => quote! {
                Self::default()
            },
            InstructionEntity::Unit { variant } => {
                let var = variant.map_or(quote! {}, |v| quote! {:: #v});
                quote! { Self #var }
            }
            InstructionEntity::Named {
                variant: None,
                field,
            } => {
                quote! {
                    Self { #field: v.into(), ..Default::default() }
                }
            }
            InstructionEntity::Named {
                variant: Some(var),
                field,
            } => {
                quote! {
                    Self :: #var { #field: v.into() }
                }
            }
            InstructionEntity::Unnamed {
                variant,
                index,
                total,
            } => {
                let var = variant.map_or(quote! {}, |v| quote! {:: #v});
                let prefix =
                    (0..index).fold(TokenStream2::new(), |mut stream, _| {
                        stream.extend(quote! {Default::default(),});
                        stream
                    });
                let suffix = ((index + 1)..total).fold(
                    TokenStream2::new(),
                    |mut stream, _| {
                        stream.extend(quote! {Default::default(),});
                        stream
                    },
                );
                quote! {
                    Self #var ( #prefix v.into(), #suffix )
                }
            }
        }
    }
}

#[derive(Clone)]
struct InstructionEntry(pub Type, pub InstructionEntity);

impl PartialEq for InstructionEntry {
    // Ugly way, but with current `syn` version no other way is possible
    fn eq(&self, other: &Self) -> bool {
        let l = &self.0;
        let r = &other.0;
        let a = quote! { #l };
        let b = quote! { #r };
        format!("{}", a) == format!("{}", b)
    }
}

impl InstructionEntry {
    pub fn with_type(ty: &Type, entity: &InstructionEntity) -> Self {
        Self(ty.clone(), entity.clone())
    }

    pub fn with_path(path: &Path, entity: &InstructionEntity) -> Self {
        Self(
            Type::Path(TypePath {
                path: path.clone(),
                qself: None,
            }),
            entity.clone(),
        )
    }

    pub fn parse(
        fields: &Fields,
        attrs: &Vec<Attribute>,
        entity: InstructionEntity,
    ) -> Result<Vec<InstructionEntry>> {
        let mut list = Vec::<InstructionEntry>::new();
        for attr in attrs.iter().filter(|attr| attr.path.is_ident(NAME)) {
            // #[from]
            if attr.tokens.is_empty() {
                match (fields.len(), fields.iter().next()) {
                    (1, Some(field)) => list
                        .push(InstructionEntry::with_type(&field.ty, &entity)),
                    _ => err!(
                        attr,
                        "empty attribute is allowed only for entities \
                         with a single field; for multi-field entities \
                         specify the attribute right ahead of the target field"
                    ),
                }
            } else {
                list.push(InstructionEntry::with_path(
                    &attr.parse_args()?,
                    &entity,
                ));
            }
        }
        Ok(list)
    }
}

#[derive(Default)]
struct InstructionTable(Vec<InstructionEntry>);

impl InstructionTable {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn parse(
        &mut self,
        fields: &Fields,
        attrs: &Vec<Attribute>,
        variant: Option<Ident>,
    ) -> Result<&Self> {
        self.extend(InstructionEntry::parse(
            &fields,
            &attrs,
            InstructionEntity::with_fields(fields, variant.clone())?,
        )?)?;
        for (index, field) in fields.iter().enumerate() {
            let mut punctuated = Punctuated::new();
            punctuated.push_value(field.clone());
            self.extend(InstructionEntry::parse(
                &field.ident.as_ref().map_or(
                    Fields::Unnamed(FieldsUnnamed {
                        paren_token: Default::default(),
                        unnamed: punctuated.clone(),
                    }),
                    |_| {
                        Fields::Named(FieldsNamed {
                            brace_token: Default::default(),
                            named: punctuated,
                        })
                    },
                ),
                &field.attrs,
                InstructionEntity::with_field(
                    index,
                    fields.len(),
                    field,
                    variant.clone(),
                ),
            )?)?;
        }
        Ok(self)
    }

    fn extend<T>(&mut self, list: T) -> Result<usize>
    where
        T: IntoIterator<Item = InstructionEntry>,
    {
        let mut count = 0;
        for entry in list {
            self.0.iter().find(|e| *e == &entry).map_or(Ok(()), |_| {
                Err(Error::new(
                    Span::call_site(),
                    format!(
                        "Attribute `#[{}]`: repeated use of type `{}`",
                        NAME,
                        quote! {ty}
                    ),
                ))
            })?;
            self.0.push(entry);
            count += 1;
        }
        Ok(count)
    }

    pub fn into_token_stream2(self, input: &DeriveInput) -> TokenStream2 {
        let (impl_generics, ty_generics, where_clause) =
            input.generics.split_for_impl();
        let ident_name = &input.ident;

        self.0.into_iter().fold(TokenStream2::new(), |mut stream, InstructionEntry(from, entity)| {
            let convert = entity.into_token_stream2();
            stream.extend(quote! {
                impl #impl_generics ::std::convert::From<#from> for #ident_name #ty_generics #where_clause {
                    fn from(v: #from) -> Self {
                        #convert
                    }
                }
            });
            stream
        })
    }
}

pub(crate) fn inner(input: DeriveInput) -> Result<TokenStream2> {
    match input.data {
        Data::Struct(ref data) => inner_struct(&input, data),
        Data::Enum(ref data) => inner_enum(&input, data),
        Data::Union(ref data) => inner_union(&input, data),
    }
    .map(|stream| {
        println!("{}", stream);
        stream
    })
}

fn inner_struct(
    input: &DeriveInput,
    data: &DataStruct,
) -> Result<TokenStream2> {
    let mut instructions = InstructionTable::new();
    instructions.parse(&data.fields, &input.attrs, None)?;
    Ok(instructions.into_token_stream2(input))
}

fn inner_enum(input: &DeriveInput, data: &DataEnum) -> Result<TokenStream2> {
    // Do not let top-level `from` on enums
    input
        .attrs
        .iter()
        .find(|attr| attr.path.is_ident(NAME))
        .map_or(Ok(()), |a| {
            Err(attr_err!(
                a,
                "top-level attribute is not allowed, use it for specific fields or variants"
            ))
        })?;

    let mut instructions = InstructionTable::new();
    for v in &data.variants {
        instructions.parse(&v.fields, &v.attrs, Some(v.ident.clone()))?;
    }
    Ok(instructions.into_token_stream2(input))
}

fn inner_union(input: &DeriveInput, data: &DataUnion) -> Result<TokenStream2> {
    let mut instructions = InstructionTable::new();
    instructions.parse(
        &Fields::Named(data.fields.clone()),
        &input.attrs,
        None,
    )?;
    Ok(instructions.into_token_stream2(input))
}
