use husky_entity_route::EntityRoutePtr;
use husky_primitive_literal_syntax::PrimitiveLiteralData;
use husky_text::{TextRange, TextRanged};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RawPattern {
    pub range: TextRange,
    pub variant: RawPatternVariant,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RawPatternVariant {
    PrimitiveLiteral(PrimitiveLiteralData),
    OneOf { subpatterns: Vec<RawPattern> },
    EnumLiteral(EntityRoutePtr),
}

impl TextRanged for RawPattern {
    fn text_range(&self) -> TextRange {
        self.range
    }
}

impl RawPattern {
    pub fn primitive_literal(value: PrimitiveLiteralData, range: TextRange) -> Self {
        Self {
            variant: RawPatternVariant::PrimitiveLiteral(value),
            range,
        }
    }

    pub fn enum_literal(value: EntityRoutePtr, range: TextRange) -> Self {
        Self {
            variant: RawPatternVariant::EnumLiteral(value),
            range,
        }
    }

    pub fn or(self, new_pattern: RawPattern) -> Self {
        let range = self.text_range_to(&new_pattern);
        let patterns = match self.variant {
            RawPatternVariant::PrimitiveLiteral(_) | RawPatternVariant::EnumLiteral(_) => {
                vec![self, new_pattern]
            }
            RawPatternVariant::OneOf {
                subpatterns: mut patterns,
            } => {
                patterns.push(new_pattern);
                patterns
            }
        };
        RawPattern {
            variant: RawPatternVariant::OneOf {
                subpatterns: patterns,
            },
            range,
        }
    }
}