use mould::workers::Worker;
use mould::session::SessionData;

pub struct Session;

impl Default for Session {
    fn default() -> Self {
        Session
    }
}

impl SessionData for Session {
}

pub type SessionWorker = Box<Worker<Session>>;
