use super::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PrefixOpr {
    Minus,  // -
    Not,    // !$0
    BitNot, // ~
    Shared, // &
    Move,   // !$0 after WithType or Vec or Array
}

impl Into<RawOpnVariant> for PrefixOpr {
    fn into(self) -> RawOpnVariant {
        RawOpnVariant::Prefix(self)
    }
}

impl PrefixOpr {
    //     pub fn act_on_primitive(&self, opd: PrimitiveValueData) -> PrimitiveValueData {
    //         match self {
    //             PrefixOpr::Minus => match opd {
    //                 PrimitiveValueData::I32(i) => (-i).into(),
    //                 PrimitiveValueData::I64(i) => (-i).into(),
    //                 PrimitiveValueData::F32(f) => (-f).into(),
    //                 PrimitiveValueData::B32(_) => todo!(),
    //                 PrimitiveValueData::B64(_) => todo!(),
    //                 PrimitiveValueData::Bool(_) => todo!(),
    //                 PrimitiveValueData::Void(_) => panic!(),
    //             },
    //             PrefixOpr::Not => match opd {
    //                 PrimitiveValueData::I32(i) => i == 0,
    //                 PrimitiveValueData::I64(i) => i == 0,
    //                 PrimitiveValueData::F32(f) => f == 0.,
    //                 PrimitiveValueData::B32(b) => b == 0,
    //                 PrimitiveValueData::B64(b) => b == 0,
    //                 PrimitiveValueData::Bool(b) => !b,
    //                 PrimitiveValueData::Void(_) => panic!(),
    //             }
    //             .into(),
    //             PrefixOpr::BitNot => match opd {
    //                 PrimitiveValueData::B32(b) => (!b).into(),
    //                 PrimitiveValueData::B64(b) => (!b).into(),
    //                 _ => panic!(),
    //             },
    //             PrefixOpr::Shared => todo!(),
    //             PrefixOpr::Move => todo!(),
    //         }
    //     }

    pub fn code(self) -> &'static str {
        match self {
            PrefixOpr::Minus => "-",
            PrefixOpr::Not => "!",
            PrefixOpr::BitNot => "~",
            PrefixOpr::Shared => "&",
            PrefixOpr::Move => "!!",
        }
    }

    pub fn rust_code(self) -> &'static str {
        match self {
            PrefixOpr::Minus => "-",
            PrefixOpr::Not => "!",
            PrefixOpr::BitNot => "!",
            PrefixOpr::Shared => "&",
            PrefixOpr::Move => "!!",
        }
    }
}
