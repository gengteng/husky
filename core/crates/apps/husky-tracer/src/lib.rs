mod config;
mod error;
pub mod flags;
mod gui;
mod internal;
mod mode;
mod notif;

pub use error::{DebuggerError, DebuggerResult};
use husky_trace_time::HuskyTraceTime;
pub use mode::Mode;

use avec::Avec;
use config::DebuggerConfig;
use futures::executor::ThreadPool;
use gui::handle_query;
use husky_compile_time::HuskyCompileTime;
use husky_debugger_protocol::*;
use internal::HuskyTracerInternal;
use json_result::JsonResult;
use notif::handle_notif;
use print_utils::*;
use std::sync::Mutex;
use std::{collections::HashMap, convert::Infallible, net::ToSocketAddrs, sync::Arc};
use test_utils::TestResult;
use warp::Filter;

pub struct HuskyTracer {
    internal: Mutex<HuskyTracerInternal>,
    threadpool: ThreadPool,
}

impl HuskyTracer {
    pub fn new(init_compile_time: impl FnOnce(&mut HuskyCompileTime)) -> Self {
        let config = DebuggerConfig::from_env();
        let mut trace_time = HuskyTraceTime::new(init_compile_time, config.verbose);
        if let Some(ref input_id_str) = config.opt_input_id {
            match trace_time.lock_input(input_id_str) {
                (_, Some(msg)) => panic!("{}", msg),
                (Some(Some(input_id)), None) => {
                    for trace in trace_time.root_traces().iter() {
                        let stalk = trace_time.trace_stalk(*trace, input_id);
                    }
                }
                _ => (),
            }
        }
        Self {
            internal: Mutex::new(HuskyTracerInternal { trace_time, config }),
            threadpool: ThreadPool::new().unwrap(),
        }
    }

    pub async fn serve_on_error(self, addr: impl ToSocketAddrs, input_id: usize) -> TestResult {
        if self.has_root_error(input_id).await {
            self.serve(addr).await.unwrap();
            TestResult::Failed
        } else {
            TestResult::Success
        }
    }

    async fn has_root_error(&self, input_id: usize) -> bool {
        let mut error_flag = false;
        let internal = self.internal.lock().unwrap();
        for trace in internal.trace_time.root_traces().iter() {
            let stalk = internal.trace_time.trace_stalk(*trace, input_id);
            for token in &stalk.extra_tokens {
                match token.kind {
                    TraceTokenKind::Error => {
                        error_flag = true;
                        break;
                    }
                    _ => (),
                }
            }
        }
        error_flag
    }

    pub async fn serve(self, addr: impl ToSocketAddrs) -> DebuggerResult<()> {
        let debugger = Arc::new(self);
        let addr = addr.to_socket_addrs().unwrap().next().unwrap();
        println!(
            "{}husky{}: serve on {:?}",
            print_utils::CYAN,
            print_utils::RESET,
            addr
        );
        let notif = warp::path!("notif")
            .and(warp::ws())
            .and(with_debugger(debugger.clone()))
            .and_then(handle_notif);
        let query = warp::path!("query")
            .and(warp::ws())
            .and(with_debugger(debugger.clone()))
            .and_then(handle_query);
        let routes = notif.or(query);
        warp::serve(routes).run(addr).await;
        Ok(())
    }

    pub fn change_text(&self) {
        todo!()
    }
}

fn with_debugger(
    debugger: Arc<HuskyTracer>,
) -> impl Filter<Extract = (Arc<HuskyTracer>,), Error = Infallible> + Clone {
    warp::any().map(move || debugger.clone())
}