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

use std::thread::sleep;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use job_pipeline::{Center, Pipe};
use log::Logger;
use storage::queue::Queue;
use storage::redis_cli;
use threadpool::ThreadPool;

mod common;
pub mod config;
mod constants;
mod external;
mod job_pipeline;
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
    #[derive(Serialize, Deserialize)]
    struct TestPL {};
    let pipe = Pipe {
        topic: "test:topic",
        queue: &queue,
        handler: Box::new(|a: TestPL| println!("handle!")),
    };
    let center = Center::new(vec![pipe], &queue);

    let logger = Logger::new();
    let local_logger = logger.source_logger.new(o!("func" => "main"));

    slog::info!(
        local_logger,
        "{}", conf.server.port.to_string();
    );

    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, routes::router())
}
