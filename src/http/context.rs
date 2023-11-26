use crate::config::Config;

#[derive(Debug, Clone)]
pub struct Ctx {
    pub ses: aws_sdk_sesv2::Client,
    pub cfg: Config,
}

pub trait ValidBody: axum::body::HttpBody + Send + Sync + 'static {}
impl<B> ValidBody for B where B: axum::body::HttpBody + Send + Sync + 'static {}
