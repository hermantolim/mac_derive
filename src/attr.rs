use syn::*;
use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream};

#[derive(Debug)]
struct Args {
    attrs: Vec<Attribute>
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Args {
            attrs: input.call(Attribute::parse_outer)?
        })
    }
}

pub fn awe(
	attr: TokenStream,
	input: TokenStream
) -> Result<TokenStream> {
    let attr = syn::parse2::<Args>(attr)?;
	dump!(attr);
    dump!(input);

    Ok(quote! { #input })
}
