use axum::http::header;
use axum::Router;
use crate::{logger, routes};
use tower_http::{
    compression::CompressionLayer, cors::CorsLayer, propagate_header::PropagateHeaderLayer,
    sensitive_headers::SetSensitiveHeadersLayer, trace,
};
pub async fn create_app() -> Router {
    logger::setup();

    Router::new()
        .merge(routes::status::create_route())
        .layer(trace::TraceLayer::new_for_http() // 一个用于 HTTP 的跟踪层，用于记录请求和响应的详细信息。
            .make_span_with(trace::DefaultMakeSpan::new().include_headers(true)) // 在创建跟踪跨度时包含请求头。
            .on_request(trace::DefaultOnRequest::new().level(tracing::Level::INFO)) // 在请求到达时记录日志，日志级别为 INFO。
            .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO))
        )
        .layer(SetSensitiveHeadersLayer::new(std::iter::once( // 将 Authorization 请求头标记为敏感信息，这样它就不会出现在日志中。
            header::AUTHORIZATION
        )))
        .layer(CompressionLayer::new()) // 用于压缩响应内容。
        .layer(PropagateHeaderLayer::new(header::HeaderName::from_static("x-request-id"))) // 用于将 X-Request-Id 请求头从请求传播到响应。
        .layer(CorsLayer::permissive())
}