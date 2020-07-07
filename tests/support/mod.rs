use gyromone::routes;

use gotham::test::TestServer;

pub fn bind_uri(endpoint: &'static str) -> String {
    let addr = String::from("localhost");
    let root_uri = format!("http://{}", addr);
    let uri = format!("{}/{}", root_uri, endpoint);

    uri
}

pub fn start_test_server() -> TestServer {
    let router = routes::router();
    TestServer::new(router).unwrap()
}
