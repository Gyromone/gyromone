use crate::storage::redis_cli;

use redis::Commands;

pub struct Queue {
    pub redis: redis_cli::Redis,
}

const POP_TIMEOUT: u8 = 1;

impl Queue {
    pub fn new(redis: redis_cli::Redis) -> Self {
        Queue { redis: redis }
    }

    pub fn push(&self, key: &'static str, value: String) {
        let mut conn = self.redis.pool.get().unwrap();
        let _: () = conn.rpush(key, value).unwrap();
    }

    pub fn pop(&self, key: &'static str) -> Option<String> {
        let mut conn = self.redis.pool.get().unwrap();
        match conn
            .blpop::<&str, Vec<String>>(key, POP_TIMEOUT as usize)
            .unwrap()
            .as_slice()
        {
            [_, v] => Some(v.clone()),
            _ => None,
        }
    }
}
