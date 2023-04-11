use sqlx::FromRow;

use super::Database;

#[derive(FromRow, Debug)]
pub struct Context {
    pub id: i64,
    pub role: String,
    pub message: String,
    pub active: bool,
    pub created_at: i64,
    pub cid: String,
    pub channel: i64,
    pub user: Option<i64>,
}

pub async fn get_channel_context(db: &Database, channel: i64) -> Vec<Context> {
    let result = sqlx::query_as("SELECT * FROM Contexts WHERE channel = ? and active = true")
        .bind(channel)
        .fetch_all(db.get_pool())
        .await
        .unwrap();
    result
}

pub async fn add_context(db: &Database, context: &Context) -> Result<i64, sqlx::Error> {
    let result = sqlx::query("INSERT INTO Contexts (role, message, active, created_at, cid, channel, user) VALUES (?, ?, ?, ?, ?, ?, ?)")
        .bind(&context.role)
        .bind(&context.message)
        .bind(context.active)
        .bind(context.created_at)
        .bind(&context.cid)
        .bind(context.channel)
        .bind(context.user)
        .execute(db.get_pool())
        .await?;
    Ok(result.last_insert_id() as i64)
}

pub async fn deactivate_channel_context(db: &Database, channel: i64) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE Contexts SET active = false WHERE channel = ?")
        .bind(channel)
        .execute(db.get_pool())
        .await?;
    Ok(())
}