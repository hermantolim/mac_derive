#![feature(trace_macros)]
#[macro_use]
extern crate mac_derive;

use std::marker::PhantomData;

#[enum_repr(rename_all = "UPPERCASE", other(Meta1, Meta2))]
#[repr(C)]
#[derive(Debug, EnumRepr)]
#[enum_repr(default(PathBaseDefault))]
#[cfg_attr(target_os = "linux", enum_repr(rename_all = "UPPERCASE"))]
#[enum_repr(rename_all = "lowercase")]
#[enum_repr(default = "Variant1")]
#[enum_repr(attr(
    all(target_os = "linux", target_os = "macos"),
    attr_nest(nest1 = "nest1_lit", nest2 = "nest2_lit")
))]
pub enum LittleEnum {
    Variant1 = 2,
    Variant2,
    Variant3,
    Variant4,
}

#[derive(EnumRepr)]
#[repr(u16)]
#[enum_repr(rename_all = "lowercase")]
#[enum_repr(default = "Variant1")]
pub enum BigEnum {
    Variant1 = 10,
    Variant2 = 20,
}

#[cfg(target_endian = "little")]
type MyEnum = LittleEnum;

#[cfg(target_endian = "big")]
type MyEnum = BigEnum;

trace_macros!(true);
mac_derive::make_enum! {
    #[derive(Debug, Copy, Clone)]
    pub enum MakeEnum: u16 -> VarIdent {
        VarIdent(CONST_NAME = 1),
        Another(ANOTHER_CONST = 2)
    }
}

trace_macros!(false);

fn main() {
    let e = MyEnum::Variant1;

    let b = MakeEnum::Another;

    println!("{:?}", b);
    test_sym();
}

const fn ident<T: Sized>(id: T) -> T { id }

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Sym<T>(T);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct One;

const SYM1: Sym<One> = Sym(One);

macro_rules! imp {
    (impl $($traits:ident),*) => {
        $(impl traits::$traits for () {})*
    };
    (impl $($traits:ident),* for $id:ident) => {
        $(impl traits::$traits for $id {})*
    };
}

macro_rules! rev_imp {
        (impl $traits:ident into $($id:ident),*) => {
            $(
            #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
            pub struct $id;
            impl $id { pub fn new() -> Self { $id }}
            impl traits::$traits for $id {}
            )*
        };
    }

rev_imp!(impl Color into Red, Green, Blue, Yellow, Cyan, Magenta, Black);

imp!(impl Color, Derive, Endian, Limit, Default, Rename);

mod traits {
    pub trait Color {}
    pub trait Derive {}
    pub trait Endian {}
    pub trait Limit {}
    pub trait Default {}
    pub trait Rename {}
}

impl Opt for () {
    type Rename = ();
    type Default = ();
    type Limit = ();
    type Endian = ();
    type Derive = ();
    type Color = ();
}

trait Opt: Sized {
    type Rename: traits::Rename;
    type Default: traits::Default;
    type Limit: traits::Limit;
    type Endian: traits::Endian;
    type Derive: traits::Derive;
    type Color: traits::Color;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default, Hash)]
pub struct DefOpt;

impl Opt for DefOpt {
    type Rename = ();
    type Default = ();
    type Limit = ();
    type Endian = ();
    type Derive = ();
    type Color = ();
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default, Hash)]
pub struct FullOpt<O: Opt> {
    rename: PhantomData<O::Rename>,
    default: PhantomData<O::Default>,
    limit: PhantomData<O::Limit>,
    endian: PhantomData<O::Endian>,
    derive: PhantomData<O::Derive>,
    color: PhantomData<O::Color>,
}

impl FullOpt<DefOpt> {
    fn new() -> FullOpt<DefOpt> {
        FullOpt {
            rename: PhantomData,
            default: PhantomData,
            limit: PhantomData,
            endian: PhantomData,
            derive: PhantomData,
            color: PhantomData,
        }
    }
}

use std::convert::identity;

fn test_sym() {
    let s1 = SYM1;
    let id = ident(s1);

    let a = FullOpt::new();

    println!("{:?} {:?}", s1, id);
}
fn te() {
    let a = Red::new();
    let b = Blue::new();

    struct Color<N: traits::Color, P: traits::Color> {
        now: PhantomData<N>,
        _prev: PhantomData<P>,
    };

    impl Color<Red, Red> {
        pub fn new() -> Color<Red, Red> {
            Color {
                now: PhantomData,
                _prev: PhantomData,
            }
        }
    }
    impl<P: traits::Color> Color<Red, P> {
        fn r(self) -> Color<Red, Red> {
            Color {
                now: PhantomData,
                _prev: PhantomData,
            }
        }
        fn b(self) -> Color<Blue, Red> {
            Color {
                now: PhantomData,
                _prev: PhantomData,
            }
        }

        fn g(self) -> Color<Green, Red> {
            Color {
                now: PhantomData,
                _prev: PhantomData,
            }
        }

        fn c(self) -> Color<Cyan, Red> {
            Color {
                now: PhantomData,
                _prev: PhantomData,
            }
        }
    }

    let c = Color::new();
    let r = c.r();
    let b = c.b();
    let g = c.g();
    let y = c.c();
}
