use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready};
use actix_web::{Error as ActixError, web};
use futures::future::LocalBoxFuture;
use std::future::{Ready, ready};

use crate::configs::AppState;
use crate::services::AuthService;

// Re-export Claims for convenience
// pub use crate::models::Claims;

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixError;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService { service }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixError;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let path = req.path().to_string();

        // Skip authentication for public routes
        if path.starts_with("/api/auth/") || path == "/health" || path == "/" {
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            });
        }

        let auth_header = req.headers().get("Authorization");

        let token = match auth_header {
            Some(header) => {
                let header_str = header.to_str().unwrap_or("");
                header_str
                    .strip_prefix("Bearer ")
                    .map(|stripped| stripped.to_string())
            }
            None => None,
        };

        if token.is_none() {
            return Box::pin(async move {
                Err(actix_web::error::ErrorUnauthorized(
                    "Missing authorization token",
                ))
            });
        }

        let token = token.unwrap();
        let state = req.app_data::<web::Data<AppState>>().unwrap().clone();

        let fut = self.service.call(req);

        Box::pin(async move {
            match AuthService::validate_token(&state, &token).await {
                Ok(_) => {
                    let res = fut.await?;
                    Ok(res)
                }
                Err(_) => Err(actix_web::error::ErrorUnauthorized(
                    "Invalid or expired token",
                )),
            }
        })
    }
}
