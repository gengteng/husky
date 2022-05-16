use wild_utils::ref_to_mut_ref;

use super::*;
use crate::*;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_match(&mut self, token_group: &[Token]) -> AstResult<RawStmtVariant> {
        expect_block_head!(token_group);
        let match_expr = self.parse_expr(&token_group[1..(token_group.len() - 1)])?;
        Ok(RawStmtVariant::Match {
            match_expr,
            match_contract: MatchContract::Pure,
        })
    }

    pub(super) fn parse_case(&mut self, token_group: &[Token]) -> AstResult<RawStmtVariant> {
        expect_block_head!(token_group);
        if token_group.len() < 3 {
            return err!("expect `case <pattern>:`", token_group.text_range());
        }
        let atoms = self.parse_atoms(&token_group[1..(token_group.len() - 1)], |parser| {
            parser.parse_all()
        })?;
        Ok(RawStmtVariant::Branch(RawBranchVariant::Case {
            pattern: MatchPatternParser::new(&atoms).parse()?,
        }))
    }

    fn parse_match_pattern(&mut self, tokens: &[Token]) -> AstResult<CasePattern> {
        todo!()
    }

    fn parse_first_match_pattern(&mut self, tokens: &[Token]) -> AstResult<AstKind> {
        todo!()
    }
}

pub struct MatchPatternParser<'a> {
    atom_iter: std::slice::Iter<'a, Atom>,
}

impl<'a> MatchPatternParser<'a> {
    pub fn new(atoms: &'a [Atom]) -> Self {
        Self {
            atom_iter: atoms.iter(),
        }
    }

    pub fn parse(mut self) -> AstResult<CasePattern> {
        let mut pattern = self.parse_simple_pattern()?.unwrap();
        while let Some(separator) = self.atom_iter.next() {
            match separator.kind {
                AtomVariant::Binary(BinaryOpr::Pure(PureBinaryOpr::BitOr)) => {
                    if let Some(new_pattern) = self.parse_simple_pattern()? {
                        pattern = pattern.or(new_pattern)?
                    } else {
                        return err!("expect pattern after `|`", separator.range);
                    }
                }
                _ => todo!(),
            }
        }
        Ok(pattern)
    }

    fn parse_simple_pattern(&mut self) -> AstResult<Option<CasePattern>> {
        Ok(if let Some(atom) = self.atom_iter.next() {
            Some(match atom.kind {
                AtomVariant::EntityRoute { route, kind } => match kind {
                    EntityKind::EnumLiteral => CasePattern::enum_literal(route, atom.range),
                    _ => err!(format!("expect enum literal"), atom.range)?,
                },
                AtomVariant::Variable { varname, init_row } => todo!(),
                AtomVariant::FrameVariable { varname, init_row } => todo!(),
                AtomVariant::ThisData {
                    opt_ty,
                    opt_contract,
                } => todo!(),
                AtomVariant::Unrecognized(_) => todo!(),
                AtomVariant::PrimitiveLiteral(value) => {
                    CasePattern::primitive_literal(value, atom.range)
                }
                AtomVariant::Binary(_) => todo!(),
                AtomVariant::Prefix(_) => todo!(),
                AtomVariant::Suffix(_) => todo!(),
                AtomVariant::ListStart(_, _) => todo!(),
                AtomVariant::ListEnd(_, _) => todo!(),
                AtomVariant::ListItem => todo!(),
                AtomVariant::LambdaHead(_) => todo!(),
            })
        } else {
            None
        })
    }
}