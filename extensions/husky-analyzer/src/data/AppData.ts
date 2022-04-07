import type InitData from "./InitData";
import TraceCache from "./TraceCache";
import FigureCache from "./FigureCache";
import UserState from "./UserState";
import type { Writable } from "svelte/store";
import { writable, get } from "svelte/store";
import type Trace from "src/trace/Trace";
import type Focus from "./Focus";

class AppData {
    trace_cache: TraceCache = new TraceCache();
    figure_cache: FigureCache = new FigureCache();
    user_state = new UserState();
    trace_listing_store: Writable<number[]> = writable([]);

    init(init_state: InitData) {
        this.trace_cache.init(init_state);
        this.figure_cache.init(init_state);
        this.user_state.init(init_state);
    }

    update_trace_listing() {
        let root_traces = get(this.trace_cache.root_traces_store);
        if (root_traces === null) {
            this.trace_listing_store.set([]);
        } else {
            let trace_listing: number[] = [];
            for (const trace of root_traces) {
                this.update_trace_listing_dfs(trace, trace_listing);
            }
            this.trace_listing_store.set(trace_listing);
        }
    }

    focus(): Focus {
        return get(this.user_state.focus_store);
    }

    update_trace_listing_dfs(trace: Trace, trace_listing: number[]) {
        this.add_associated_traces(trace.id, trace_listing);
        if (this.user_state.is_expanded(trace.id)) {
            let subtraces = this.trace_cache.get_subtraces(this.focus(), trace);
            if (subtraces !== null) {
                for (const trace of subtraces) {
                    this.update_trace_listing_dfs(trace, trace_listing);
                }
            }
        }
    }

    add_associated_traces(id: number, trace_listing: number[]) {
        let trace = this.trace_cache.get_trace(id);
        if (trace !== null) {
            for (const line of trace.lines) {
                let tokens = line.tokens;
                for (const token of tokens) {
                    let associated_trace_id = token.associated_trace;
                    if (associated_trace_id !== null) {
                        if (this.user_state.is_shown(associated_trace_id)) {
                            trace_listing.push(associated_trace_id);
                        }
                    }
                }
            }
        }
    }
}
export default AppData;

// export function get_id_before(id: number): number | undefined {
//     let trace_listing = get(trace_listing_store);
//     return trace_listing[trace_listing.indexOf(id) - 1];
// }

// export function get_id_after(id: number): number | undefined {
//     let trace_listing = get(trace_listing_store);
//     return trace_listing[trace_listing.indexOf(id) - 1];
// }