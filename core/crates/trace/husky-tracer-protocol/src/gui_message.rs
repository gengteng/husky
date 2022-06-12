use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(tag = "kind")]
pub struct DebuggerGuiMessage {
    pub opt_request_id: Option<usize>,
    pub variant: DebuggerGuiMessageVariant,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(tag = "kind")]
pub enum DebuggerGuiMessageVariant {
    InitRequest,
    Activate {
        trace_id: TraceId,
        opt_focus_for_figure: Option<Focus>,
    },
    ToggleExpansion {
        trace_id: TraceId,
        request_subtraces: bool,
    },
    ToggleShow {
        trace_id: TraceId,
    },
    Trace {
        id: TraceId,
    },
    DecodeFocus {
        command: String,
    },
    LockFocus {
        focus: Focus,
        opt_active_trace_id_for_figure: Option<TraceId>,
    },
    TraceStalk {
        trace_id: TraceId,
        input_id: usize,
    },
    UpdateFigureControlProps {
        trace_id: TraceId,
        focus: Focus,
        figure_control_props: FigureControlProps,
    },
}