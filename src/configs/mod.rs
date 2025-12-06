pub mod database;
pub mod logging;

use redis::aio::ConnectionManager;
use sqlx::PgPool;
use std::env;

use database::{DatabaseConfig, RedisConfig};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub redis: ConnectionManager,
    pub jwt_secret: String,
}

impl AppState {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let jwt_secret = env::var("JWT_SECRET")
            .unwrap_or_else(|_| "your-secret-key-change-in-production".to_string());

        tracing::info!("Initializing application state...");

        let db_config = DatabaseConfig::from_env();
        let redis_config = RedisConfig::from_env();

        let db = db_config.connect().await?;
        let redis = redis_config.connect().await?;

        tracing::info!("✓ Application state initialized successfully");

        Ok(AppState {
            db,
            redis,
            jwt_secret,
        })
    }

    /// Health check for database and Redis connections
    pub async fn health_check(&self) -> Result<HealthStatus, Box<dyn std::error::Error>> {
        let mut status = HealthStatus::default();

        // Check PostgreSQL
        match sqlx::query("SELECT 1").execute(&self.db).await {
            Ok(_) => {
                status.database = "healthy".to_string();
                log::info!("✓ PostgreSQL health check passed");
            }
            Err(e) => {
                status.database = format!("unhealthy: {}", e);
                log::error!("✗ PostgreSQL health check failed: {}", e);
            }
        }

        // Check Redis
        let mut redis_conn = self.redis.clone();
        match redis::cmd("PING").query_async::<()>(&mut redis_conn).await {
            Ok(_) => {
                status.redis = "healthy".to_string();
                log::info!("✓ Redis health check passed");
            }
            Err(e) => {
                status.redis = format!("unhealthy: {}", e);
                log::error!("✗ Redis health check failed: {}", e);
            }
        }

        // Set overall status
        status.status = if status.database == "healthy" && status.redis == "healthy" {
            "healthy".to_string()
        } else {
            "unhealthy".to_string()
        };

        Ok(status)
    }
}

#[derive(Debug, serde::Serialize)]
#[allow(dead_code)]
pub struct HealthStatus {
    pub status: String,
    pub database: String,
    pub redis: String,
    pub timestamp: String,
}

impl Default for HealthStatus {
    fn default() -> Self {
        Self {
            status: "unknown".to_string(),
            database: "unknown".to_string(),
            redis: "unknown".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}
