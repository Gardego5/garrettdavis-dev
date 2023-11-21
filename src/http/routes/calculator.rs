use axum::Router;
use maud::{html, Markup};

use crate::http::{
    components::{
        layout::{header, margins},
        template::template,
    },
    context::{Ctx, ValidBody},
};

pub fn router<B: ValidBody>() -> Router<Ctx, B> {
    Router::new().route("/", axum::routing::get(index))
}

async fn index() -> Markup {
    let head = html! {
        title { "Calculator" }
        meta name="description" content="Calculator";
    };

    let body = html! {
        (header("Calculator"))
        (margins(html! {
        }))
    };

    template(head, body)
}
