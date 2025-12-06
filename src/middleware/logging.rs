use actix_web::Error as ActixError;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready};
use futures::future::LocalBoxFuture;
use std::future::{Ready, ready};
use std::time::Instant;

/// Custom logging middleware for detailed request/response logging
pub struct LoggingMiddleware;

impl<S, B> Transform<S, ServiceRequest> for LoggingMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixError;
    type InitError = ();
    type Transform = LoggingMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(LoggingMiddlewareService { service }))
    }
}

pub struct LoggingMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for LoggingMiddlewareService<S>
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
        let start_time = Instant::now();
        let method = req.method().to_string();
        let path = req.path().to_string();
        let query = req.query_string().to_string();

        // Get client IP
        let peer_addr = req
            .peer_addr()
            .map(|addr| addr.to_string())
            .unwrap_or_else(|| "unknown".to_string());

        // Get user agent
        let user_agent = req
            .headers()
            .get("User-Agent")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("unknown")
            .to_string();

        // Log request
        tracing::info!(
            method = %method,
            path = %path,
            query = %query,
            ip = %peer_addr,
            user_agent = %user_agent,
            "Incoming request"
        );

        let fut = self.service.call(req);

        Box::pin(async move {
            let result = fut.await;
            let elapsed = start_time.elapsed();

            match &result {
                Ok(response) => {
                    let status = response.status();

                    if status.is_success() {
                        tracing::info!(
                            method = %method,
                            path = %path,
                            status = %status.as_u16(),
                            duration_ms = %elapsed.as_millis(),
                            "Request completed successfully"
                        );
                    } else if status.is_client_error() {
                        tracing::warn!(
                            method = %method,
                            path = %path,
                            status = %status.as_u16(),
                            duration_ms = %elapsed.as_millis(),
                            "Client error"
                        );
                    } else if status.is_server_error() {
                        tracing::error!(
                            method = %method,
                            path = %path,
                            status = %status.as_u16(),
                            duration_ms = %elapsed.as_millis(),
                            "Server error"
                        );
                    }
                }
                Err(error) => {
                    tracing::error!(
                        method = %method,
                        path = %path,
                        error = %error,
                        duration_ms = %elapsed.as_millis(),
                        "Request failed with error"
                    );
                }
            }

            result
        })
    }
}
