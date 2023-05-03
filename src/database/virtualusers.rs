use super::Database;
use num_enum::IntoPrimitive;
use sqlx::{Type, FromRow};

#[derive(IntoPrimitive, Clone, Copy, Type, PartialEq)]
#[repr(u8)]
pub enum VirtualUserRoles {
    Normal,
    Admin,
}

#[derive(FromRow)]
pub struct VirtualUser {
    pub id: i64,
    pub user_id: i64,
    pub virtual_user_id: i64,
    pub role: VirtualUserRoles
}

pub async fn add_virtual_user(db: &Database, user: &VirtualUser) -> Result<i64, sqlx::Error> {
    let result = sqlx::query("INSERT INTO VirtualUsers (user_id, virtual_user_id, role) VALUES (?, ?, ?)")
        .bind(user.user_id)
        .bind(user.virtual_user_id)
        .bind(user.role as u8)
        .execute(db.get_pool())
        .await?;
    Ok(result.last_insert_id() as i64)
}

pub async fn get_virtual_users(db: &Database, user_id: i64, role: VirtualUserRoles) -> Result<Vec<VirtualUser>, sqlx::Error> {
    let result = sqlx::query_as("SELECT * FROM VirtualUsers WHERE user_id = ? AND role = ?")
        .bind(user_id)
        .bind(role as u8)
        .fetch_all(db.get_pool())
        .await?;
    Ok(result)
}

pub async fn get_virtual_by_role(db: &Database, id: i64, role: VirtualUserRoles) -> Result<Vec<VirtualUser>, sqlx::Error> {
    let result = sqlx::query_as("SELECT * FROM VirtualUsers WHERE virtual_user_id = ? AND role = ?")
        .bind(id)
        .bind(role as u8)
        .fetch_all(db.get_pool())
        .await?;
    Ok(result)
}

pub async fn get_all_virtual_users(db: &Database, id: i64) -> Result<Vec<VirtualUser>, sqlx::Error> {
    let result = sqlx::query_as("SELECT * FROM VirtualUsers WHERE virtual_user_id = ?")
        .bind(id)
        .fetch_all(db.get_pool())
        .await?;
    Ok(result)
}