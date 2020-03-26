use hyper::{Body, Response};
use serde_json;

pub fn line_chat_resp_builder(raw_resp: Result<(), errors::Errors>) -> Response<Body> {
    match raw_resp {
        Ok(()) => {
            let payload = success::Success::<()> { payload: () };
            Response::builder()
                .body(Body::from(serde_json::to_string(&payload).unwrap()))
                .unwrap()
        }
        Err(e) => {
            let errors::ErrorParts {
                status_code,
                payload,
            } = e.to_parts();

            Response::builder()
                .status(status_code)
                .body(Body::from(serde_json::to_string(&payload).unwrap()))
                .unwrap()
        }
    }
}

pub mod errors;
pub mod success;
