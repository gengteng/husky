use super::*;
use web_sys::{Event, HtmlDialogElement, HtmlInputElement, KeyboardEvent};

impl DebuggerContext {
    pub(crate) fn shown_signal(&self, trace_id: TraceId) -> &'static Signal<bool> {
        self.trace_context.trace_nodes.borrow(file!(), line!())[trace_id.0].shown
    }

    pub(super) fn toggle_shown(&'static self, trace_id: TraceId) {
        let shown_signal = self.shown_signal(trace_id);
        shown_signal.set(!shown_signal.cget());
        self.ws
            .send_message(HuskyTracerGuiMessageVariant::ToggleShow { trace_id }, None)
    }
}
