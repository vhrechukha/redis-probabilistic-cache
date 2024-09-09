use rand::Rng;
use redis::{Commands, Connection, RedisResult};
use std::thread::sleep;
use std::time::Duration;

struct RedisCacheWrapper {
    connection: Connection,
    ttl: usize,
}

impl RedisCacheWrapper {
    pub fn new(redis_url: &str, ttl: usize) -> RedisResult<Self> {
        let client = redis::Client::open(redis_url)?;
        let connection = client.get_connection()?;
        Ok(RedisCacheWrapper { connection, ttl })
    }

    pub fn set(&mut self, key: &str, value: &str) -> RedisResult<()> {
        let mut rng = rand::thread_rng();
        let random_ttl = self.ttl + rng.gen_range(0..self.ttl / 10);
        self.connection
            .set_ex(key, value, random_ttl.try_into().unwrap())?;
        Ok(())
    }

    pub fn get<F>(&mut self, key: &str, regenerate_function: F) -> RedisResult<String>
    where
        F: Fn() -> String,
    {
        let value: Option<String> = self.connection.get(key)?;
        if let Some(val) = value {
            // Probabilistic early expiration
            let mut rng = rand::thread_rng();
            if rng.gen_bool(0.01) {
                self.connection.expire(key, 1)?;
            }
            Ok(val)
        } else {
            let new_value = regenerate_function();
            self.set(key, &new_value)?;
            Ok(new_value)
        }
    }
}

fn main() -> RedisResult<()> {
    let mut cache = RedisCacheWrapper::new("redis://127.0.0.1:6379/", 300)?;

    // Example of setting and getting a cache value
    cache.set("example_key", "example_value")?;

    let value = cache.get("example_key", || {
        // Simulate an expensive operation
        sleep(Duration::from_secs(5));
        "expensive_value".to_string()
    })?;

    println!("Retrieved value: {}", value);
    Ok(())
}
