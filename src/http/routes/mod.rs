use std::path::PathBuf;

use axum::Router;
use tower_http::{compression::CompressionLayer, services::ServeDir};

use super::{
    context::{Ctx, ValidBody},
    errors::not_found,
};

mod blog;
mod calculator;
mod contact;
mod pomodoro;
mod resume;

// #[cfg(debug_assertions)]
// const CACHE_CONTROL_VALUE: &'static str = "no-cache";
// #[cfg(not(debug_assertions))]
// const CACHE_CONTROL_VALUE: &'static str = "max-age=604800, stale-while-revalidate=86400";

pub fn router<B: ValidBody>(ctx: Ctx) -> Router<(), B>
where
    <B as axum::body::HttpBody>::Error: Send + Sync + Into<axum::BoxError>,
    <B as axum::body::HttpBody>::Data: Send,
{
    Router::new()
        .nest("/blog", blog::router())
        .nest("/contact", contact::router())
        .nest("/resume", resume::router())
        .nest("/calculator", calculator::router())
        .nest("/pomodoro", pomodoro::router())
        .nest_service("/static", ServeDir::new(PathBuf::from("static")))
        .layer(CompressionLayer::new().br(true).gzip(true).deflate(true))
        //.layer(SetResponseHeaderLayer::if_not_present(
        //    CACHE_CONTROL,
        //    HeaderValue::from_static(CACHE_CONTROL_VALUE),
        //))
        .fallback(|| async { not_found() })
        .with_state(ctx)
}
