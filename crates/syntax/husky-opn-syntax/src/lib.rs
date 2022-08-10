mod binary;
mod list;
mod prefix;
mod suffix;

pub use binary::*;
pub use list::*;
pub use prefix::*;
pub use suffix::*;

use husky_entity_route::EntityRoutePtr;
use husky_text::RangedCustomIdentifier;

#[derive(Clone, PartialEq, Eq)]
pub enum RawOpnVariant {
    Binary(BinaryOpr),
    Prefix(PrefixOpr),
    Suffix(RawSuffixOpr),
    List(ListOpr),
    Field(RangedCustomIdentifier),
}

impl std::fmt::Debug for RawOpnVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Binary(arg0) => {
                f.write_str("Binary ")?;
                arg0.fmt(f)
            }
            Self::Prefix(arg0) => {
                f.write_str("Prefix ")?;
                arg0.fmt(f)
            }
            Self::Suffix(arg0) => {
                f.write_str("Suffix ")?;
                arg0.fmt(f)
            }
            Self::List(arg0) => {
                f.write_str("List ")?;
                arg0.fmt(f)
            }
            RawOpnVariant::Binary(_) => todo!(),
            RawOpnVariant::Prefix(_) => todo!(),
            RawOpnVariant::Suffix(_) => todo!(),
            RawOpnVariant::List(_) => todo!(),
            RawOpnVariant::Field(field_ident) => {
                f.write_str("FieldAccess .")?;
                field_ident.ident.fmt(f)
            }
        }
    }
}