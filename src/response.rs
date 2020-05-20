use gotham::handler::IntoResponse;
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
    pub fn into_result_response(self, _state: &State) -> Result<Response<Body>, Box<Error>> {
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
}

//impl<T: Serialize> SuccessFuture<_, T> {
//fn into_future_result(self) -> FutureResult<(&State, Response<Body>), (&State, HandlerError)> {
//match to_vec(&self.value) {
//Ok(v) => future::ok((
//self.state,
//Response::builder()
//.status(StatusCode::OK)
//.body(v.into())
//.unwrap(),
//)),
//}
//}
//}

pub mod errors;
