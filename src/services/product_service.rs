use sqlx::PgPool;
use uuid::Uuid;

use crate::dao::ProductDao;
use crate::models::{CreateProductDto, Product, ProductQuery, UpdateProductDto};

pub struct ProductService;

impl ProductService {
    pub async fn create(
        pool: &PgPool,
        dto: CreateProductDto,
        user_id: Uuid,
    ) -> Result<Product, String> {
        ProductDao::create(pool, &dto, user_id)
            .await
            .map_err(|e| format!("Failed to create product: {}", e))
    }

    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Product, String> {
        ProductDao::find_by_id(pool, id)
            .await
            .map_err(|_| "Product not found".to_string())
    }

    pub async fn search(pool: &PgPool, query: ProductQuery) -> Result<Vec<Product>, String> {
        ProductDao::find_all_dynamic(pool, &query)
            .await
            .map_err(|e| format!("Failed to search products: {}", e))
    }

    pub async fn update(pool: &PgPool, id: Uuid, dto: UpdateProductDto) -> Result<Product, String> {
        ProductDao::update(pool, id, &dto)
            .await
            .map_err(|e| format!("Failed to update product: {}", e))
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), String> {
        ProductDao::delete(pool, id)
            .await
            .map_err(|e| format!("Failed to delete product: {}", e))?;
        Ok(())
    }
}
