use crate::config;
use crate::constants;
use crate::external::line;
use crate::log::Logger;
use crate::response::errors::Errors;
use crate::response::successes::SuccessResponse;
use base64;
use futures::{future, Future, Stream};
use gotham::handler::{HandlerFuture, IntoHandlerError};
use gotham::state::{FromState, State};
use hmac::{Hmac, Mac};
use hyper::rt::run;
use hyper::{body, Body, HeaderMap, StatusCode};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

use std::sync::Arc;
use std::thread;

const TYPE_MESSAGE: &'static str = "message";

#[derive(PartialEq, Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Message {
    id: String,
    #[serde(rename = "type")]
    _type: String,
    text: Option<String>,
}

#[derive(PartialEq, Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Event {
    #[serde(rename = "type")]
    _type: String,
    message: Option<Message>,
    reply_token: String,
}

#[derive(PartialEq, Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LineReqBody {
    destination: Option<String>,
    events: Vec<Event>,
}

#[derive(PartialEq, Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LineResp {
    message: String,
}

impl LineReqBody {
    fn borrow_message_event(&self) -> Option<&Event> {
        self.events
            .iter()
            .find(|event| event._type == String::from(TYPE_MESSAGE))
    }
}

fn borrow_message_reply_token(event: Option<&Event>) -> Option<&String> {
    match event {
        Some(event) => Some(&event.reply_token),
        None => None,
    }
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
                let conf = &config::SYSTEM_CONFIG;
                let secret = &conf.line_chat.secret;
                let should_verify_linechat_secret = &conf.debug.should_verify_linechat_secret;
                let bytes = body::Chunk::into_bytes(valid_body);

                let signature = match headers
                    .get(constants::LINE_SIGNATURE_KEY)
                    .ok_or(String::from("fail to get signature header"))
                    .and_then(|key| key.to_str().map_err(|e| format!("error code: {}", e)))
                {
                    Ok(s) => s,
                    Err(err_msg) => {
                        slog::error!(
                            local_logger,
                            "{}", err_msg;
                        );
                        return Errors::GeneralUnauthorized.into_future_result(state);
                    }
                };

                let is_valid = !should_verify_linechat_secret
                    || match verify(&bytes, signature, secret.as_bytes()) {
                        Ok(iv) => iv,
                        Err(err_msg) => {
                            slog::error!(
                                local_logger,
                                "{}", err_msg;
                            );
                            return Errors::GeneralSystemError.into_future_result(state);
                        }
                    };

                match is_valid {
                    true => {
                        let req_body: LineReqBody = match serde_json::from_slice(&bytes) {
                            Ok(body) => body,
                            Err(err_msg) => {
                                slog::error!(
                                    local_logger,
                                    "{}", err_msg;
                                );
                                return Errors::GeneralWrongRequest.into_future_result(state);
                            }
                        };

                        let success = SuccessResponse {
                            status_code: StatusCode::OK,
                            value: LineResp {
                                message: String::from("success!"),
                            },
                        };
                        let resp = success.into_future_result(state);
                        thread::spawn(move || {
                            let logger = Logger::new();
                            let local_logger =
                                Arc::new(logger.source_logger.new(o!("func" => "reply worker")));

                            let reply_token =
                                match borrow_message_reply_token(req_body.borrow_message_event()) {
                                    Some(r) => r,
                                    None => {
                                        slog::error!(
                                            local_logger.clone(),
                                            "{}",
                                            "can't grab the reply token"
                                        );
                                        return ();
                                    }
                                };

                            let f = line::reply_message_future(
                                reply_token,
                                &String::from("hello from rust"),
                            );

                            let handled_f = f
                                .map({
                                    let l_logger = local_logger.clone();
                                    move |_| {
                                        let message = "Done.";
                                        slog::info!(
                                            l_logger,
                                            "{}", message;
                                        );
                                    }
                                })
                                // If there was an error, let the user know...
                                .map_err({
                                    let l_logger = local_logger.clone();

                                    move |err| {
                                        slog::debug!(
                                            l_logger,
                                            "{}", err;
                                        );
                                    }
                                });

                            run(handled_f);
                        });
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_req_body_borrow_message_event() {
        let non_message_event = Event {
            _type: String::from("follow"),
            ..Default::default()
        };

        let message_event = Event {
            _type: String::from(TYPE_MESSAGE),
            ..Default::default()
        };

        let mut line_req_body: LineReqBody = Default::default();
        line_req_body
            .events
            .append(&mut vec![non_message_event, message_event]);
        assert!(line_req_body.borrow_message_event().is_some())
    }

    #[test]
    fn test_line_req_body_borrow_message_event_no_message_event() {
        let line_req_body: LineReqBody = Default::default();
        assert!(line_req_body.borrow_message_event().is_none())
    }

    #[test]
    fn test_borrow_message_reply_token() {
        let expected_reply_token = String::from("token");
        let message_event = Event {
            _type: String::from(TYPE_MESSAGE),
            reply_token: expected_reply_token,
            ..Default::default()
        };

        assert_eq!(
            borrow_message_reply_token(Some(&message_event)),
            Some(&String::from("token"))
        )
    }
}
