use husky_trace_time::HuskyTracetime;

pub struct HuskyDebuggerInternal {
    pub(crate) tracetime: HuskyTracetime,
    pub(crate) next_request_id: usize,
}
