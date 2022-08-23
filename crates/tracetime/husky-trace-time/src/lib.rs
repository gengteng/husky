mod impl_figure;
mod impl_figure_control;
mod impl_lines;
mod impl_ops;
mod impl_pin;
mod impl_restriction;
mod impl_subtraces;
mod impl_trace;
mod impl_trace_stalk;
mod impl_trace_stats;
mod trace_node;

use avec::Avec;
use husky_comptime::*;
use husky_defn_head::Parameter;
use husky_eager_semantics::*;
use husky_entity_semantics::*;
use husky_feature_eval::EvalFeature;
use husky_feature_gen::*;
use husky_file::FilePtr;
use husky_init_syntax::*;
use husky_loop_syntax::*;
use husky_opn_syntax::*;
use husky_print_utils::p;
use husky_root_static_defn::__resolve_root_defn;
use husky_runtime::*;
use husky_text::{HuskyText, TextQueryGroup};
use husky_trace::*;
use husky_trace_protocol::*;
use husky_vm::*;
use impl_lines::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use sync_utils::ASafeRwLock;
use trace_node::*;
use upcast::Upcast;
use vec_like::VecSet;
use wild_utils::{arb_ref, ref_to_mut_ref};

pub struct HuskyTraceTime {
    runtime: HuskyRuntime,
    restriction: Restriction,
    pins: VecSet<TraceId>,
    trace_nodes: Vec<Option<TraceNode>>,
    opt_active_trace_id: Option<TraceId>,
    pub trace_stalks: HashMap<TraceStalkKey, TraceStalkData>,
    pub trace_statss: HashMap<TraceStatsKey, Option<TraceStats>>,
    root_trace_ids: Vec<TraceId>,
    subtrace_ids_map: HashMap<SubtracesKey, Vec<TraceId>>,
    figure_controls: HashMap<FigureControlKey, FigureControlData>,
}

impl HuskyTraceTime {
    pub fn new(
        init_compile_time: impl FnOnce(&mut HuskyComptime),
        eval_time_config: HuskyRuntimeConfig,
    ) -> Self {
        let mut trace_time = Self {
            runtime: HuskyRuntime::new(init_compile_time, eval_time_config),
            trace_nodes: Default::default(),
            trace_stalks: Default::default(),
            trace_statss: Default::default(),
            opt_active_trace_id: Default::default(),
            subtrace_ids_map: Default::default(),
            figure_controls: Default::default(),
            root_trace_ids: Default::default(),
            restriction: Default::default(),
            pins: Default::default(),
        };
        assert!(trace_time.restriction.opt_sample_id().is_none());
        trace_time.update();
        trace_time
    }

    pub fn opt_active_trace_id(&mut self) -> Option<TraceId> {
        self.opt_active_trace_id
    }

    pub fn activate(&mut self, trace_id: TraceId) {
        self.opt_active_trace_id = Some(trace_id);
    }

    pub fn root_traces(&self) -> Vec<TraceId> {
        self.root_trace_ids.clone()
    }

    pub fn runtime(&self) -> &HuskyRuntime {
        &self.runtime
    }

    pub fn comptime(&self) -> &HuskyComptime {
        self.runtime.comptime()
    }

    pub fn all_trace_nodes(&self) -> Vec<TraceNodeData> {
        self.trace_nodes
            .iter()
            .filter_map(|opt_trace| opt_trace.as_ref().map(|node| node.to_data()))
            .collect()
    }

    pub fn subtrace_ids(&mut self, trace_id: TraceId) -> Vec<TraceId> {
        let trace = &self.trace(trace_id);
        let opt_sample_id = self.restriction.opt_sample_id();
        if !trace.raw_data.has_subtraces(opt_sample_id.is_some()) {
            return vec![];
        }
        let key = SubtracesKey::new(trace.raw_data.kind, trace_id, opt_sample_id);
        if let Some(subtrace_ids) = self.subtrace_ids_map.get(&key) {
            subtrace_ids.clone()
        } else {
            if let Some(subtrace_ids) = self.gen_subtraces(trace_id) {
                self.subtrace_ids_map
                    .insert(key.clone(), subtrace_ids.clone());
                subtrace_ids
            } else {
                todo!()
            }
        }
    }

    fn trace_node_data(&self, trace_id: TraceId) -> TraceNodeData {
        self.trace_nodes[trace_id.0].as_ref().unwrap().to_data()
    }

