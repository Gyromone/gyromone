extern crate gyromone;

mod support;

#[cfg(test)]
mod tests {
    use crate::support;

    use hyper::StatusCode;

    #[test]
    fn healthy_check() {
        let test_server = support::start_test_server();
        let uri = support::bind_uri("");

        let response = test_server.client().get(uri).perform().unwrap();
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }
}
