use bcrypt::verify;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use redis::AsyncCommands;

use crate::configs::AppState;
use crate::dao::UserDao;
use crate::models::{Claims, LoginDto, TokenResponse};

pub struct AuthService;

impl AuthService {
    pub async fn login(state: &AppState, dto: LoginDto) -> Result<TokenResponse, String> {
        let user = UserDao::find_by_username(&state.db, &dto.username)
            .await
            .map_err(|_| "Invalid credentials")?;

        let valid =
            verify(&dto.password, &user.password_hash).map_err(|_| "Invalid credentials")?;

        if !valid {
            return Err("Invalid credentials".to_string());
        }

        let expiration = Utc::now() + Duration::hours(24);
        let claims = Claims {
            sub: user.username.clone(),
            user_id: user.id.to_string(),
            exp: expiration.timestamp() as usize,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
        )
        .map_err(|_| "Failed to generate token")?;

        let mut redis_conn = state.redis.clone();
        let _: () = redis_conn
            .set_ex(format!("token:{}", token), user.id.to_string(), 86400)
            .await
            .map_err(|_| "Failed to store token")?;

        Ok(TokenResponse {
            token,
            expires_in: 86400,
        })
    }

    pub async fn logout(state: &AppState, token: &str) -> Result<(), String> {
        let mut redis_conn = state.redis.clone();
        let _: () = redis_conn
            .del(format!("token:{}", token))
            .await
            .map_err(|_| "Failed to invalidate token")?;

        Ok(())
    }

    pub async fn validate_token(state: &AppState, token: &str) -> Result<Claims, String> {
        let mut redis_conn = state.redis.clone();
        let exists: bool = redis_conn
            .exists(format!("token:{}", token))
            .await
            .map_err(|_| "Token validation failed")?;

        if !exists {
            return Err("Token expired or invalid".to_string());
        }

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| "Invalid token")?;

        Ok(token_data.claims)
    }
}
