extern crate redis;
use redis::Commands;

pub fn connect_redis(url: &str) -> redis::RedisResult<redis::Connection> {
    let client = redis::Client::open(url)?;
    client.get_connection()
}
