use session::{Session, SessionWorker};
use mould::handlers::Handler;
use mould::session::{Request, Extractor};
use mould::workers::{Worker, RejectWorker, WorkerResult, WorkerError, Shortcut, Realize};

pub struct FilesHandler;

impl FilesHandler {
	pub fn new() -> Self {
		FilesHandler
	}
}

impl Handler<Session> for FilesHandler {
	fn build(&self, mut request: Request) -> SessionWorker {
		if request.action == "view-list" {
			Box::new(ListWorker::new(request.extract("path")))
		} else {
			Box::new(RejectWorker::new(
				format!("Unknown action {} for files-service!", request.action)))
		}
	}
}

struct ListWorker {
	path: Option<String>,
	list: Option<Vec<String>>,
}

impl ListWorker {
	fn new(path: Option<String>) -> Self {
		ListWorker {
			path: path,
			list: None,
		}
	}
}

impl Worker<Session> for ListWorker {
	fn shortcut(&mut self, _: &mut Session) -> WorkerResult<Shortcut> {
		// TODO Check token
		use std::fs;
		let path = try!(self.path.as_ref().ok_or(
			WorkerError::Reject("Path not set.".to_string())));
		let paths = try!(fs::read_dir(path));
		let mut result = Vec::new(); 
		for path in paths {
			let path = try!(path).path();
			let path = try!(path.to_str().ok_or(
				WorkerError::Reject("Can't convert filename to string".to_string())
				)).to_owned();
			debug!("{:?}", path);
			result.push(path);
		}
		self.list = Some(result);
		Ok(Shortcut::Tuned)
	}
	fn realize(&mut self, _: &mut Session, _: Option<Request>) -> WorkerResult<Realize> {
		Ok(Realize::Done)
	}
}

/*
struct ReadWorker {
	path: Option<String>,
	content: Option<String>,
}

struct WriteWorker {
	path: Option<String>,
	content: Option<String>,
}
*/

