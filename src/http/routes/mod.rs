use std::path::PathBuf;

use axum::Router;
use tower_http::{compression::CompressionLayer, services::ServeDir};

use super::{
    context::{Ctx, ValidBody},
    errors::not_found,
};

mod blog;
mod contact;
mod resume;

pub fn router<B: ValidBody>(ctx: Ctx) -> Router<(), B>
where
    <B as axum::body::HttpBody>::Error: Send + Sync + Into<axum::BoxError>,
    <B as axum::body::HttpBody>::Data: Send,
{
    let mut router = Router::new()
        .nest("/blog", blog::router())
        .nest("/contact", contact::router())
        .nest("/resume", resume::router());

    #[cfg(debug_assertions)] // Static files are served by the CDN in production
    {
        let static_files =
            ServeDir::new(PathBuf::from("static")).fallback(ServeDir::new(PathBuf::from("dist")));
        let compression_layer = CompressionLayer::new().br(true).gzip(true).deflate(true);
        router = router
            .nest_service("/static", static_files)
            .layer(compression_layer);
    }

    router.fallback(|| async { not_found() }).with_state(ctx)
}
