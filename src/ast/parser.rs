#![allow(dead_code)]

use ast::{AttributesBucket, ContainerAttr, VariantAttr, ID};
use proc_macro2::{Group, Literal, Span, TokenStream, TokenTree};
use quote::ToTokens;
use std::{collections::HashSet, convert::TryFrom, hash::Hash};
use syn::{
    parse::Parse, spanned::Spanned, Attribute, Error, Lit, LitStr, Meta,
    MetaList, MetaNameValue, NestedMeta, Path, Result, Type,
};

fn parse_lit_str_into_ty(s: &LitStr) -> Result<Type> {
    token_from_str(s.value()).map_err(|e| {
        Error::new(
            s.span(),
            format!(
                "failed to parse literal `{}` into `Type`\ngot: {}",
                s.value(),
                e
            ),
        )
    })
}

fn parse_lit_into_path(s: &LitStr) -> Result<Path> {
    token_from_str(s.value()).map_err(|e| {
        Error::new(
            s.span(),
            format!(
                "failed to parse literal `{}` into `Path`\ngot: {}",
                s.value(),
                e
            ),
        )
    })
}

pub fn token_from_lit<T: Parse>(lit: &Lit) -> Result<T> {
    match lit {
        Lit::Str(s) => token_from_str(s.value()),
        Lit::ByteStr(s) => {
            token_from_str(String::from_utf8_lossy(&s.value()[..]))
        },
        _ => Err(Error::new(lit.span(), "unsupported literal type")),
    }
}

pub fn token_from_str<T>(s: impl AsRef<str>) -> Result<T>
where
    T: Parse,
{
    let tokens = spanned_tokens(s)?;
    syn::parse2(tokens)
}

pub fn spanned_tokens(s: impl AsRef<str>) -> Result<TokenStream> {
    let stream = syn::parse_str(s.as_ref())?;
    Ok(respan_token_stream(stream, s.as_ref().span()))
}

pub fn respan_token_stream(stream: TokenStream, span: Span) -> TokenStream {
    stream
        .into_iter()
        .map(|token| respan_token_tree(token, span))
        .collect()
}

pub fn respan_token_tree(mut token: TokenTree, span: Span) -> TokenTree {
    if let TokenTree::Group(g) = &mut token {
        *g = Group::new(g.delimiter(), respan_token_stream(g.stream(), span));
    }
    token.set_span(span);
    token
}

pub fn lit_from_token<I: ?Sized + ToTokens>(token: &I) -> Result<Lit> {
    let mut lit = Literal::string(token.to_token_stream().to_string().as_str());
    lit.set_span(token.span());
    syn::parse2(lit.to_token_stream())
}

pub fn string_from_lit(lit: &Lit) -> Result<String> {
    match lit {
        Lit::Str(s) => Ok(s.value()),
        _ => Err(Error::new(lit.span(), "unsupported literal")),
    }
}

pub fn get_meta_items2(attr: &Attribute) -> Result<Vec<Meta>> {
    if attr.path != ID {
        return Ok(Vec::new());
    }

    match attr.parse_meta() {
        Ok(Meta::List(meta)) => Ok(meta
            .nested
            .into_iter()
            .filter_map(|nest| match nest {
                NestedMeta::Meta(m) => Some(m),
                _ => None,
            })
            .collect()),
        _ => err!("expected #[enum_repr(...)]"),
    }
}

pub fn get_meta_items(attr: &Attribute, set: &mut HashSet<Meta>) -> Result<()> {
    if attr.path != ID {
        return Ok(());
    }

    match attr.parse_meta() {
        Ok(Meta::List(meta)) => {
            for nest in meta.nested {
                if let NestedMeta::Meta(m) = nest {
                    set.insert(m);
                }
            }

            Ok(())
        },
        _ => err!("expected #[enum_repr(...)]"),
    }
}

pub fn get_all_attrs2(attrs: Vec<Attribute>) -> Result<Vec<Meta>> {
    Ok(attrs
        .iter()
        .flat_map(|attr| get_meta_items2(attr))
        .flatten()
        .collect())
}

pub fn get_all_attrs(attrs: &[Attribute]) -> Result<HashSet<Meta>> {
    let mut set = HashSet::new();
    for attr in attrs {
        get_meta_items(attr, &mut set)?;
    }
    Ok(set)
}

pub fn get_container_attrs<A>(attrs: Vec<Attribute>) -> Result<HashSet<A>>
where
    A: AttributesBucket,
{
    let mut set = HashSet::new();
    for attr in attrs {
        get_attr(attr.parse_meta()?, &mut set)?;
    }
    Ok(set)
}

pub fn get_variant_attrs<A>(attrs: Vec<Attribute>) -> Result<HashSet<A>>
where
    A: AttributesBucket,
{
    let mut set = HashSet::new();
    for attr in attrs {
        get_attr(attr.parse_meta()?, &mut set)?;
    }
    Ok(set)
}

pub fn get_attr<A>(meta: Meta, set: &mut HashSet<A>) -> Result<()>
where
    A: AttributesBucket,
{
    if meta.path() != ID {
        return Ok(());
    }

    match meta {
        Meta::List(list) => {
            for nest in list.nested {
                match nest {
                    NestedMeta::Meta(Meta::NameValue(m)) => {
                        let m = A::try_from(m)?;
                        set.insert(m);
                    },
                    NestedMeta::Meta(Meta::List(m)) => {
                        let m = A::try_from(m)?;
                        set.insert(m);
                    },
                    m => {
                        return err!(m: "unexpected attribute {}!", m.to_token_stream())
                    },
                }
            }

            Ok(())
        },
        Meta::NameValue(m) => {
            let m = A::try_from(m)?;
            set.insert(m);
            Ok(())
        },
        m => err!(
            m: "unexpected attribute {}!", m.to_token_stream()
        ),
    }
}
