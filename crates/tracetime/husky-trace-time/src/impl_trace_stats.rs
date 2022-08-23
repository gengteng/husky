use crate::*;
use husky_entity_route::EntityRoutePtr;
use husky_text::HuskyText;
use husky_vm::{ControlSnapshot, History, VMControl};
use husky_word::RootIdentifier;

impl HuskyTraceTime {
    pub(crate) fn collect_new_trace_statss(&mut self) -> Vec<(TraceStatsKey, Option<TraceStats>)> {
        let mut trace_statss = Vec::new();
        for root_trace_id in self.root_trace_ids.clone() {
            self.collect_new_trace_statss_within_trace(root_trace_id, &mut trace_statss);
        }
        trace_statss
    }

    fn collect_new_trace_statss_within_trace(
        &mut self,
        trace_id: TraceId,
        trace_statss: &mut Vec<(TraceStatsKey, Option<TraceStats>)>,
    ) {
        let trace_node_data = self.trace_node_data(trace_id);
        let expanded = trace_node_data.expanded;
        let trace_raw_data = &trace_node_data.trace_data;
        let trace_stats_key = TraceStatsKey {
            trace_id,
            partitions: self.restriction.partitions().clone(),
        };
        let associated_trace_ids = trace_raw_data.associated_trace_ids();
        if !self.trace_statss.contains_key(&trace_stats_key) {
            let opt_trace_stats = self.produce_trace_stats(trace_id);
            self.trace_statss
                .insert(trace_stats_key.clone(), opt_trace_stats.clone());
            trace_statss.push((trace_stats_key, self.produce_trace_stats(trace_id)))
        }
        for associated_trace_id in associated_trace_ids {
            self.collect_new_trace_statss_within_trace(associated_trace_id, trace_statss)
        }
        if expanded {
            for subtrace_id in self.subtrace_ids(trace_id) {
                self.collect_new_trace_statss_within_trace(subtrace_id, trace_statss)
            }
        }
    }

    fn produce_trace_stats(&mut self, trace_id: TraceId) -> Option<TraceStats> {
        self.trace(trace_id)
            .variant
            .opt_stats(self.runtime(), self.restriction.partitions())
            .expect("todo")
    }
}