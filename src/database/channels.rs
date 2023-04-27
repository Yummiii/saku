use super::Database;
use num_enum::IntoPrimitive;
use poise::ChoiceParameter;
use sqlx::{FromRow, Type};

#[derive(IntoPrimitive, Clone, Copy, Type, ChoiceParameter, PartialEq)]
#[repr(u8)]
pub enum ChannelStates {
    Disabled,
    Enabled,
}

#[derive(FromRow)]
pub struct Channel {
    pub id: i64,
    pub discord_id: i64,
    pub ccid: String,
    pub state: ChannelStates,
    pub system: Option<String>
}

pub async fn add_channel(db: &Database, channel: &Channel) -> Result<i64, sqlx::Error> {
    let result = sqlx::query("INSERT INTO Channels (discord_id, state, ccid) VALUES (?, ?, ?)")
        .bind(channel.discord_id)
        .bind(channel.state as u8)
        .bind(&channel.ccid)
        .execute(db.get_pool())
        .await?;
    Ok(result.last_insert_id() as i64)
}

pub async fn get_by_discord_id(db: &Database, discord_id: i64) -> Option<Channel> {
    let result = sqlx::query_as("SELECT * FROM Channels WHERE discord_id = ?")
        .bind(discord_id)
        .fetch_optional(db.get_pool())
        .await
        .unwrap();
    result
}

pub async fn change_state(db: &Database, channel: &Channel) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE Channels SET state = ? WHERE id = ?")
        .bind(channel.state as u8)
        .bind(channel.id)
        .execute(db.get_pool())
        .await?;
    Ok(())
}

pub async fn set_ccid(db: &Database, channel: &Channel) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE Channels SET ccid = ? WHERE id = ?")
        .bind(&channel.ccid)
        .bind(channel.id)
        .execute(db.get_pool())
        .await?;
    Ok(())
}

pub async fn set_system(db: &Database, channel: &Channel) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE Channels SET system = ? WHERE id = ?")
        .bind(&channel.system)
        .bind(channel.id)
        .execute(db.get_pool())
        .await?;
    Ok(())
}