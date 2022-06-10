use super::*;
use crate::*;
use defn_head::Parameter;
use semantics_entity::*;
use word::{CustomIdentifier, Identifier};

impl HuskyTraceTime {
    pub fn new_call_head(&mut self, entity: Arc<EntityDefn>, text: &Text) -> Arc<Trace> {
        let tokens = match entity.variant {
            EntityDefnVariant::Func { ref parameters, .. } => {
                routine_call_head_tokens("func ", entity.ident, parameters, text)
            }
            EntityDefnVariant::Proc {
                parameters: ref parameters,
                ..
            } => routine_call_head_tokens("proc ", entity.ident, parameters, text),
            _ => todo!(),
        };
        return self.new_trace(None, 0, TraceVariant::CallHead { entity, tokens }, text);

        fn routine_call_head_tokens<'eval>(
            routine_keyword: &'static str,
            ident: Identifier,
            parameters: &[Parameter],
            text: &Text,
        ) -> Vec<TraceTokenProps> {
            let mut tokens = vec![
                keyword!(routine_keyword),
                ident!(ident.as_str()),
                special!("("),
            ];
            for i in 0..parameters.len() {
                let input_placeholder = &parameters[i];
                tokens.push(ident!(input_placeholder.ranged_ident.ident.as_str()));
                tokens.push(special!(": "));
                tokens.push(route!(text.ranged(input_placeholder.ranged_ty.range)));
                if i < parameters.len() - 1 {
                    tokens.push(special!(", "));
                }
            }
            tokens.push(special!("):"));
            tokens
        }
    }
}