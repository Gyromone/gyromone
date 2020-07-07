extern crate gyromone;

mod support;

#[cfg(test)]
mod tests {
    use crate::support;

    use hyper::StatusCode;

    #[test]
    fn healthy_check() {
        let addr = String::from("localhost");
        let root_uri = format!("http://{}", addr);
        let uri = format!("{}/{}", root_uri, "");

        let test_server = support::start_test_server();

        let response = test_server.client().get(uri).perform().unwrap();
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }
}
