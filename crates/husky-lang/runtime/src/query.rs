use common::epin;
use dataset::LabeledData;
use feature::{
    eval_feature_block, eval_feature_expr, eval_feature_stmt, FeatureBlock, FeatureExpr,
    FeaturePtr, FeatureSheet, FeatureStmt,
};
use vm::EvalValue;

use trace::*;

use crate::*;

pub trait AskCompileTime {
    fn compile_time(&self, version: usize) -> &HuskyLangCompileTime;
}

pub trait EvalFeature {
    fn session(&self) -> &Arc<Mutex<Session<'static>>>;

    fn eval_feature_block(
        &self,
        block: &FeatureBlock,
        input_id: usize,
    ) -> EvalValue<'static, 'static> {
        let dev = &mut self.session().lock().unwrap().dev;
        let sheet = &mut dev.sheets[input_id];
        let indicator = &mut dev.indicators[input_id];
        let input = dev.loader.load(input_id).input;
        eval_feature_block(block, input, sheet, indicator)
    }

    fn eval_feature_stmt(
        &self,
        stmt: &FeatureStmt,
        input_id: usize,
    ) -> EvalValue<'static, 'static> {
        let dev = &mut self.session().lock().unwrap().dev;
        let sheet = &mut dev.sheets[input_id];
        let indicator = &mut dev.indicators[input_id];
        let input = dev.loader.load(input_id).input;
        eval_feature_stmt(stmt, input, sheet, indicator)
    }

    fn eval_feature_expr(
        &self,
        expr: &FeatureExpr,
        input_id: usize,
    ) -> EvalValue<'static, 'static> {
        let dev = &mut self.session().lock().unwrap().dev;
        let sheet = &mut dev.sheets[input_id];
        let indicator = &mut dev.indicators[input_id];
        let input = dev.loader.load(input_id).input;
        eval_feature_expr(expr, input, sheet, indicator)
    }
}

#[salsa::query_group(RuntimeQueryGroupStorage)]
pub trait RuntimeQueryGroup: AskCompileTime + AllocateTrace + EvalFeature {
    #[salsa::input]
    fn package_main(&self) -> FilePtr;

    #[salsa::input]
    fn version(&self) -> usize;

    fn subtraces(&self, id: TraceId, input_locked_on: Option<usize>) -> Arc<Vec<Arc<Trace>>>;
    fn root_traces(&self) -> Arc<Vec<Arc<Trace>>>;

    fn trace_stalk(&self, trace_id: TraceId, input_id: usize) -> Arc<TraceStalk>;
}

pub fn root_traces(this: &dyn RuntimeQueryGroup) -> Arc<Vec<Arc<Trace>>> {
    let compile_time = this.compile_time(this.version());
    let package_main = this.package_main();
    Arc::new(vec![this.new_trace(
        None,
        package_main,
        0,
        TraceKind::Main(compile_time.main_feature_block(package_main).unwrap()),
    )])
}

pub fn subtraces(
    this: &dyn RuntimeQueryGroup,
    trace_id: TraceId,
    input_locked_on: Option<usize>,
) -> Arc<Vec<Arc<Trace>>> {
    let trace: &Trace = &this.trace(trace_id);
    match trace.kind {
        TraceKind::Main(ref block) => Arc::new(this.feature_block_subtraces(&trace, block)),
        TraceKind::FeatureStmt(ref stmt) => Arc::new(vec![]),
        TraceKind::FeatureExpr(ref expr) => this
            .trace_allocator()
            .feature_expr_subtraces(trace, expr, None),
        TraceKind::FeatureBranch(ref branch) => this.feature_branch_subtraces(trace, branch),
    }
}

pub fn trace_stalk(
    this: &dyn RuntimeQueryGroup,
    trace_id: TraceId,
    input_id: usize,
) -> Arc<TraceStalk> {
    let trace: &Trace = &this.trace(trace_id);
    let result = match trace.kind {
        TraceKind::Main(ref block) => Arc::new(TraceStalk {
            extra_tokens: vec![
                trace::fade!(" = "),
                this.eval_feature_block(block, input_id).into(),
            ],
        }),
        TraceKind::FeatureStmt(_) => todo!(),
        TraceKind::FeatureBranch(_) => todo!(),
        TraceKind::FeatureExpr(_) => todo!(),
    };
    result
}