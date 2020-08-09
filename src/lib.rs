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

use log::Logger;
use storage::queue::Queue;
use storage::redis_cli;
use threadpool::ThreadPool;

use std::thread::sleep;
use std::time::Duration;

mod common;
pub mod config;
mod constants;
mod external;
mod log;
mod response;
pub mod routes;
mod storage;
mod threadpool;

pub fn run_server(addr: String) {
    let _redis = redis_cli::Redis::new();
    let conf = &*config::SYSTEM_CONFIG;

    // try queue start
    let queue = Queue::new(_redis);
    queue.push("queue:test1", "I'm a message".to_string());

    match queue.pop::<String>("queue:test1") {
        Some(v) => println!("queue value 1 {}", v),
        None => println!("queue value 1, no value"),
    }
    match queue.pop::<String>("queue:test1") {
        Some(v) => println!("queue value 2 {}", v),
        None => println!("queue value 2, no value"),
    };
    // try queue end

    // try threadpool start
    let pool = ThreadPool::new(5);
    for i in 1..11 {
        pool.execute(move || {
            sleep(Duration::from_secs(3));
            println!("do {}", i);
        });
    }
    // try threadpool end

    let logger = Logger::new();
    let local_logger = logger.source_logger.new(o!("func" => "main"));

    slog::info!(
        local_logger,
        "{}", conf.server.port.to_string();
    );

    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, routes::router())
}
