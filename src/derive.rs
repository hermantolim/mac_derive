use super::{ast::Enum, util::*};
use proc_macro2::TokenStream;
use syn::Result;

pub fn enum_repr(input: TokenStream) -> Result<TokenStream> {
    let input = syn::parse2::<Enum>(input)?;
    _enum_repr(&input)
}

fn _enum_repr(input: &Enum) -> Result<TokenStream> {
    //split!(input as attrs, ident, generics, variants);
    Ok(wrap_in_const(None, "ENUM_REPR", &input.ident, quote! {}))
}

/*
_match_discrs(input: &DeriveInput) -> Result<TokenStream> {
    let fields = input.variants.iter()
        .filter(|v| !v.
}
*/
