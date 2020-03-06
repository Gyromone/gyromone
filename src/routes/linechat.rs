use serde::{Deserialize, Serialize};
use serde_json::from_slice;
use hyper::{body, Body, Request, Response};

#[derive(Debug, Serialize, Deserialize)]
struct ReqBody {
    destination: String
}

pub async fn handler(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let _req_body = _req.into_body();
    let bytes = body::to_bytes(_req_body).await.unwrap();
    let s: ReqBody = from_slice(&bytes).unwrap();
    println!("{:?}", s);
    Ok(Response::new("line message".into()))
}
