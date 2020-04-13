#![allow(unused_macros)]

pub use proc_macro2::{Span, TokenStream};
pub use syn::{Error, Ident};

macro_rules! count {
    ($head:expr $(, $tail:expr)*) => {  1 + $crate::count!($($tail),*) };
    ($tail:expr) => { 1 };
    () => { 0 };
}

macro_rules! call {
	(
        $( #[ $outer:meta ] )*
		$mod:ident::$method:ident
    	$( ($arg:ident: $argv:ty) )? as $ty:ty
	) => {
        $( #[ $outer ] )*
		pub fn $method(
			$( $arg: $argv )?
			input: proc_macro::TokenStream
		) -> proc_macro::TokenStream {
			$mod::$method(
				$( proc_macro2::TokenStream::from($arg) ,)?
				syn::parse2::<$ty>(
					proc_macro2::TokenStream::from(input)
				)
			)
			.unwrap_or_else(|e| e.to_compile_error())
			.into()
		}
	};

	(
        $( #[ $outer:meta ] )*
		$mod:ident::$method:ident
    	$( ($arg:ident: $argv:ty) )?
	) => {
        $( #[ $outer ] )*
		pub fn $method(
			$( $arg: $argv ,)?
			input: proc_macro::TokenStream
		) -> proc_macro::TokenStream {
			$mod::$method(
				$( proc_macro2::TokenStream::from($arg) ,)?
				proc_macro2::TokenStream::from(input)
 			)
			.unwrap_or_else(|e| e.to_compile_error())
			.into()
		}
	};
}

/// split input into fields
macro_rules! split {
	($id:ident as $($field:ident),+) => {
		$(let $field = &$id.$field;)+
	};
}

/// split input into fields
macro_rules! split_owned {
	($id:ident as $($field:ident),+) => {
		$(let $field = $id.$field;)+
	};
}

/// strip #[?] attr meta
macro_rules! strip {
    ($id:ident, $what:expr) => {
        $id.iter().filter(|at| !at.path.is_ident($what))
    };
}

/// strip doc attribute meta
macro_rules! strip_doc {
    ($id:ident) => {
        strip!($id, "doc")
    };
}

/// take #[?] attr
macro_rules! take {
    ($id:ident, $what:expr) => {
        $id.iter().filter(|at| at.path.is_ident($what))
    };
}

/// take #[cfg] attr
macro_rules! take_cfg {
    ($id:ident) => {
        take!($id, "cfg")
    };
}

macro_rules! dump {
	( $( $( $arg:tt );+) +) => {
        $(
    		println!("{:>10} {} => [", "-", stringify!( $( $arg )* ));
    		println!("{:#?}", $($arg)*);
    		println!("]");
    		println!("{:>30}", "-");
        )+
	};
}

pub fn wrap_in_const(
    nl_path: Option<&syn::Path>,
    trait_: &str,
    ty: &Ident,
    code: TokenStream,
) -> TokenStream {
    let dummy_const = Ident::new(
        &format!("_IMPL_NL_{}_FOR_{}", trait_, ty),
        Span::call_site(),
    );

    let use_nl = match nl_path {
        Some(path) => quote! {
            use #path as _derive;
        },
        None => quote! {
            #[allow(unknown_lints)]
            #[cfg_attr(feature = "cargo-clippy", allow(useless_attribute))]
            #[allow(rust_2018_idioms)]
            extern crate mac_derive as _derive;
        },
    };

    quote! {
        #[allow(non_upper_case_globals)]
        #[allow(unused_attributes)]
        #[allow(unused_qualifications)]
        const #dummy_const: () = {
            #use_nl
            #code
        };
    }
}

macro_rules! err {
    ($span:ident: $( $e:expr ),+ $(,)* ) => {
        Err(syn::Error::new(
            $span.span(),
            format!( $( $e ),+ )
        ))
    };
    ( $( $e:expr ),+ $(,)* ) => {
        Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            format!( $( $e ),+ )
        ))
    };
}
