#![feature(trace_macros)]
#[macro_use]
extern crate mac_derive;

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
}
