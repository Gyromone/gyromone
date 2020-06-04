use crate::config;
use crate::constants;
use crate::log::Logger;
use crate::response::errors::Errors;
use crate::response::successes::SuccessResponse;
use base64;
use futures::{future, Future, Stream};
use gotham::handler::{HandlerFuture, IntoHandlerError};
use gotham::state::{FromState, State};
use hmac::{Hmac, Mac};
use hyper::{body, Body, HeaderMap, StatusCode};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::str;

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    id: String,
    #[serde(rename = "type")]
    _type: String,
    text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Event {
    #[serde(rename = "type")]
    _type: String,
    message: Message,
}

#[derive(Debug, Serialize, Deserialize)]
struct LineReqBody {
    destination: Option<String>,
    events: Vec<Event>,
}

#[derive(Debug, Serialize, Deserialize)]
struct LineResp {
    message: String,
}

fn verify(message: &[u8], code: &str, key: &[u8]) -> Result<bool, &'static str> {
    type HmacSha256 = Hmac<Sha256>;

    let mut mac = match HmacSha256::new_varkey(key) {
        Ok(m) => m,
        Err(_) => return Err("occur error while verifying"),
    };
    mac.input(message);

    let result = mac.result().code();
    let r2 = base64::encode(&result);

    Ok(r2 == code)
}

pub fn post_handler(mut state: State) -> Box<HandlerFuture> {
    let logger = Logger::new();
    let local_logger = logger
        .source_logger
        .new(o!("func" => "linechat post handler"));

    let f = Body::take_from(&mut state)
        .concat2()
        .then(move |full_body| match full_body {
            Ok(valid_body) => {
                let headers = HeaderMap::borrow_from(&state);
                let conf = &*config::SYSTEM_CONFIG;
                let secret = &conf.line_chat.secret;
                let should_verify_linechat_secret = &conf.debug.should_verify_linechat_secret;
                let bytes = body::Chunk::into_bytes(valid_body);

                let signature = match headers.get(constants::LINE_SIGNATURE_KEY).unwrap().to_str() {
                    Ok(s) => s,
                    Err(_) => return Errors::GeneralWrongRequest.into_future_result(state),
                };

                let is_valid = !should_verify_linechat_secret
                    || match verify(&bytes, signature, secret.as_bytes()) {
                        Ok(iv) => iv,
                        Err(err_msg) => {
                            slog::debug!(
                                local_logger,
                                "{}", err_msg;
                            );
                            return Errors::GeneralSystemError.into_future_result(state);
                        }
                    };

                match is_valid {
                    true => {
                        let success = SuccessResponse {
                            status_code: StatusCode::OK,
                            value: LineResp {
                                message: String::from("success!"),
                            },
                        };
                        let resp = success.into_future_result(state);
                        resp
                    }
                    false => {
                        slog::debug!(
                            local_logger,
                            "{}", "token is not valid";
                        );
                        Errors::GeneralUnauthorized.into_future_result(state)
                    }
                }
            }
            Err(e) => future::err((state, e.into_handler_error())),
        });

    Box::new(f)
}
