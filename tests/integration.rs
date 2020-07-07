extern crate gyromone;

mod support;

#[cfg(test)]
mod tests {
    use crate::support;

    use hyper::StatusCode;
    use mime;

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
}
