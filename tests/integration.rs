extern crate gyromone;

mod support;

#[cfg(test)]
mod tests {
    use crate::support;

    use hyper::StatusCode;
    use mime;
    use serde_json::json;

    #[test]
    fn healthy_check() {
        let test_server = support::start_test_server();
        let uri = support::bind_uri("");

        let response = test_server.client().get(uri).perform().unwrap();
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }

    #[test]
    fn linechat_no_header() {
        let test_server = support::start_test_server();
        let uri = support::bind_uri("linechat");

        let response = test_server
            .client()
            .post(uri, "{}", mime::APPLICATION_JSON)
            .perform()
            .unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn linechat_invalid_body() {
        let test_server = support::start_test_server();
        let uri = support::bind_uri("linechat");

        let response = test_server
            .client()
            .post(uri, "{}", mime::APPLICATION_JSON)
            .with_header("X-Line-Signature", "".parse().unwrap())
            .perform()
            .unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn linechat_happy_path() {
        let test_server = support::start_test_server();
        let uri = support::bind_uri("linechat");

        let body = json!({
        "destination": "foo",
        "events": [
            {
                "replyToken": "0f3779fba3b349968c5d07db31eab56f",
                "type": "message",
                "mode": "active",
                "timestamp": 1462629479859 as u64,
                "source": {
                    "type": "user",
                    "userId": "U4af4980629..."
                },
                "message": {
                    "id": "325708",
                    "type": "text",
                    "text": "Hello, world"
                }
            }
        ]});

        let response = test_server
            .client()
            .post(uri, body.to_string(), mime::APPLICATION_JSON)
            .with_header("X-Line-Signature", "".parse().unwrap())
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
