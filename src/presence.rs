// /src/presence.rs

use redis::AsyncCommands;
use anyhow::Result;

pub struct PresenceClient {
    client: redis::Client,
}

impl PresenceClient {
    pub fn new(redis_url: &str) -> Self {
        Self {
            client: redis::Client::open(redis_url).expect("redis url invalid"),
        }
    }

    pub async fn set_online(&self, user_id: &str, channel: &str) -> Result<()> {
        let mut conn = self.client.get_async_connection().await?;
        let key = format!("presence:{}:{}", channel, user_id);
        let _: () = conn.set_ex(key, "1", 60).await?; // expires in 60s
        Ok(())
    }

    pub async fn set_offline(&self, user_id: &str, channel: &str) -> Result<()> {
        let mut conn = self.client.get_async_connection().await?;
        let key = format!("presence:{}:{}", channel, user_id);
        let _: () = conn.del(key).await?;
        Ok(())
    }
}

