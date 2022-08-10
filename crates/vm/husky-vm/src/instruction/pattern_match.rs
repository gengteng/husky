use crate::*;
use husky_entity_route::EntityRoutePtr;

#[derive(Debug, PartialEq)]
pub struct VMPatternBranch {
    pub opt_pattern: Option<VMPattern>,
    pub body: Arc<InstructionSheet>,
}

#[derive(Debug, PartialEq)]
pub enum VMPattern {
    Primitive(__Register<'static>),
    OneOf(Vec<VMPattern>),
    EnumKind { kind_idx: i32 },
}

impl VMPattern {
    pub fn matches<'temp, 'eval>(&self, value: &__Register<'eval>) -> bool {
        match self {
            VMPattern::Primitive(primitive) => value == primitive,
            VMPattern::OneOf(subpatterns) => {
                for subpattern in subpatterns {
                    if subpattern.matches(value) {
                        return true;
                    }
                }
                false
            }
            VMPattern::EnumKind { kind_idx } => {
                let value: &__VirtualEnum =
                    unsafe { value.downcast_temp_ref(&__VIRTUAL_ENUM_VTABLE) };
                value.kind_idx == *kind_idx
            }
        }
    }
}