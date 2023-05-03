use sqlx::{
    mysql::{MySqlDatabaseError, MySqlPoolOptions},
    MySql, Pool,
};
use std::time::Duration;
use tokio::time;

pub mod users;
pub mod channels;
pub mod contexts;
pub mod usage;
pub mod virtualusers;

pub struct Database {
    pool: Pool<MySql>,
}

impl Database {
    pub async fn new(url: &str) -> Self {
        let mysql_pool = loop {
            let connection = MySqlPoolOptions::new()
                .max_connections(10)
                .connect(url)
                .await;

            if let Ok(connection) = connection {
                break connection;
            } else {
                time::sleep(Duration::from_secs(5)).await;
            }
        };

        Self { pool: mysql_pool }
    }

    pub async fn migrate(&self) {
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await
            .unwrap();
    }

    pub(self) fn get_pool(&self) -> &Pool<MySql> {
        &self.pool
    }
}

pub(crate) trait SqlxErrorExtension {
    fn get_mysql(&self) -> &MySqlDatabaseError;
}

impl SqlxErrorExtension for sqlx::Error {
    fn get_mysql(&self) -> &MySqlDatabaseError {
        match self {
            sqlx::Error::Database(err) => err.downcast_ref::<MySqlDatabaseError>(),
            _ => panic!("Unexpected error type"),
        }
    }
}
