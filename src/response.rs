use futures::future;
use futures::future::FutureResult;
use gotham::handler::{HandlerError, IntoHandlerError};
use gotham::state::State;
use hyper::{Body, Response, StatusCode};
use serde::ser::Serialize;
use serde_json::to_vec;
use std::error::Error;

pub struct SuccessResponse<T: Serialize> {
    pub status_code: StatusCode,
    pub value: T,
}

impl<T: Serialize> SuccessResponse<T> {
    fn into_result_response(self, _state: &State) -> Result<Response<Body>, Box<dyn Error>> {
        match to_vec(&self.value) {
            Ok(v) => {
                let resp = Response::builder()
                    .status(&self.status_code)
                    .body(v.into())
                    .unwrap();

                Ok(resp)
            }
            Err(e) => Err(Box::new(e)),
        }
    }
    pub fn into_future_result(
        self,
        _state: State,
    ) -> FutureResult<(State, Response<Body>), (State, HandlerError)> {
        let resp_result = self.into_result_response(&_state);
        match resp_result {
            Ok(resp) => future::ok((_state, resp)),
            Err(_) => future::err((
                _state,
                errors::Errors::GeneralSystemError.into_handler_error(),
            )),
        }
    }
}

pub mod errors;
