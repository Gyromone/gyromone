use hyper::{Body, Response};
use serde_json;

pub fn line_chat_resp_builder(raw_resp: Result<(), errors::Errors>) -> Response<Body> {
    match raw_resp {
        Ok(()) => Response::builder().body(Body::from("good!")).unwrap(),
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
