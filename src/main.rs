extern crate gyromone;

fn main() {
    let conf = &*gyromone::config::SYSTEM_CONFIG;

    let addr = format!("127.0.0.1:{}", conf.server.port);
    gyromone::run_server(addr);
}
