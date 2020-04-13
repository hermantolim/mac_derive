use ast::{lit_from_token, EnumMacro};
use proc_macro2::TokenStream;
use syn::*;

pub fn forward_impl(input: TokenStream) -> Result<TokenStream> {
    dump!(input);

    Ok(quote! { #input })
}

pub fn make_enum(input: TokenStream) -> Result<TokenStream> {
    let input = syn::parse2::<EnumMacro>(input)?;
    split!(input as attrs, vis, ident, repr, default, variants);

    let default = lit_from_token(&default)?;

    let declare_variants = variants.iter().map(|v| {
        split!(v as attrs, ident, fields);

        let disp = lit_from_token(&fields.ident)
            .map(|l| quote! { #[enum_repr(discr = #l)] })
            .unwrap_or(quote! {});

        let (eq, e) = match &fields.discriminant {
            Some((eq, e)) => (Some(eq), Some(e)),
            _ => (None, None),
        };

        quote! {
            #(#attrs)*
             #disp
             #ident #eq #e,
        }
    });

    let output = quote! {
        #(#attrs)*
        #[repr(#repr)]
        #[derive(EnumRepr)]
        #[enum_repr(default = #default)]
        #vis enum #ident {
            #(#declare_variants)*
        }
    };

    Ok(output)
}
