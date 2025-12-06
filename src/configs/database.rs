use redis::aio::ConnectionManager;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;

pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connection_timeout: Duration,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://postgres:password@localhost/rustapi".to_string()),
            max_connections: 10,
            min_connections: 2,
            connection_timeout: Duration::from_secs(30),
        }
    }
}

impl DatabaseConfig {
    pub fn from_env() -> Self {
        Self {
            url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://postgres:password@localhost/rustapi".to_string()),
            max_connections: std::env::var("DB_MAX_CONNECTIONS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(10),
            min_connections: std::env::var("DB_MIN_CONNECTIONS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(2),
            connection_timeout: Duration::from_secs(
                std::env::var("DB_CONNECTION_TIMEOUT")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(30),
            ),
        }
    }

    pub async fn connect(&self) -> Result<PgPool, sqlx::Error> {
        log::info!("Connecting to PostgreSQL at {}...", self.url);
        log::info!("  Max connections: {}", self.max_connections);
        log::info!("  Min connections: {}", self.min_connections);
        log::info!("  Connection timeout: {:?}", self.connection_timeout);

        let pool = PgPoolOptions::new()
            .max_connections(self.max_connections)
            .min_connections(self.min_connections)
            .acquire_timeout(self.connection_timeout)
            .connect(&self.url)
            .await?;

        // Test the connection
        sqlx::query("SELECT 1").execute(&pool).await?;

        log::info!("✓ Connected to PostgreSQL successfully");
        Ok(pool)
    }
}

pub struct RedisConfig {
    pub url: String,
    pub connection_timeout: Duration,
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            url: std::env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            connection_timeout: Duration::from_secs(10),
        }
    }
}

impl RedisConfig {
    pub fn from_env() -> Self {
        Self {
            url: std::env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            connection_timeout: Duration::from_secs(
                std::env::var("REDIS_CONNECTION_TIMEOUT")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(10),
            ),
        }
    }

    pub async fn connect(&self) -> Result<ConnectionManager, redis::RedisError> {
        log::info!("Connecting to Redis at {}...", self.url);
        log::info!("  Connection timeout: {:?}", self.connection_timeout);

        let client = redis::Client::open(self.url.as_str())?;
        let conn = ConnectionManager::new(client).await?;

        // Test the connection
        let mut test_conn = conn.clone();
        redis::cmd("PING").query_async::<()>(&mut test_conn).await?;

        log::info!("✓ Connected to Redis successfully");
        Ok(conn)
    }
}
