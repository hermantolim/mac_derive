use ast::{
    get_all_attrs, get_container_attrs, get_variant_attrs, ContainerAttr,
    VariantAttr,
};
use quote::ToTokens;
use std::{collections::HashSet, convert::TryFrom};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token, Attribute, Data, DeriveInput, Error, Expr, Fields, Generics, Ident,
    Meta, Result, Visibility,
};

/// ```no_run
///
/// use mac_derive::EnumRepr;
///
/// #[derive(EnumRepr)]
/// #[enum_repr(rename_all = "UPPERCASE")]
/// pub enum EnumIdent {
///     Var1,
///     #[enum_repr(from = "FromType")]
///     Var2 = 12,
///     Etc
/// }
/// ```
#[derive(Debug)]
pub struct Enum {
    pub attrs: HashSet<ContainerAttr>,
    pub ident: Ident,
    pub generics: Generics,
    pub variants: Vec<Variant>,
}

#[derive(Debug)]
pub struct Variant {
    pub attrs: HashSet<VariantAttr>,
    pub ident: Ident,
    pub fields: Fields,
}

impl Parse for Enum {
    fn parse(input: ParseStream) -> Result<Self> {
        let derive_input: DeriveInput = input.parse()?;

        split_owned!(derive_input as attrs, ident, generics, data);

        let data = match data {
            Data::Enum(data) => data,
            _ => return Err(Error::new(ident.span(), "only work for enum")),
        };

        let content = get_all_attrs(&attrs)?
            .into_iter()
            .map(|m| match &m {
                Meta::Path(p) => format!(
                    "\n\n{:->100}\nTOKEN: {}\nTREE:{:?}\nVAL: {}\nTYPE: PATH\n",
                    "-",
                    m.to_token_stream(),
                    m.path().get_ident(),
                    p.to_token_stream()
                ),
                Meta::List(l) => format!(
                    "\n\n{:->100}\nTOKEN: {}\nTREE: {:?}\nVAL: {}\nTYPE: LIST\n",
                    "-",
                    m.to_token_stream(),
                    m.path().get_ident(),
                    l.to_token_stream()
                ),
                Meta::NameValue(n) => format!(
                    "\n\n{:->100}\nTOKEN: {}\nTREE: {:?}\nVAL: {}\nTYPE: NVAL\n",
                    "-",
                    m.to_token_stream(),
                    m.path().get_ident(),
                    n.to_token_stream()
                ),
            })
            .collect::<String>();

        use std::{
            fs::{write, File, OpenOptions},
            io::Write,
        };

        let mut file = OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open("/tmp/parse.rs")
            .unwrap();

        file.write_all(content.as_bytes()).unwrap();

        let attrs = get_container_attrs(attrs)?;
        let variants = data
            .variants
            .into_iter()
            .flat_map(Variant::try_from)
            .collect();

        Ok(Self {
            attrs,
            ident,
            generics,
            variants,
        })
    }
}

impl TryFrom<syn::Variant> for Variant {
    type Error = Error;

    fn try_from(var: syn::Variant) -> Result<Variant> {
        split_owned!(var as attrs, ident, fields);
        let attrs = get_variant_attrs(attrs)?;

        Ok(Variant {
            attrs,
            ident,
            fields,
        })
    }
}

/// ```no_run
///
/// #[macro_use]
/// extern crate mac_derive;
///
/// use mac_derive::make_enum;
///
/// make_enum! {
///     pub enum EnumIdent: u16 -> VarIdent {
///         VarIdent(CONST_NAME = 1),
///         Another(ANOTHER_CONST = 2)
///     }
/// }
///
/// fn main() {
///     let ei = EnumIdent::Another as u16;
///     assert_eq!(ei, 2);
/// }
/// ```
#[derive(Debug)]
pub struct EnumMacro {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub enum_token: token::Enum,
    pub ident: Ident,
    pub colon_token: token::Colon,
    pub repr: Ident,
    pub arrow_token: token::RArrow,
    pub default: Ident,
    pub brace_token: token::Brace,
    pub variants: Punctuated<VariantMacro, token::Comma>,
}

#[derive(Debug)]
pub struct VariantMacro {
    pub attrs: Vec<Attribute>,
    pub ident: Ident,
    pub fields: FieldsMacro,
}

#[derive(Debug)]
pub struct FieldsMacro {
    /// wrapping ident and discriminant
    pub paren_token: token::Paren,
    pub ident: Ident,
    pub discriminant: Option<(token::Eq, Expr)>,
}

impl Parse for EnumMacro {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;

        let attrs = input.call(Attribute::parse_outer)?;
        let vis = input.parse()?;
        let enum_token = input.parse()?;
        let ident = input.parse()?;
        let colon_token = input.parse()?;
        let repr = input.parse()?;
        let arrow_token = input.parse()?;
        let default = input.parse()?;
        let brace_token = braced!(content in input);
        let variants = content.parse_terminated(VariantMacro::parse)?;

        Ok(Self {
            attrs,
            vis,
            enum_token,
            ident,
            colon_token,
            repr,
            arrow_token,
            default,
            brace_token,
            variants,
        })
    }
}

impl Parse for VariantMacro {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            attrs: input.call(Attribute::parse_outer)?,
            ident: input.parse()?,
            fields: input.parse()?,
        })
    }
}

impl Parse for FieldsMacro {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let paren_token = parenthesized!(content in input);

        Ok(Self {
            paren_token,
            ident: content.parse()?,
            discriminant: {
                if content.peek(token::Eq) {
                    Some((
                        content.parse::<token::Eq>()?,
                        content.parse::<Expr>()?,
                    ))
                } else {
                    None
                }
            },
        })
    }
}