    pub(crate) fn next_id(&mut self) -> TraceId {
        self.trace_nodes.push(None);
        TraceId(self.trace_nodes.len() - 1)
    }

    fn new_trace(
        &mut self,
        opt_parent_id: Option<TraceId>,
        indent: Indent,
        variant: TraceVariant<'static>,
    ) -> TraceId {
        let trace_id = self.next_id();
        let trace = {
            let (file, range) = variant.file_and_range();
            let text = self.runtime().comptime().text(file).unwrap();
            let reachable = variant.reachable();
            let can_have_subtraces = variant.can_have_subtraces(reachable);
            let lines = self.trace_lines(trace_id, indent, &variant, opt_parent_id.is_some());
            Trace {
                raw_data: TraceData {
                    id: trace_id,
                    opt_parent_id,
                    indent,
                    compile_time_version: 0, //compile time version
                    can_have_subtraces,
                    reachable,
                    lines,
                    kind: variant.kind(),
                    // opt_stats: variant.opt_stats(self.runtime()).expect("todo"),
                },
                variant,
                file,
                range,
            }
        };
        assert!(self.trace_nodes[trace.id().0].is_none());
        self.trace_nodes[trace_id.0] = Some(TraceNode {
            expansion: false,
            shown: match trace.raw_data.kind {
                TraceKind::FeatureExprLazy | TraceKind::FeatureExprEager | TraceKind::EagerExpr => {
                    trace.raw_data.opt_parent_id.is_some()
                }
                _ => true,
            },
            arrival: false,
            pin: false,
            enter: false,
            trace,
        });
        trace_id
    }

    pub fn toggle_expansion(
        &mut self,
        trace_id: TraceId,
    ) -> Option<(
        Vec<TraceNodeData>,
        Vec<TraceId>,
        Vec<(TraceStalkKey, TraceStalkData)>,
        Vec<(TraceStatsKey, Option<TraceStats>)>,
    )> {
        let old_len = self.trace_nodes.len();
        let expansion = &mut self.trace_nodes[trace_id.0].as_mut().unwrap().expansion;
        *expansion = !*expansion;
        let subtrace_ids = self.subtrace_ids(trace_id);
        if self.trace_nodes.len() > old_len {
            let new_traces: Vec<TraceNodeData> = self.trace_nodes[old_len..]
                .iter()
                .map(|opt_node| opt_node.as_ref().unwrap().to_data())
                .collect();
            let trace_stalks = self.collect_new_trace_stalks();
            let trace_stats = self.collect_new_trace_statss();
            Some((new_traces, subtrace_ids, trace_stalks, trace_stats))
        } else {
            None
        }
    }

    pub fn is_expanded(&mut self, trace_id: TraceId) -> bool {
        self.trace_nodes[trace_id.0].as_ref().unwrap().expansion
    }

    pub fn toggle_show(&mut self, trace_id: TraceId) {
        let shown = &mut self.trace_nodes[trace_id.0].as_mut().unwrap().shown;
        *shown = !*shown
    }

    pub fn trace(&self, trace_id: TraceId) -> &Trace {
        &self.trace_nodes[trace_id.0].as_ref().unwrap().trace
    }

    pub(crate) unsafe fn trace_ref<'a>(&self, trace_id: TraceId) -> &'a Trace {
        let ptr: *const Trace = &self.trace_nodes[trace_id.0].as_ref().unwrap().trace;
        &*ptr
    }

    pub fn init_data(&mut self) -> InitData {
        let root_trace_ids = self.root_trace_ids.clone();
        let mut figure_canvases = Vec::default();
        let mut figure_controls = Vec::default();
        let opt_active_trace_id = self.opt_active_trace_id;
        if let Some(active_trace_id) = opt_active_trace_id {
            let active_trace = self.trace(active_trace_id);
            let figure_canvas_key =
                FigureCanvasKey::from_trace_data(&active_trace.raw_data, &self.restriction);
            figure_canvases.push((
                figure_canvas_key,
                self.figure_canvas(active_trace_id).unwrap(),
            ));
            figure_controls.push((
                FigureControlKey::from_trace_data(&active_trace.raw_data, &self.restriction),
                self.figure_control(active_trace_id),
            ));
        }
        let pins = self.pins.clone();
        let traces = self.all_trace_nodes();
        InitData {
            trace_init_data: TraceInitData {
                opt_active_trace_id,
                trace_nodes: traces,
                subtrace_ids_map: self
                    .subtrace_ids_map
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect(),
                trace_stalks: self
                    .trace_stalks
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect(),
                trace_statss: self
                    .trace_statss
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect(),
                root_trace_ids,
            },
            restriction: self.restriction.clone(),
            figure_canvases,
            figure_controls,
            pins,
        }
    }

