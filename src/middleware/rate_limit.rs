use actix_web::Error as ActixError;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready};
use futures::future::LocalBoxFuture;
use std::collections::HashMap;
use std::future::{Ready, ready};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Simple rate limiting middleware
pub struct RateLimitMiddleware {
    requests_per_minute: usize,
}

impl RateLimitMiddleware {
    pub fn new(requests_per_minute: usize) -> Self {
        Self {
            requests_per_minute,
        }
    }
}

struct RateLimitData {
    requests: Vec<Instant>,
}

impl<S, B> Transform<S, ServiceRequest> for RateLimitMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixError;
    type InitError = ();
    type Transform = RateLimitMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RateLimitMiddlewareService {
            service,
            limits: Arc::new(Mutex::new(HashMap::new())),
            requests_per_minute: self.requests_per_minute,
        }))
    }
}

pub struct RateLimitMiddlewareService<S> {
    service: S,
    limits: Arc<Mutex<HashMap<String, RateLimitData>>>,
    requests_per_minute: usize,
}

impl<S, B> Service<ServiceRequest> for RateLimitMiddlewareService<S>
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
        let ip = req
            .peer_addr()
            .map(|addr| addr.to_string())
            .unwrap_or_else(|| "unknown".to_string());

        let mut limits = self.limits.lock().unwrap();
        let now = Instant::now();
        let one_minute_ago = now - Duration::from_secs(60);

        let data = limits.entry(ip.clone()).or_insert(RateLimitData {
            requests: Vec::new(),
        });

        // Remove old requests
        data.requests.retain(|&time| time > one_minute_ago);

        if data.requests.len() >= self.requests_per_minute {
            tracing::warn!(
                ip = %ip,
                requests = %data.requests.len(),
                "Rate limit exceeded"
            );

            drop(limits); // Release lock

            return Box::pin(async move {
                Err(actix_web::error::ErrorTooManyRequests(
                    "Rate limit exceeded. Please try again later.",
                ))
            });
        }

        data.requests.push(now);
        drop(limits); // Release lock

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
