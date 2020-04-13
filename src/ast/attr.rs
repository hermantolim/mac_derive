use ast::{
    get_meta_items, get_meta_items2, token_from_lit, RenameRule, ATTR,
    ATTR_NEST, DEFAULT, DISCR, FROM, INTO, OTHER, RENAME, RENAME_ALL, TRY_FROM,
};
use proc_macro2::{Literal, TokenStream, TokenTree};
use quote::ToTokens;
use std::{
    collections::HashSet, convert::TryFrom, marker::PhantomData, str::FromStr,
};
use syn::{
    export::Hash,
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Attribute, Error, Ident, Lit, LitStr, Meta, MetaList, MetaNameValue,
    NestedMeta, Result,
};

pub trait AttributesBucket
where
    Self: Sized
        + Hash
        + Eq
        + TryFrom<MetaList, Error = Error>
        + TryFrom<MetaNameValue, Error = Error>,
{
}

macro_rules! make_attr_enum {
    (

        pub enum $id:ident {
            $(
                $num:tt: $field:ident( $( $argv:ty ),+ )
            ),+
            $(,)*
        }
    ) => {
        #[allow(dead_code)]
        #[derive(Debug, Eq, PartialEq, Hash)]
        pub enum $id {
            $(
                $field( $( $argv ),+ )
            ),+
        }

        impl AttributesBucket for $id {
        }
    };
}

make_attr_enum! {
    pub enum ContainerAttr {
        0: RenameAll(RenameRule),
        1: Repr(Ident),
        2: Default(Ident),
        3: Trans(PhantomData<bool>),
        4: Other(String),
        5: Attr(String),
        6: From(Ident),
        7: Into(Ident),
        8: TryFrom(Ident)
    }
}

impl TryFrom<MetaList> for ContainerAttr {
    type Error = Error;
    fn try_from(m: MetaList) -> Result<Self> {
        match (&m.path, &m.nested) {
            (id, list) if id == DEFAULT => {
                let lts = list.to_token_stream();
                Ok(ContainerAttr::Default(syn::parse2::<Ident>(lts)?))
            },
            (id, list) if id == FROM => {
                let lts = list.to_token_stream();
                Ok(ContainerAttr::From(syn::parse2::<Ident>(lts)?))
            },
            (id, list) if id == INTO => {
                let lts = list.to_token_stream();
                Ok(ContainerAttr::Into(syn::parse2::<Ident>(lts)?))
            },
            (id, list) if id == TRY_FROM => {
                let lts = list.to_token_stream();
                Ok(ContainerAttr::TryFrom(syn::parse2::<Ident>(lts)?))
            },
            (id, list) if id == ATTR || id == ATTR_NEST || id == OTHER => {
                Ok(ContainerAttr::Other(m.to_token_stream().to_string()))
            },
            _ => {
                err!(m: r##"unexpected container attribute {}"##, m.to_token_stream())
            },
        }
    }
}

impl TryFrom<MetaNameValue> for ContainerAttr {
    type Error = Error;

    fn try_from(m: MetaNameValue) -> Result<Self> {
        match (&m.path, &m.lit) {
            (id, lit) if id == DEFAULT => {
                Ok(ContainerAttr::Default(token_from_lit(lit)?))
            },
            (id, Lit::Str(lit)) if id == RENAME_ALL => {
                Ok(ContainerAttr::RenameAll(RenameRule::from_str(
                    lit.value().as_str(),
                )?))
            },
            (id, _) if (id == DEFAULT || id == RENAME_ALL) => err!(
                r##"expected attribute value {}, expected {} = "str literal""##,
                &m.to_token_stream(),
                &m.path.to_token_stream()
            ),
            (id, _) if id == OTHER => {
                Ok(ContainerAttr::Other(m.to_token_stream().to_string()))
            },
            (id, _) if id == ATTR => {
                Ok(ContainerAttr::Attr(m.to_token_stream().to_string()))
            },
            _ => {
                err!(m: r##"unexpected container attribute {}"##, m.to_token_stream())
            },
        }
    }
}

make_attr_enum! {
    pub enum VariantAttr {
        0: Default(bool),
        1: Discriminant(String),
        2: Rename(RenameRule),
        3: Trans(PhantomData<bool>),
        4: Other(String)
    }
}

pub struct VarAttr<'a> {
    pub display: &'a Ident,
    pub rename: &'a LitStr,
    pub default: bool,
}

impl TryFrom<MetaNameValue> for VariantAttr {
    type Error = Error;

    fn try_from(m: MetaNameValue) -> Result<VariantAttr> {
        match (&m.path, &m.lit) {
            (id, Lit::Str(s)) if id == DISCR => {
                Ok(VariantAttr::Discriminant(s.value()))
            },

            (id, Lit::Str(s)) if id == RENAME => Ok(VariantAttr::Rename(
                RenameRule::from_str(s.value().as_str())?,
            )),
            (id, _) if (id == RENAME || id == DISCR) => err!(
                m: r##"unexpected attribute value {}, expected {} = "str literal""##,
                &m.to_token_stream(),
                &m.path.to_token_stream()
            ),
            (id, _) if id == OTHER => {
                Ok(VariantAttr::Other(m.to_token_stream().to_string()))
            },
            _ => err!(m: "unexpected attribute {:?}", &m.to_token_stream()),
        }
    }
}

impl TryFrom<MetaList> for VariantAttr {
    type Error = Error;

    fn try_from(m: MetaList) -> Result<Self> {
        match (&m.path, &m.nested) {
            (id, list) if id == OTHER => Ok(VariantAttr::Trans(PhantomData)),
            _ => err!(m: "unexpected attribute {:?}", m.to_token_stream()),
        }
    }
}
