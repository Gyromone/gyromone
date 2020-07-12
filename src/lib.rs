#[macro_use]
extern crate slog;
extern crate slog_async;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate gotham_derive;
extern crate redis;

extern crate gotham;
extern crate hyper;
extern crate hyper_tls;
extern crate r2d2;
extern crate sha2;
extern crate slog_term;

use crate::redis::Commands;

use log::Logger;
use storage::redis_cli;

mod common;
pub mod config;
mod constants;
mod external;
mod log;
mod response;
pub mod routes;
mod storage;

pub fn run_server(addr: String) {
    let _redis = redis_cli::Redis::new();
    let conf = &*config::SYSTEM_CONFIG;

    // try redis
    let mut conn = _redis.pool.get().unwrap();
    let _: () = conn.set("my_key", 42).unwrap();
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.
    let value: isize = conn.get("my_key").unwrap();
    println!("reids value: {}", value);
    // try redis end

    let logger = Logger::new();
    let local_logger = logger.source_logger.new(o!("func" => "main"));

    slog::info!(
        local_logger,
        "{}", conf.server.port.to_string();
    );

    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, routes::router())
}
