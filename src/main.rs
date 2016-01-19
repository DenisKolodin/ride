#[macro_use]
extern crate log;
extern crate env_logger;
extern crate mould;

use mould::server::{self, ServicesMap};

mod session;
mod files;

fn main() {
	env_logger::init().unwrap();

	info!("Starting websocket server...");
	let mut services: ServicesMap<session::Session> = ServicesMap::new();

	let files_serivce = files::FilesHandler::new();
	services.insert("files-service".to_owned(), Box::new(files_serivce));

	server::start("0.0.0.0:12345", services);

}

/*
#[cfg(target_os = "windows")]
fn open_docs(path: &Path) {
    match Command::new("cmd").arg("/C").arg("start").arg("").arg(path).status() {
        Ok(_) => return,
        Err(_) => ()
    };
}
*/

