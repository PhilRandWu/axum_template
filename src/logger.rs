use crate::settings::SETTINGS;
use std::env;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::prelude::*;

pub fn setup() {
    if env::var_os("RUST_LOG").is_none() {
        let app_name = env::var("CARGO_PKG_NAME").unwrap();
        let level = SETTINGS.logger.level.as_str();
        let env = format!("{app_name}={level},tower_http={level}");
        env::set_var("RUST_LOG", env);
    }

    // 设置文件日志appender，每天轮转一次
    let file_appender = RollingFileAppender::new(
        Rotation::DAILY,
        "logs",
        "app.log",
    );

    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking)
        .with_thread_ids(true)
        .with_target(true)
        .with_level(true)
        .with_file(true)
        .with_line_number(true);

    let console_layer = tracing_subscriber::fmt::layer()
        .with_thread_ids(true)
        .with_target(true)
        .with_level(true);

    tracing_subscriber::registry()
        .with(file_layer)
        .with(console_layer)
        .init();
}