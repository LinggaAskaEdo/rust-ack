use crate::models::{CreateProductDto, Product, ProductQuery, UpdateProductDto};
use sqlx::{PgPool, postgres::PgQueryResult};
use uuid::Uuid;

pub struct ProductDao;

impl ProductDao {
    pub async fn create(
        pool: &PgPool,
        dto: &CreateProductDto,
        user_id: Uuid,
    ) -> Result<Product, sqlx::Error> {
        sqlx::query_as::<_, Product>(
            "INSERT INTO rustack.products (name, description, price, stock, created_by) VALUES ($1, $2, $3, $4, $5) RETURNING *"
        )
        .bind(&dto.name)
        .bind(&dto.description)
        .bind(dto.price)
        .bind(dto.stock)
        .bind(user_id)
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Product, sqlx::Error> {
        sqlx::query_as::<_, Product>("SELECT * FROM rustack.products WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await
    }

    pub async fn find_all_dynamic(
        pool: &PgPool,
        query_params: &ProductQuery,
    ) -> Result<Vec<Product>, sqlx::Error> {
        let mut query = String::from("SELECT * FROM rustack.products WHERE 1=1");

        if let Some(name) = &query_params.name {
            query.push_str(&format!(" AND name ILIKE '%{}%'", name));
        }
        if let Some(min_price) = query_params.min_price {
            query.push_str(&format!(" AND price >= {}", min_price));
        }
        if let Some(max_price) = query_params.max_price {
            query.push_str(&format!(" AND price <= {}", max_price));
        }
        if let Some(min_stock) = query_params.min_stock {
            query.push_str(&format!(" AND stock >= {}", min_stock));
        }

        query.push_str(" ORDER BY created_at DESC");

        sqlx::query_as::<_, Product>(&query).fetch_all(pool).await
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        dto: &UpdateProductDto,
    ) -> Result<Product, sqlx::Error> {
        let mut query = String::from("UPDATE rustack.products SET updated_at = NOW()");
        let mut params: Vec<String> = vec![];

        if let Some(name) = &dto.name {
            params.push(format!("name = '{}'", name));
        }

        if let Some(description) = &dto.description {
            params.push(format!("description = '{}'", description));
        }

        if let Some(price) = dto.price {
            params.push(format!("price = {}", price));
        }

        if let Some(stock) = dto.stock {
            params.push(format!("stock = {}", stock));
        }

        if !params.is_empty() {
            query.push_str(&format!(", {}", params.join(", ")));
        }

        query.push_str(&format!(" WHERE id = '{}' RETURNING *", id));

        sqlx::query_as::<_, Product>(&query).fetch_one(pool).await
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query("DELETE FROM rustack.products WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await
    }
}
