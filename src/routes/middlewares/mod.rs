use gotham::handler::HandlerFuture;
use gotham::middleware::Middleware;
use gotham::state::{FromState, State};
use hyper::{Method, Uri, Version};

#[derive(NewMiddleware, Clone)]
pub struct LoggerMiddleware {
    pub logger: slog::Logger,
}

impl Middleware for LoggerMiddleware {
    fn call<Chain>(self, state: State, chain: Chain) -> Box<HandlerFuture>
    where
        Chain: FnOnce(State) -> Box<HandlerFuture> + Send + 'static,
    {
        let uri = Uri::borrow_from(&state);
        let method = Method::borrow_from(&state);
        let http_version = Version::borrow_from(&state);

        slog::info!(
            self.logger,
            "request in";
            "http" => format!("{:?}", http_version),
            "method" => format!("{:?}", method),
            "uri" => format!("{:?}", uri)
        );

        chain(state)
    }
}
