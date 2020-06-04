use futures::future;
use futures::future::FutureResult;
use gotham::handler::HandlerError;
use gotham::state::State;
use hyper::{Body, Response, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::to_vec;

pub enum Errors {
    GeneralSystemError,
    GeneralWrongRequest,
    GeneralUnauthorized,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorPayload {
    pub code: u16,
    pub message: String,
}

#[derive(Debug)]
pub struct ErrorParts {
    pub status_code: StatusCode,
    pub payload: ErrorPayload,
}

impl Errors {
    pub fn into_future_result(
        self,
        _state: State,
    ) -> FutureResult<(State, Response<Body>), (State, HandlerError)> {
        let resp = self.into_response(&_state);
        future::ok((_state, resp))
    }

    fn into_response(self, state: &State) -> Response<Body> {
        let ErrorParts {
            status_code,
            payload,
        } = self.to_parts();

        match to_vec(&payload) {
            Ok(v) => {
                let resp = Response::builder()
                    .status(status_code)
                    .body(v.into())
                    .unwrap();
                resp
            }
            Err(_) => Errors::GeneralSystemError.into_response(state),
        }
    }
}

impl Errors {
    pub fn to_parts(&self) -> ErrorParts {
        match &self {
            Errors::GeneralSystemError => ErrorParts {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                payload: ErrorPayload {
                    code: 1,
                    message: String::from("something wrong in system"),
                },
            },
            Errors::GeneralWrongRequest => ErrorParts {
                status_code: StatusCode::BAD_REQUEST,
                payload: ErrorPayload {
                    code: 2,
                    message: String::from("wrong request"),
                },
            },
            Errors::GeneralUnauthorized => ErrorParts {
                status_code: StatusCode::UNAUTHORIZED,
                payload: ErrorPayload {
                    code: 3,
                    message: String::from("signature is unauthorized"),
                },
            },
        }
    }
}
