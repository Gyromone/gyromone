use serde::{Deserialize, Serialize};
use serde_json::from_slice;
use hyper::{Body, Request, Response};
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use base64;
use crate::constants;
use crate::config;

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    id: String,
    #[serde(rename = "type")]
    _type: String,
    text: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Event {
    #[serde(rename = "type")]
    _type: String,
    message: Message,
}

#[derive(Debug, Serialize, Deserialize)]
struct LineReqBody {
    destination: String,
    events: Vec<Event>
}

fn valid_signature(secret: &str, signature: &str) -> bool {
    // create a Sha256 object
    let mut hasher = Sha256::new();

    // write input message
    hasher.input_str(secret);

    // read hash digest
    let mut hex = hasher.result_str();
    hex = base64::encode(hex);
    hex == signature
}

pub async fn handler(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let (parts, _req_body) = _req.into_parts();
    let bytes = hyper::body::to_bytes(_req_body).await.unwrap();
    let s: LineReqBody = from_slice(&bytes).unwrap();
    println!("{:?}", s);

    let header = parts.headers;
    
    // handle error, no unwrap here
    let signature = header.get(constants::LINE_SIGNATURE_KEY).unwrap();
    println!("{}: {:?}", constants::LINE_SIGNATURE_KEY, signature);

    println!("{:?}", header);

    Ok(Response::new("line message".into()))
}
