use std::sync::Arc;

use sqlx::postgres::PgPoolOptions;

pub mod types;

pub type DatabaseConnectionResource = sqlx::Pool<sqlx::Postgres>;
pub type DatabaseConnection = DatabaseConnectionResource;

pub trait IntoDbConnectionString {
    fn into_db_connection_string(self) -> String;
}

impl IntoDbConnectionString for &str {
    fn into_db_connection_string(self) -> String {
        self.to_string()
    }
}

impl IntoDbConnectionString for String {
    fn into_db_connection_string(self) -> String {
        self
    }
}

pub struct DbConnectionParameters {
    pub host: String,
    pub port: u32,
    pub database: String,
    pub user: String,
    pub password: String,
}

impl IntoDbConnectionString for DbConnectionParameters {
    fn into_db_connection_string(self) -> String {
        format!(
            "postgresql://{}:{}/{}?user={}&password={}",
            self.host, self.port, self.database, self.user, self.password,
        )
    }
}

pub async fn init_db_connection(
    connect: impl IntoDbConnectionString,
) -> anyhow::Result<DatabaseConnection> {
    Ok(PgPoolOptions::new()
        .max_connections(5)
        .connect(&connect.into_db_connection_string())
        .await?)
}
