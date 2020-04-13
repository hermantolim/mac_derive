use std::{fmt, str::FromStr};
use syn::{Error, Result};

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum RenameRule {
    Upper,
    Lower,
    Pascal,
    Snake,
    Scream,
}

impl RenameRule {
    const ALL: &'static str =
        "UPPERCASE | lowercase | PascalCase | snake_case | SCREAMING_SNAKE_CASE";
}

impl FromStr for RenameRule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "UPPERCASE" => Ok(RenameRule::Upper),
            "lowercase" => Ok(RenameRule::Lower),
            "PascalCase" => Ok(RenameRule::Pascal),
            "snake_case" => Ok(RenameRule::Snake),
            "SCREAMING_SNAKE_CASE" => Ok(RenameRule::Scream),
            _ => err!(
                r##"invalid rename attribute: {}, expected #[enum_repr(rename = "{}")]"##,
                s,
                RenameRule::ALL
            ),
        }
    }
}

impl fmt::Display for RenameRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            RenameRule::Upper => "UPPERCASE",
            RenameRule::Lower => "lowercase",
            RenameRule::Pascal => "PascalCase",
            RenameRule::Snake => "snake_case",
            RenameRule::Scream => "SCREAMING_SNAKE_CASE",
        })
    }
}
