pub(crate) mod client_comm;
pub(crate) mod event_loop_comm;

use crossbeam_channel::Sender;

use client_comm::ClientCommunicator;
use event_loop_comm::EventLoopCommunicator;

pub(crate) struct Server {
    pub(crate) client_comm: ClientCommunicator,
    pub(crate) event_loop_comm: EventLoopCommunicator,
    pub(crate) db: husky_lang_db::HuskyLangDatabase,
    pub(crate) threadpool: threadpool::ThreadPool,
}

impl Server {
    pub fn new(sender: Sender<lsp_server::Message>) -> Server {
        Server {
            client_comm: ClientCommunicator::new(sender),
            threadpool: threadpool::ThreadPool::default(),
            event_loop_comm: EventLoopCommunicator::default(),
            db: husky_lang_db::HuskyLangDatabase::new(),
        }
    }
}

pub enum TaskSet {
    Nothing,
    Shutdown,
    SendUpdates,
    Respond(lsp_server::Response),
}

impl TaskSet {
    pub fn merge(&mut self, next: TaskSet) {
        match self {
            TaskSet::Nothing => *self = next,
            TaskSet::Shutdown => (),
            TaskSet::SendUpdates => todo!(),
            TaskSet::Respond(_) => todo!(),
        }
    }
}