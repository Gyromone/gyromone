#[macro_use]
extern crate slog;
extern crate slog_async;
#[macro_use]
extern crate lazy_static;

extern crate slog_term;
extern crate sha2;

use std::net::SocketAddr;
use hyper::{Server};
use hyper::service::{make_service_fn, service_fn};
use routes::root;
use log::Logger;

mod config;
mod routes;
mod log;
mod constants;
mod response;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let conf = &*config::SYSTEM_CONFIG;
    let logger = Logger::new();

    let addr = SocketAddr::from(([127, 0, 0, 1], conf.server.port));

    slog::info!(
        logger.source_logger,
        "{}", conf.server.port.to_string();
        "feature" => "main"
    );
        

    let service = make_service_fn(|_| async {
        // service_fn converts our function into a `Service`
        Ok::<_, hyper::Error>(service_fn(root::handler))
    });

    let server = Server::bind(&addr).serve(service);

    server.await?;

    Ok(())
}


