use sqlx::FromRow;

use super::Database;

#[derive(FromRow, Debug)]
pub struct Usage {
    pub id: i64,
    pub created_at: i64,
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub cid: String,
    pub user: i64,
}

pub async fn add_usage(db: &Database, usage: &Usage) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO UsageLog (created_at, prompt_tokens, completion_tokens, cid, user) VALUES (?, ?, ?, ?, ?)")
        .bind(usage.created_at)
        .bind(usage.prompt_tokens)
        .bind(usage.completion_tokens)
        .bind(&usage.cid)
        .bind(usage.user)
        .execute(db.get_pool())
        .await?;
    Ok(())
}

pub async fn get_user_usage_from_month(
    db: &Database,
    user: i64,
    month: i32,
) -> Result<Vec<Usage>, sqlx::Error> {
    let result = sqlx::query_as(
        "SELECT * FROM UsageLog WHERE user = ? AND MONTH(FROM_UNIXTIME(created_at)) = ?",
    )
    .bind(user)
    .bind(month)
    .fetch_all(db.get_pool())
    .await?;
    Ok(result)
}

pub async fn get_usage_from_month(db: &Database, month: i32) -> Result<Vec<Usage>, sqlx::Error> {
    let result =
        sqlx::query_as("SELECT * FROM UsageLog WHERE MONTH(FROM_UNIXTIME(created_at)) = ?")
            .bind(month)
            .fetch_all(db.get_pool())
            .await?;
    Ok(result)
}
