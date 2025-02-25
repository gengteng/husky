use crate::*;

pub trait Transformer {
    type Input: ?Sized;
    type InputStorage: FoldableStorage<Value = Self::Input>;
    type Output;

    fn _enter_block(&mut self);
    fn _exit_block(&mut self);
    fn transform(
        &mut self,
        indent: Indent,
        input: &Self::Input,
        enter_block: impl FnOnce(&mut Self),
    ) -> Self::Output;
    fn foldable_inputs(&self) -> &Self::InputStorage;
    fn foldable_outputs_mut(&mut self) -> &mut FoldableList<Self::Output>;
    fn misplaced(&self) -> Self::Output;

    fn transform_all_recr<'a>(&mut self, mut iter: FoldableIter<'a, Self::InputStorage>)
    where
        Self::Input: 'a,
    {
        while let Some(item) = iter.next() {
            use husky_check_utils::*;
            while self.foldable_outputs_mut().nodes.len() < item.idx {
                let indent = self.foldable_inputs().indent(item.idx);
                let misplaced = self.misplaced();
                let folding_end = self.foldable_inputs().folding_end(item.idx);
                self.foldable_outputs_mut()
                    .append(indent, misplaced, folding_end)
            }
            should_eq!(self.foldable_outputs_mut().nodes.len(), item.idx);
            let mut block_entered = false;

            // parse current
            let parse_result = self.transform(item.indent, item.value, |this| {
                block_entered = true;
                this._enter_block();
            });
            self.foldable_outputs_mut()
                .append(item.indent, parse_result, item.folding_end);
            // parse children
            if !block_entered {
                self._enter_block();
            }
            if let Some(children) = item.opt_children {
                self.transform_all_recr(children);
            }
            self._exit_block();
        }
    }
}
