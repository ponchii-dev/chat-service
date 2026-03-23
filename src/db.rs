// /src/db.rs

use sqlx::{PgPool, postgres::PgPoolOptions};
use crate::models::ChatMessage;
use anyhow::Result;

pub async fn create_pool(database_url: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;
    Ok(pool)
}

pub async fn persist_message(pool: &PgPool, msg: &ChatMessage) -> Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO messages (id, channel, sender_id, content, created_at)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        msg.id,
        msg.channel,
        msg.sender_id,
        msg.content,
        msg.created_at
    )
    .execute(pool)
    .await?;
    Ok(())
}
