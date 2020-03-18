#[macro_use]
extern crate slog;
extern crate slog_async;

extern crate slog_term;

use std::net::SocketAddr;
use hyper::{Server};
use hyper::service::{make_service_fn, service_fn};
use routes::root;
use log::Logger;

mod config;
mod routes;
mod log;
mod constants;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config = config::parse_config();
    let logger = Logger::new();

    let addr = SocketAddr::from(([127, 0, 0, 1], config.server.port));

    slog::info!(
        logger.source_logger,
        "{}", config.server.port.to_string();
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


