use crate::{
    atom::{parser::AtomLRParser, AtomKind},
    *,
};
use text::TextRanged;
use token::Token;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_expr(&mut self, tokens: &[Token]) -> AstResult<RawExprIdx> {
        let atoms = AtomLRParser::new(self.symbol_proxy(), tokens).parse_all()?;
        should!(atoms.len() > 0);
        Ok({
            let mut atom_iter = atoms.iter().peekable();
            let mut stack = ExprStack::new(&mut self.arena);
            while let Some(atom) = atom_iter.next() {
                match &atom.kind {
                    AtomKind::Variable(_) | AtomKind::Literal(_) | AtomKind::Scope(_, _) => {
                        stack.accept_atom_expr(atom.into())
                    }
                    AtomKind::Binary(opr) => stack.accept_binary(*opr),
                    AtomKind::Prefix(prefix) => stack.accept_prefix(*prefix, atom.text_end()),
                    AtomKind::Suffix(suffix) => stack.accept_suffix(*suffix, atom.text_end()),
                    AtomKind::ListStart(bra, attr) => {
                        stack.accept_list_start(*bra, *attr, atom.text_start())
                    }
                    AtomKind::ListEnd(ket, attr) => {
                        stack.accept_list_end(*ket, *attr, atom.text_end())?
                    }
                    AtomKind::ListItem => stack.accept_list_item(),
                    AtomKind::LambdaHead(args) => {
                        stack.accept_lambda_head(args.clone(), atom.text_start())
                    }
                }
            }
            stack.finish()
        })
    }
}