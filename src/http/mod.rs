use context::Ctx;

use self::routes::router;

pub mod components;
pub mod context;
pub mod errors;
pub mod routes;

pub async fn serve(ctx: Ctx) -> anyhow::Result<()> {
    let app = router(ctx);

    lambda_http::run(app)
        .await
        .map_err(|_| anyhow::anyhow!("couldn't start app"))
}
