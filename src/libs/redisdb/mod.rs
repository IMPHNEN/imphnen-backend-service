use super::Env;
use redis::Client;
use redis::RedisResult;

pub mod key;
pub use key::*;

pub async fn redisdb_init() -> RedisResult<Client> {
	let env = Env::new();
	let url = format!("redis://{}", env.redisdb_url);
	let client = Client::open(url)?;
	Ok(client)
}
