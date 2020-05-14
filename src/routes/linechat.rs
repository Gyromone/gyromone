use crate::config;
use crate::constants;
use crate::response;
use crate::response::errors::Errors;
use base64;
use hmac::{Hmac, Mac};
use hyper::{body, Body, Request, Response};
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

fn verify(message: &[u8], code: &str, key: &[u8]) -> bool {
    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_varkey(key).unwrap();
    mac.input(message);

    let result = mac.result().code();
    let r2 = base64::encode(&result);

    r2 == code
}

//pub async fn handler(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
//let conf = &*config::SYSTEM_CONFIG;
//let secret = &conf.line_chat.secret;

//let (parts, _req_body) = _req.into_parts();
//let bytes = body::Chunk::into_bytes(_req_body).await.unwrap();
//let s: LineReqBody = from_slice(&bytes).unwrap();

//let header = parts.headers;

//// handle error, no unwrap here
//let signature = header
//.get(constants::LINE_SIGNATURE_KEY)
//.unwrap()
//.to_str()
//.unwrap();

//let is_valid = verify(&bytes, signature, secret.as_bytes());

//match is_valid {
//true => Ok(response::line_chat_resp_builder(Ok(()))),
//false => Ok(response::line_chat_resp_builder(Err(
//Errors::GeneralUnauthorized,
//))),
//}
//}