    fn vm_config(&self) -> &VMConfig {
        self.runtime().vm_config()
    }
}

// pub trait ProduceTrace: AskRuntime {
//     fn trace_factory(&self) -> &TraceFactory;

//     fn feature_repr_subtraces(
//         &self,
//         parent: &Trace,
//         feature_repr: &FeatureRepr,
//     ) ->  Vec<TraceId>  {
//         let text = &self.comptime().text(parent.file).unwrap();
//         Arc::new(
//             self.trace_factory()
//                 .feature_repr_subtraces(parent, feature_repr, text),
//         )
//     }

//     fn feature_block_subtraces(
//         &self,
//         parent: &Trace,
//         feature_block: &FeatureLazyBlock,
//     ) ->  Vec<TraceId>  {
//         let text = &self.comptime().text(parent.file).unwrap();
//         Arc::new(
//             self.trace_factory()
//                 .feature_lazy_block_subtraces(parent, feature_block, text),
//         )
//     }

//     fn feature_branch_subtraces(
//         &self,
//         parent: &Trace,
//         branch: &FeatureBranch,
//     ) ->  Vec<TraceId>  {
//         let text = &self.comptime().text(parent.file).unwrap();
//         self.trace_factory()
//             .feature_branch_subtraces(parent, branch, self.trace_factory(), text)
//     }

//     fn eager_expr_subtraces(
//         &self,
//         parent: &Trace,
//         expr: &Arc<EagerExpr>,
//         history: &Arc<History>,
//     ) ->  Vec<TraceId>  {
//         todo!()
//         // self.trace_factory()
//     }

//     fn loop_subtraces(
//         &self,
//         parent: &Trace,
//         loop_kind: VMLoopKind,
//         loop_stmt: &Arc<ProcStmt>,
//         stmts: &Arc<Vec<Arc<ProcStmt>>>,
//         stack_snapshot: &StackSnapshot<'static>,
//         body_instruction_sheet: &Arc<InstructionSheet>,
//         verbose: bool,
//     ) ->  Vec<TraceId>  {
//         self.trace_factory().loop_subtraces(
//             self.runtime.upcast(),
//             parent,
//             loop_kind,
//             loop_stmt,
//             stmts,
//             stack_snapshot,
//             body_instruction_sheet,
//             verbose,
//         )
//     }

//     fn loop_frame_subtraces(
//         &self,
//         loop_stmt: &Arc<ProcStmt>,
//         stmts: &[Arc<ProcStmt>],
//         instruction_sheet: &InstructionSheet,
//         loop_frame_data: &LoopFrameData<'static>,
//         parent: &Trace,
//         verbose: bool,
//     ) ->  Vec<TraceId>  {
//         let text = &self.comptime().text(parent.file).unwrap();
//         self.trace_factory().loop_frame_subtraces(
//             self.runtime.upcast(),
//             text,
//             loop_stmt,
//             stmts,
//             instruction_sheet,
//             loop_frame_data,
//             parent,
//             verbose,
//         )
//     }

//     fn proc_branch_subtraces(
//         &self,
//         stmts: &[Arc<ProcStmt>],
//         instruction_sheet: &InstructionSheet,
//         stack_snapshot: &StackSnapshot<'static>,
//         parent: &Trace,
//         verbose: bool,
//     ) ->  Vec<TraceId>  {
//         let text = &self.comptime().text(parent.file).unwrap();
//         self.trace_factory().proc_branch_subtraces(
//             self.runtime.upcast(),
//             text,
//             stmts,
//             instruction_sheet,
//             stack_snapshot,
//             parent,
//             verbose,
//         )
//     }

//     fn new_trace(
//         &self,
//         parent_id: Option<TraceId>,
//         file: FilePtr,
//         indent: Indent,
//         kind: TraceVariant<'static>,
//     ) -> TraceId {
//         self.trace_factory().new_trace(
//             parent_id,
//             indent,
//             kind,
//             &self.comptime().text(file).unwrap(),
//         )
//     }
// }