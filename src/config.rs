// /src/config.rs

use dotenvy::dotenv;
use std::env;

#[derive(Clone)]
pub struct Config {
    pub addr: String,
    pub database_url: String,
    pub kafka_brokers: String,
    pub redis_url: String,
    pub kafka_topic: String,
    pub log_level: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        Self {
            addr: env::var("ADDR").unwrap_or_else(|_| "0.0.0.0:3000".into()),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            kafka_brokers: env::var("KAFKA_BROKERS").expect("KAFKA_BROKERS must be set"),
            redis_url: env::var("REDIS_URL").expect("REDIS_URL must be set"),
            kafka_topic: env::var("KAFKA_TOPIC").unwrap_or_else(|_| "chat_messages".into()),
            log_level: env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        }
    }
}
