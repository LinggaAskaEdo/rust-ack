use crate::models::{CreateUserDto, UpdateUserDto, User};
use sqlx::{PgPool, postgres::PgQueryResult};
use uuid::Uuid;

pub struct UserDao;

impl UserDao {
    pub async fn create(
        pool: &PgPool,
        dto: &CreateUserDto,
        password_hash: &str,
    ) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "INSERT INTO rustack.users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(&dto.username)
        .bind(&dto.email)
        .bind(password_hash)
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM rustack.users WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await
    }

    pub async fn find_by_username(pool: &PgPool, username: &str) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM rustack.users WHERE username = $1")
            .bind(username)
            .fetch_one(pool)
            .await
    }

    pub async fn find_all(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM rustack.users ORDER BY created_at DESC")
            .fetch_all(pool)
            .await
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        dto: &UpdateUserDto,
        password_hash: Option<&str>,
    ) -> Result<User, sqlx::Error> {
        let mut query = String::from("UPDATE rustack.users SET updated_at = NOW()");
        let mut params: Vec<String> = vec![];

        if let Some(username) = &dto.username {
            params.push(format!("username = '{}'", username));
        }

        if let Some(email) = &dto.email {
            params.push(format!("email = '{}'", email));
        }

        if let Some(hash) = password_hash {
            params.push(format!("password_hash = '{}'", hash));
        }

        if !params.is_empty() {
            query.push_str(&format!(", {}", params.join(", ")));
        }

        query.push_str(&format!(" WHERE id = '{}' RETURNING *", id));

        sqlx::query_as::<_, User>(&query).fetch_one(pool).await
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query("DELETE FROM rustack.users WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await
    }
}
