//use crate::log::Logger;
use crate::routes::linechat;
use gotham::helpers::http::response::create_empty_response;
use gotham::router::builder::{build_simple_router, DefineSingleRoute, DrawRoutes};
use gotham::router::Router;
use gotham::state::State;
use hyper::{Body, Response, StatusCode};

fn hello_world(state: State) -> (State, Response<Body>) {
    let res = create_empty_response(&state, StatusCode::NO_CONTENT);

    (state, res)
}

pub fn router() -> Router {
    //let logger = Logger::new();
    //let local_logger = logger.source_logger.new(o!("layer" => "routes"));

    //let (method, uri) = (req.method(), req.uri());
    //slog::info!(local_logger, "url: {}", uri.path();
    //"method" => format!("{}", method)
    //);

    build_simple_router(|route| {
        route.get_or_head("/").to(hello_world);
        route.post("/linechat").to(linechat::post_handler);
    })
}

//pub fn router() -> Router {
//build_simple_router(|route| {
//route.get_or_head("/").to(hello_world);

////route.scope("/checkout", |route| {
////route.get("/start").to(checkout::start);

////// Associations allow a single path to be matched for multiple HTTP verbs
////// with each delegating to a unique handler or the same handler, as shown here with
////// put and patch.
////route.associate("/address", |assoc| {
////assoc.post().to(checkout::address::create);
////assoc.put().to(checkout::address::update);
////assoc.patch().to(checkout::address::update);
////assoc.delete().to(checkout::address::delete);
////});

////route
////.post("/payment_details")
////.to(checkout::payment_details::create);

////route
////.put("/payment_details")
////.to(checkout::payment_details::update);

////route.post("/complete").to(checkout::complete);
////});

////route.scope("/api", |route| {
////route.get("/products").to(api::products::index);
////});
//})
//}
