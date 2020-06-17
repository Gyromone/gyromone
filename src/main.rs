#[macro_use]
extern crate slog;
extern crate slog_async;
#[macro_use]
extern crate lazy_static;
extern crate gotham_derive;

extern crate gotham;
extern crate hyper;
extern crate sha2;
extern crate slog_term;

use log::Logger;
use routes::root;

mod config;
mod constants;
mod log;
mod response;
mod routes;

fn main() {
    let conf = &*config::SYSTEM_CONFIG;
    let logger = Logger::new();
    let local_logger = logger.source_logger.new(o!("func" => "main"));

    let addr = format!("127.0.0.1:{}", conf.server.port);

    slog::info!(
        local_logger,
        "{}", conf.server.port.to_string();
    );

    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, root::router());
}
