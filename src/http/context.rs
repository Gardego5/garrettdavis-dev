#[derive(Debug, Clone)]
pub struct Ctx {}

pub trait ValidBody: axum::body::HttpBody + Send + Sync + 'static {}
impl<B> ValidBody for B where B: axum::body::HttpBody + Send + Sync + 'static {}
