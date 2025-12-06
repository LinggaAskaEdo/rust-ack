use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

pub struct LoggingConfig {
    pub log_dir: String,
    pub log_level: String,
    pub rotation: LogRotation,
    pub format: LogFormat,
}

pub enum LogRotation {
    Hourly,
    Daily,
    Never,
}

pub enum LogFormat {
    Json,
    Pretty,
    Compact,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            log_dir: "logs".to_string(),
            log_level: "info".to_string(),
            rotation: LogRotation::Daily,
            format: LogFormat::Json,
        }
    }
}

impl LoggingConfig {
    pub fn from_env() -> Self {
        Self {
            log_dir: std::env::var("LOG_DIR").unwrap_or_else(|_| "logs".to_string()),
            log_level: std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
            rotation: match std::env::var("LOG_ROTATION").as_deref() {
                Ok("hourly") => LogRotation::Hourly,
                Ok("daily") => LogRotation::Daily,
                Ok("never") => LogRotation::Never,
                _ => LogRotation::Daily, // Default to Daily
            },
            format: match std::env::var("LOG_FORMAT").as_deref() {
                Ok("json") => LogFormat::Json,
                Ok("pretty") => LogFormat::Pretty,
                Ok("compact") => LogFormat::Compact,
                _ => LogFormat::Json, // Default to JSON
            },
        }
    }

    pub fn init(&self) {
        // Create logs directory if it doesn't exist
        std::fs::create_dir_all(&self.log_dir).expect("Failed to create logs directory");

        // Configure rotation strategy
        let rotation = match self.rotation {
            LogRotation::Hourly => Rotation::HOURLY,
            LogRotation::Daily => Rotation::DAILY,
            LogRotation::Never => Rotation::NEVER,
        };

        // Create file appender with rotation
        let file_appender = RollingFileAppender::new(rotation, &self.log_dir, "app.log");

        // Configure environment filter
        let env_filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&self.log_level));

        // Create file layer based on format
        let file_layer = match self.format {
            LogFormat::Json => fmt::layer()
                .json()
                .with_writer(file_appender)
                .with_target(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_file(true)
                .with_line_number(true)
                .with_current_span(true)
                .with_span_list(true)
                .boxed(),

            LogFormat::Compact => fmt::layer()
                .compact()
                .with_writer(file_appender)
                .with_ansi(false)
                .with_target(true)
                .with_file(true)
                .with_line_number(true)
                .boxed(),

            LogFormat::Pretty => fmt::layer()
                .with_writer(file_appender)
                .with_ansi(false)
                .with_target(true)
                .with_thread_ids(true)
                .with_file(true)
                .with_line_number(true)
                .boxed(),
        };

        // Console layer - always pretty for better readability
        let stdout_layer = fmt::layer()
            .with_writer(std::io::stdout)
            .with_target(false)
            .pretty()
            .boxed();

        // Initialize tracing subscriber with both console and file output
        tracing_subscriber::registry()
            .with(env_filter)
            .with(file_layer)
            .with(stdout_layer)
            .init();

        tracing::info!("✓ Logging initialized");
        tracing::info!("  Log directory: {}", self.log_dir);
        tracing::info!("  Log level: {}", self.log_level);
        tracing::info!(
            "  Log format: {:?}",
            match self.format {
                LogFormat::Json => "JSON",
                LogFormat::Pretty => "Pretty",
                LogFormat::Compact => "Compact",
            }
        );
        tracing::info!(
            "  Rotation: {:?}",
            match self.rotation {
                LogRotation::Hourly => "Hourly",
                LogRotation::Daily => "Daily",
                LogRotation::Never => "Never",
            }
        );
    }
}
