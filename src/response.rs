use gotham::handler::IntoResponse;
use gotham::state::State;
use hyper::{Body, Response, StatusCode};
use serde::ser::Serialize;
use serde_json::to_vec;

pub struct SuccessResponse<T: Serialize> {
    pub status_code: StatusCode,
    pub value: T,
}

impl<T: Serialize> IntoResponse for SuccessResponse<T> {
    fn into_response(self, _state: &State) -> Response<Body> {
        match to_vec(&self.value) {
            Ok(v) => Response::builder()
                .status(&self.status_code)
                .body(v.into())
                .unwrap(),
            Err(e) => Response::builder()
                .status(&self.status_code)
                .body(String::from("error!!!").into())
                .unwrap(),
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
