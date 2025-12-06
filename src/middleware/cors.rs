use actix_cors::Cors;
use actix_web::http;

pub struct CorsMiddleware;

impl CorsMiddleware {
    /// Create a permissive CORS configuration for development
    pub fn permissive() -> Cors {
        Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600)
    }

    /// Create a strict CORS configuration for production
    pub fn strict() -> Cors {
        let allowed_origins = std::env::var("ALLOWED_ORIGINS")
            .unwrap_or_else(|_| "http://localhost:3000".to_string());

        let origins: Vec<&str> = allowed_origins.split(',').collect();

        let mut cors = Cors::default();

        for origin in origins {
            cors = cors.allowed_origin(origin.trim());
        }

        cors.allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::CONTENT_TYPE,
            ])
            .max_age(3600)
    }

    /// Create CORS configuration based on environment
    pub fn from_env() -> Cors {
        let env = std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string());

        match env.as_str() {
            "production" => Self::strict(),
            _ => Self::permissive(),
        }
    }

    // Create custom CORS with specific origins
    // pub fn custom(allowed_origins: Vec<&str>) -> Cors {
    //     let mut cors = Cors::default();

    //     for origin in allowed_origins {
    //         cors = cors.allowed_origin(origin);
    //     }

    //     cors.allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
    //         .allowed_headers(vec![
    //             http::header::AUTHORIZATION,
    //             http::header::ACCEPT,
    //             http::header::CONTENT_TYPE,
    //         ])
    //         .max_age(3600)
    // }
}
