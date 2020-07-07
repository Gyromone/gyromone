use gyromone::routes;

use gotham::test::TestServer;

pub fn start_test_server() -> TestServer {
    let router = routes::router();
    TestServer::new(router).unwrap()
}
