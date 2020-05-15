use crate::config;
use crate::constants;
use crate::response::errors::Errors;
use crate::response::SuccessResponse;
use base64;
use futures::{future, Future, Stream};
use gotham::handler::{HandlerFuture, IntoHandlerError, IntoResponse};
use gotham::helpers::http::response::create_empty_response;
use gotham::state::{FromState, State};
use hmac::{Hmac, Mac};
use hyper::{body, Body, HeaderMap, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::from_slice;
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

fn verify(message: &[u8], code: &str, key: &[u8]) -> bool {
    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_varkey(key).unwrap();
    mac.input(message);

    let result = mac.result().code();
    let r2 = base64::encode(&result);

    r2 == code
}

pub fn handler(mut state: State) -> Box<HandlerFuture> {
    let f = Body::take_from(&mut state)
        .concat2()
        .then(|full_body| match full_body {
            Ok(valid_body) => {
                let headers = HeaderMap::borrow_from(&state);
                let conf = &*config::SYSTEM_CONFIG;
                let secret = &conf.line_chat.secret;
                let bytes = body::Chunk::into_bytes(valid_body);
                let s: LineReqBody = from_slice(&bytes).unwrap();

                // handle error, no unwrap here
                let signature = headers
                    .get(constants::LINE_SIGNATURE_KEY)
                    .unwrap()
                    .to_str()
                    .unwrap();

                let is_valid = verify(&bytes, signature, secret.as_bytes());

                match is_valid {
                    true => {
                        let success = SuccessResponse {
                            status_code: StatusCode::OK,
                            value: LineResp {
                                message: String::from("success!"),
                            },
                        };
                        let resp = success.into_response(&state);
                        future::ok((state, resp))
                        //let res = create_empty_response(&state, StatusCode::OK);
                        //future::ok((state, res))
                    }
                    false => future::err((state, Errors::GeneralUnauthorized.into_handler_error())),
                }
                //let body_content = String::from_utf8(valid_body.to_vec()).unwrap();
                //println!("Body: {}", body_content);
                //let res = create_empty_response(&state, StatusCode::OK);
                //future::ok((state, res))
            }
            Err(e) => future::err((state, e.into_handler_error())),
        });

    Box::new(f)
}
