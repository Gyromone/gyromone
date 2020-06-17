pub mod linechat;
pub mod middlewares;

use crate::log::Logger;
use gotham::helpers::http::response::create_empty_response;
use gotham::pipeline::new_pipeline;
use gotham::pipeline::single::single_pipeline;
use gotham::router::builder::{build_router, DefineSingleRoute, DrawRoutes};
use gotham::router::Router;
use gotham::state::State;
use hyper::{Body, Response, StatusCode};
use middlewares::LoggerMiddleware;

fn hello_world(state: State) -> (State, Response<Body>) {
    let res = create_empty_response(&state, StatusCode::NO_CONTENT);

    (state, res)
}

pub fn router() -> Router {
    let logger = Logger::new();
    let local_logger = logger.source_logger.new(o!());

    let logger_middleware = LoggerMiddleware {
        logger: local_logger,
    };
    let (chain, pipelines) = single_pipeline(new_pipeline().add(logger_middleware).build());
    build_router(chain, pipelines, |route| {
        route.get_or_head("/").to(hello_world);
        route.post("/linechat").to(linechat::post_handler);
    })
}
