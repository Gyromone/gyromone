use std::str::FromStr;

use crate::config;
use crate::external;
use crate::log::Logger;

use futures::future;
use futures::future::Future;
use hyper::client;
use hyper::{header, Body, Method, Request};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Action {
    #[serde(rename = "type")]
    _type: String,
    label: String,
    text: String,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct QuickReplyObject {
    #[serde(rename = "type")]
    _type: String,
    action: Action,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
enum Message {
    QuickReplyTextObject {
        #[serde(rename = "type")]
        _type: String,
        text: String,
        quick_reply: Vec<QuickReplyObject>,
    },
}

#[derive(PartialEq, Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ReplyReqBody {
    reply_token: String,
    messages: Vec<Message>,
}

pub fn reply_message_future(
    reply_token: &String,
    reply_text: &String,
) -> impl Future<Item = client::ResponseFuture, Error = &'static str> {
    let logger = Logger::new();
    let local_logger = logger
        .source_logger
        .new(o!("func" => "reply_message_future"));

    let conf = &config::SYSTEM_CONFIG;

    let reply_endpoint = &conf.line_chat.reply_endpoint;
    let token = &conf.line_chat.channel_token;

    let method = match Method::from_str(&reply_endpoint.method) {
        Ok(m) => m,
        Err(err_msg) => {
            slog::debug!(
                local_logger,
                "{}", err_msg;
            );
            return future::err("error occurs while method mapping");
        }
    };

    let reply_body = ReplyReqBody {
        reply_token: reply_token.to_string(),
        messages: vec![Message::QuickReplyTextObject {
            _type: "text".to_string(),
            text: reply_text.to_string(),
            quick_reply: vec![QuickReplyObject {
                _type: "action".to_string(),
                action: Action {
                    _type: "message".to_string(),
                    label: "try me please!".to_string(),
                    text: "try_me".to_string(),
                },
            }],
        }],
    };
    let reply_bytes = serde_json::to_vec(&reply_body).unwrap();
    let client = &external::HTTP_CLIENT;
    let req = Request::builder()
        .method(method)
        .uri(&reply_endpoint.endpoint)
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .body(Body::from(reply_bytes))
        .expect("request builder");

    println!("{:?}", req);

    let f = client.request(req);
    future::ok(f)
}
