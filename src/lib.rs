extern crate proc_macro;
extern crate proc_macro2;

#[macro_use]
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[macro_use]
mod util;

mod ast;
mod attr;
mod derive;
mod func;

call!(
    #[proc_macro_derive(EnumRepr, attributes(enum_repr))]
    derive::enum_repr
);

call!(
    #[proc_macro_attribute]
    attr::awe(attr: TokenStream)
);

call!(
    #[proc_macro]
    func::forward_impl
);

call!(
    #[proc_macro]
    func::make_enum
);
