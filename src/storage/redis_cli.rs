use crate::common::map_dumming_error;
use crate::config;
use crate::log::Logger;

use r2d2::Pool;
use redis::{Client, IntoConnectionInfo};
use slog;

pub struct Redis {
    pub pool: Pool<Client>,
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
            .map_err(map_dumming_error)
            .and_then(|info| Client::open(info).map_err(map_dumming_error))
            .and_then(|client| {
                Pool::builder()
                    .max_size(15)
                    .build(client)
                    .map_err(map_dumming_error)
            }) {
            Ok(pool) => {
                slog::info!(
                    local_logger,
                    "{}", "new connection pool";
                );

                Redis { pool: pool }
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
