use sqlx::{FromRow, Pool, Sqlite};

#[derive(FromRow)]
pub struct Message {
    pub role: String,
    pub content: String,
    pub channel: i64,
}

#[derive(FromRow)]
pub struct Channel {
    pub id: i64,
    pub enabled: bool,
}

pub async fn get_context(db: &Pool<Sqlite>, channel: i64) -> Vec<Message> {
    let ctx = sqlx::query_as("SELECT * FROM Context WHERE channel = ?")
        .bind(channel)
        .fetch_all(db)
        .await
        .unwrap();
    ctx
}

pub async fn add_context(db: &Pool<Sqlite>, msg: Message) {
    sqlx::query("INSERT INTO Context (role, content, channel) VALUES (?, ?, ?)")
        .bind(msg.role)
        .bind(msg.content)
        .bind(msg.channel)
        .execute(db)
        .await
        .unwrap();
}

pub async fn clear_channel_context(db: &Pool<Sqlite>, channel: i64) {
    sqlx::query("DELETE FROM Context WHERE channel = ?")
        .bind(channel)
        .execute(db)
        .await
        .unwrap();
}

pub async fn is_channel_enabled(db: &Pool<Sqlite>, channel: i64) -> bool {
    let channel = sqlx::query_as::<_, Channel>("SELECT * FROM Channels WHERE id = ?")
        .bind(channel)
        .fetch_optional(db)
        .await;
    if let Ok(Some(channel)) = channel {
        channel.enabled
    } else {
        false
    }
}

pub async fn create_channel(db: &Pool<Sqlite>, channel: &Channel) {
    sqlx::query("INSERT INTO Channels (id, enabled) VALUES (?, ?)")
        .bind(channel.id)
        .bind(channel.enabled)
        .execute(db)
        .await
        .unwrap();
}

pub async fn set_channel_state(db: &Pool<Sqlite>, channel: &Channel) {
    sqlx::query("UPDATE Channels SET enabled = ? WHERE id = ?")
        .bind(channel.enabled)
        .bind(channel.id)
        .execute(db)
        .await
        .unwrap();
}

pub async fn channel_exists(db: &Pool<Sqlite>, channel: i64) -> bool {
    let channel = sqlx::query_as::<_, Channel>("SELECT * FROM Channels WHERE id = ?")
        .bind(channel)
        .fetch_optional(db)
        .await;
    if let Ok(Some(_)) = channel {
        true
    } else {
        false
    }
}