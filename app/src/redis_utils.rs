use redis::Connection;
use redis::ConnectionLike;
use redis::RedisResult;

use crate::RedisCacheWrapper;

pub const EVICTION_POLICIES: &[&str] = &[
    "volatile-lru",
    "allkeys-lru",
    "volatile-lfu",
    "allkeys-lfu",
    "volatile-random",
    "allkeys-random",
    "volatile-ttl",
    "noeviction",
];

pub fn set_eviction_policy(connection: &mut Connection, policy: &str) -> RedisResult<()> {
    connection.req_command(
        redis::cmd("CONFIG")
            .arg("SET")
            .arg("maxmemory-policy")
            .arg(policy),
    )?;
    println!("Eviction policy set to: {}", policy);
    Ok(())
}

pub fn populate_redis(
    cache: &mut RedisCacheWrapper,
    start: usize,
    end: usize,
    value_size: usize,
) -> RedisResult<()> {
    let large_value: String = "A".repeat(value_size); // Generate a large value of specified size

    for i in start..end {
        let key = format!("extra_key{}", i);

        match cache.set(&key, &large_value) {
            Ok(_) => (),
            Err(err) => {
                println!("Encountered error when setting {}: {}", key, err);
                if err.to_string().contains("OOM") {
                    println!("Memory limit reached, stopping data insertion for this policy.");
                    break;
                } else {
                    return Err(err); // Propagate other errors
                }
            }
        }
    }

    Ok(())
}
