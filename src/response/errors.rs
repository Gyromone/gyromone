use hyper::StatusCode;
use serde::{Deserialize, Serialize};

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

pub struct ErrorParts {
    pub status_code: StatusCode,
    pub payload: ErrorPayload,
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
