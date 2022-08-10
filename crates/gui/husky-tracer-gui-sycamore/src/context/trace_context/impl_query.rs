use super::*;

impl TraceContext {
    pub(crate) fn is_expanded(&self, trace_id: TraceId) -> bool {
        self.trace_nodes.borrow(file!(), line!())[trace_id.0]
            .expansion
            .cget()
    }

    pub(crate) fn expansion_read_signal(&self, trace_id: TraceId) -> &'static ReadSignal<bool> {
        self.trace_nodes.borrow(file!(), line!())[trace_id.0]
            .expansion
            .read()
    }

    pub(crate) fn is_shown(&self, trace_id: TraceId) -> bool {
        self.trace_nodes.borrow(file!(), line!())[trace_id.0]
            .shown
            .cget()
    }

    pub(crate) fn shown_read_signal(&self, trace_id: TraceId) -> &'static ReadSignal<bool> {
        self.trace_nodes.borrow(file!(), line!())[trace_id.0]
            .shown
            .read()
    }

    pub(crate) fn did_toggle_show(&mut self, trace_id: TraceId) {
        todo!()
        // self.shown_stores.update(id, (shown) => !shown);
        // self.update_trace_listing(restriction);
    }

    pub(crate) fn did_activate(&self, trace_id: TraceId) {
        self.opt_active_trace_id.set(Some(trace_id))
    }

    // print_state() {
    //     console.log("trace control:");
    //     console.log("    shown_stores:");
    //     self.shown_stores.print_state(8);
    //     console.log("    expansion_stores:");
    //     self.expansion_stores.print_state(8);
    // }
}
// function load_store_table<T>(value_table: { [id_str: string]: T }): {
//     [id: number]: Writable<T>;
// } {
//     let store_table: { [id: number]: Writable<T> } = {};
//     for (const id_str in value_table.showns) {
//         const id = parseInt(id_str);
//         store_table[id] = writable(value_table[id]);
//     }
//     return store_table;
// }