mod redis_utils;

use rand::Rng;
use redis::{Commands, Connection, RedisResult};
use redis_utils::{populate_redis, set_eviction_policy, EVICTION_POLICIES};

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

    pub fn memory_usage(&mut self) -> RedisResult<u64> {
        let info: String = redis::cmd("INFO")
            .arg("memory")
            .query(&mut self.connection)?;

        let used_memory_line = info
            .lines()
            .find(|line| line.starts_with("used_memory:"))
            .ok_or_else(|| {
                redis::RedisError::from((redis::ErrorKind::TypeError, "used_memory not found"))
            })?;

        let used_memory: u64 = used_memory_line
            .split(':')
            .nth(1)
            .ok_or_else(|| {
                redis::RedisError::from((redis::ErrorKind::TypeError, "Invalid memory info format"))
            })?
            .trim()
            .parse()
            .map_err(|_| {
                redis::RedisError::from((redis::ErrorKind::TypeError, "Invalid number format"))
            })?;

        Ok(used_memory)
    }

    pub fn key_count(&mut self) -> RedisResult<u64> {
        let key_count: u64 = redis::cmd("DBSIZE").query(&mut self.connection)?;
        Ok(key_count)
    }
}

fn main() -> RedisResult<()> {
    let mut cache = RedisCacheWrapper::new("redis://127.0.0.1:6379/", 300)?;

    redis::cmd("CONFIG")
        .arg("SET")
        .arg("maxmemory")
        .arg("6mb")
        .exec(&mut cache.connection)?;

    populate_redis(&mut cache, 0, 10000, 100)?;

    for &policy in EVICTION_POLICIES {
        println!("\nTesting eviction policy: {}", policy);

        set_eviction_policy(&mut cache.connection, policy)?;

        let initial_memory = cache.memory_usage()?;
        let initial_keys = cache.key_count()?;
        println!("Initial memory usage: {} bytes", initial_memory);
        println!("Initial key count: {}", initial_keys);

        populate_redis(&mut cache, 10000, 20000, 10000)?;

        // Log final state
        let final_memory = cache.memory_usage()?;
        let final_keys = cache.key_count()?;
        println!("Final memory usage: {} bytes", final_memory);
        println!("Final key count: {}", final_keys);

        println!(
            "Memory usage change: {} bytes",
            final_memory as i64 - initial_memory as i64
        );
        println!(
            "Key count change: {}",
            final_keys as i64 - initial_keys as i64
        );
    }

    Ok(())
}
