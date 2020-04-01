use hyper::{Body, Method, Request, Response, StatusCode};
use crate::routes::linechat;
use crate::log::Logger;

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    Ok(Response::new("Hello, World".into()))
}

pub async fn handler(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let logger = Logger::new();
    let local_logger = logger.source_logger.new(o!("layer" => "routes"));

    let (method, uri) = (req.method(), req.uri());
    slog::info!(local_logger, "url: {}", uri.path();
            "method" => format!("{}", method) 
        );

    match (method, uri.path()) {
        (&Method::GET, "/") => hello_world(req).await,
        (&Method::POST, "/linechat") => linechat::handler(req).await,
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}
