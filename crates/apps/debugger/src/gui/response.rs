use super::*;
use crate::*;

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "type")]
pub(super) enum Response<'a> {
    Init {
        init_state: InitState<'a>,
    },
    Subtraces {
        id: TraceId,
        input_locked_on: Option<usize>,
        subtraces: Arc<Vec<Arc<Trace<'static>>>>,
    },
    Figure {
        id: TraceId,
        figure: Option<FigureProps>,
    },
    DidActivate {
        id: TraceId,
    },
    DidToggleExpansion {
        id: TraceId,
    },
    DidToggleShow {
        id: TraceId,
    },
    Trace {
        id: TraceId,
        trace: Arc<Trace<'static>>,
    },
    DidLockInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        input_locked_on: Option<Option<usize>>,
        message: Option<String>,
    },
    TraceStalk {
        trace_id: TraceId,
        input_id: usize,
        stalk: Arc<TraceStalk<'static>>,
    },
}