use crate::storage::redis_cli;

use redis::{Commands, FromRedisValue, ToRedisArgs};

pub struct Queue {
    pub redis: redis_cli::Redis,
}

const POP_TIMEOUT: u8 = 1;

impl Queue {
    pub fn new(redis: redis_cli::Redis) -> Self {
        Queue { redis: redis }
    }

    pub fn push<T>(&self, key: &'static str, value: T)
    where
        T: ToRedisArgs,
    {
        let mut conn = self.redis.pool.get().unwrap();
        let _: () = conn.rpush(key, value).unwrap();
    }

    pub fn pop<T>(&self, key: &'static str) -> Option<T>
    where
        T: FromRedisValue + Clone,
    {
        let mut conn = self.redis.pool.get().unwrap();
        let value = match conn
            .blpop::<&str, Vec<T>>(key, POP_TIMEOUT as usize)
            .unwrap()
            .as_slice()
        {
            [_, v] => v.clone(),
            _ => return None,
        };

        return Some(value);
    }
}
