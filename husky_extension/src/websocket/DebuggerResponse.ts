import * as t from "io-ts";

import { tFigureProps } from "trace/figure/FigureProps";
import { tTrace } from "src/trace/Trace";
import { tTraceStalk } from "src/trace/stalk/TraceStalk";

const tRootTracesResponse = t.interface({
    type: t.literal("RootTraces"),
    root_traces: t.array(tTrace),
});
const tSubtracesResponse = t.interface({
    type: t.literal("Subtraces"),
    id: t.number,
    input_locked_on: t.union([t.number, t.null]),
    subtraces: t.array(tTrace),
});
const tTraceResponse = t.interface({
    type: t.literal("Trace"),
    trace: tTrace,
});
const tFigureResponse = t.interface({
    type: t.literal("Figure"),
    id: t.number,
    figure: t.union([tFigureProps, t.null]),
});
const tDidActivateResponse = t.interface({
    type: t.literal("DidActivate"),
    id: t.number,
});
const tDidToggleExpansionResponse = t.interface({
    type: t.literal("DidToggleExpansion"),
    id: t.number,
});
const tDidToggleShowResponse = t.interface({
    type: t.literal("DidToggleShow"),
    id: t.number,
});
const tDidLockInputResponse = t.interface({
    type: t.literal("DidLockInput"),
    input_locked_on: t.union([t.number, t.null, t.undefined]),
    message: t.union([t.string, t.null]),
});
const tTraceStalkReponse = t.interface({
    type: t.literal("TraceStalk"),
    id: t.number,
    input_locked_on: t.number,
    stalk: tTraceStalk,
});
export const tDebuggerResponse = t.union([
    tRootTracesResponse,
    tSubtracesResponse,
    tTraceResponse,
    tTraceStalkReponse,
    tFigureResponse,
    tDidActivateResponse,
    tDidToggleExpansionResponse,
    tDidToggleShowResponse,
    tDidLockInputResponse,
]);

type DebuggerResponse = t.TypeOf<typeof tDebuggerResponse>;
export default DebuggerResponse;