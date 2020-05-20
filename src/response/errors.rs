use gotham::handler::{HandlerError, IntoHandlerError};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::fmt::Display;

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

impl Display for ErrorPayload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.code, self.message)
    }
}

impl Error for ErrorPayload {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}

impl IntoHandlerError for Errors {
    fn into_handler_error(self) -> HandlerError {
        let ErrorParts {
            status_code,
            payload,
        } = self.to_parts();

        payload.into_handler_error().with_status(status_code)

        //HandlerError {
        //status_code: status_code,
        //cause: Box::new(payload),
        //}
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
