use bcrypt::{DEFAULT_COST, hash};
use sqlx::PgPool;
use uuid::Uuid;

use crate::dao::UserDao;
use crate::models::{CreateUserDto, UpdateUserDto, User};

pub struct UserService;

impl UserService {
    pub async fn create(pool: &PgPool, dto: CreateUserDto) -> Result<User, String> {
        let password_hash =
            hash(&dto.password, DEFAULT_COST).map_err(|_| "Failed to hash password")?;

        UserDao::create(pool, &dto, &password_hash)
            .await
            .map_err(|e| format!("Failed to create user: {}", e))
    }

    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<User, String> {
        UserDao::find_by_id(pool, id)
            .await
            .map_err(|_| "User not found".to_string())
    }

    pub async fn get_all(pool: &PgPool) -> Result<Vec<User>, String> {
        UserDao::find_all(pool)
            .await
            .map_err(|e| format!("Failed to fetch users: {}", e))
    }

    pub async fn update(pool: &PgPool, id: Uuid, dto: UpdateUserDto) -> Result<User, String> {
        let password_hash = if let Some(password) = &dto.password {
            Some(hash(password, DEFAULT_COST).map_err(|_| "Failed to hash password")?)
        } else {
            None
        };

        UserDao::update(pool, id, &dto, password_hash.as_deref())
            .await
            .map_err(|e| format!("Failed to update user: {}", e))
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), String> {
        UserDao::delete(pool, id)
            .await
            .map_err(|e| format!("Failed to delete user: {}", e))?;
        Ok(())
    }
}
