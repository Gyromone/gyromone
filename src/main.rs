use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use routes::root;

mod config;
mod routes;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config = config::parse_config();

    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], config.server.port));
        

    let service = make_service_fn(|_| async {
        // service_fn converts our function into a `Service`
        Ok::<_, hyper::Error>(service_fn(root::handler))
    });

    let server = Server::bind(&addr).serve(service);

    server.await?;

    Ok(())
}


