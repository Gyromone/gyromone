extern crate gyromone;

#[cfg(test)]
mod tests {
    use gotham::test::TestServer;
    use gyromone::routes;
    use hyper::StatusCode;

    #[test]
    fn healthy_check() {
        let addr = String::from("localhost");
        let root_uri = format!("http://{}", addr);
        let uri = format!("{}/{}", root_uri, "");

        let test_server = TestServer::new(routes::router()).unwrap();

        let response = test_server.client().get(uri).perform().unwrap();
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }
}
