use super::Database;
use enumflags2::bitflags;
use num_enum::IntoPrimitive;
use poise::ChoiceParameter;
use sqlx::{FromRow, Type};

#[derive(IntoPrimitive, Clone, Copy, Type, ChoiceParameter, PartialEq, PartialOrd)]
#[repr(u8)]
#[bitflags]
pub enum UserStates {
    Normal,
    Blocked,
    SemImposto,
    DmEnabled
}

#[derive(FromRow)]
pub struct User {
    pub id: i64,
    pub discord_id: i64,
    pub name: String,
    pub state: UserStates,
    //this is not a typo
    #[sqlx(rename = "virtual")]
    pub virtal: bool
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: 0,
            discord_id: 0,
            name: "".into(),
            state: UserStates::Normal,
            virtal: false
        }
    }
}

pub async fn add_user(db: &Database, user: &User) -> Result<i64, sqlx::Error> {
    let result = sqlx::query("INSERT INTO Users (discord_id, name, state, virtual) VALUES (?, ?, ?, ?)")
        .bind(user.discord_id)
        .bind(&user.name)
        .bind(user.state as u8)
        .bind(user.virtal)
        .execute(db.get_pool())
        .await?;
    Ok(result.last_insert_id() as i64)
}

pub async fn get_by_discord_id(db: &Database, discord_id: i64) -> Option<User> {
    let result = sqlx::query_as("SELECT * FROM Users WHERE discord_id = ?")
        .bind(discord_id)
        .fetch_optional(db.get_pool())
        .await
        .unwrap();
    result
}

pub async fn change_state(db: &Database, user: &User) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE Users SET state = ? WHERE id = ?")
        .bind(user.state as u8)
        .bind(user.id)
        .execute(db.get_pool())
        .await?;
    Ok(())
}

pub async fn get_by_id(db: &Database, id: i64) -> Option<User> {
    let result = sqlx::query_as("SELECT * FROM Users WHERE id = ?")
        .bind(id)
        .fetch_optional(db.get_pool())
        .await
        .unwrap();
    result
}
