use crate::config;
use crate::log::Logger;

use redis::{Client, Connection, IntoConnectionInfo};
use slog;

pub struct Redis {
    client: Client,
}

impl Redis {
    pub fn new() -> Self {
        let logger = Logger::new();
        let local_logger = logger.source_logger.new(o!("func" => "redis:new"));

        let conf = &config::SYSTEM_CONFIG;
        let redis_uri = &conf.redis.uri;

        match redis_uri
            .clone()
            .into_connection_info()
            .and_then(|info| Client::open(info))
        {
            Ok(client) => {
                slog::info!(
                    local_logger,
                    "{}", "new connection";
                );

                Redis { client: client }
            }
            Err(e) => {
                slog::error!(
                    local_logger,
                    "{}", e;
                );

                panic!(e)
            }
        }
    }
}
