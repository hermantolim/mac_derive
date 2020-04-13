use std::fmt::{self, Debug, Display};
use syn::Path;

#[derive(Copy, Clone)]
pub struct Symbol(&'static str);

impl PartialEq<Symbol> for Path {
    fn eq(&self, word: &Symbol) -> bool { self.is_ident(word.0) }
}

impl<'a> PartialEq<Symbol> for &'a Path {
    fn eq(&self, word: &Symbol) -> bool { self.is_ident(word.0) }
}

impl<T: ?Sized + AsRef<str>> PartialEq<T> for Symbol {
    fn eq(&self, word: &T) -> bool { self.0 == word.as_ref() }
}

impl AsRef<str> for Symbol {
    fn as_ref(&self) -> &str { &*self.0 }
}

impl Debug for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

macro_rules! make_symbols {
    ($( $const:ident = $val:expr ),+) => {
        $(
            #[allow(unused)]
            pub const $const: Symbol = Symbol($val);
        )+
    };
}

make_symbols! {
    ID = "enum_repr",
    CRATE = "crate",
    ALIAS = "alias",
    BORROW = "borrow",
    BOUND = "bound",
    CONTENT = "content",
    DEFAULT = "default",
    DENY_UNKNOWN_FIELDS = "deny_unknown_fields",
    FIELD_IDENTIFIER = "field_identifier",
    FLATTEN = "flatten",
    FROM = "from",
    GETTER = "getter",
    SETTER = "setter",
    INTO = "into",
    OTHER = "other",
    RENAME = "rename",
    DISCR = "discr",
    DISPLAY = "display",
    RENAME_ALL = "rename_all",
    SKIP = "skip",
    TAG = "tag",
    TRANSPARENT = "transparent",
    TRY_FROM = "try_from",
    UNTAGGED = "untagged",
    VARIANT_IDENTIFIER = "variant_identifier",
    WITH = "with",
    REPR = "repr",
    ATTR = "attr",
    CFG_ATTR = "cfg_attr",
    ATTR_NEST = "attrr_nest"
}
