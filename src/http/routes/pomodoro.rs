use std::f64::consts::PI;

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
    let two_sqrt = 2f64.sqrt();
    let factor = 1. / (1. + two_sqrt);

    let head = html! {
        title { "Pomodoro" }
        meta name="description" content="Pomodoro timer";
    };

    let body = html! {
        (header("Pomodoro"))
        (margins(html! {
            svg class="max-w-md m-auto" viewBox="-2 -2 104 104" x-data {
                style { "text { font-family: Montserrat; text-anchor: middle; alignment-baseline: middle; }" }

                // Main Wrapper
                // circle cy="50" cx="50" r="50" class="fill-transparent stroke-1 stroke-slate-500" { }

                // Tomotoes
                @for i in 0..4 {
                    @let r = f64::from(i) * PI * 0.5 - PI / 2.;
                    @let cx = 50. + 50. * (1. - factor) * r.cos();
                    @let cy = 50. + 50. * (1. - factor) * r.sin();
                    text class="fill-white" x=(cx) y=(cy) { (i) }
                    path class="stroke-slate-500 fill-red-200 stroke-1 fill-transparent"
                        d=(format!("M {cx} {cy} a {r} {r} 10 1 0 14,10")) { }
                    // circle class="stroke-slate-500 stroke-1 fill-transparent"
                    //     x-on:mousemove={ "console.log(" (i) ")" }
                    //     r=((50. * factor * 0.8)) cx=(cx) cy=(cy) { }

                    @let cx = 50. + 50. * (1. - factor / two_sqrt) * (r + PI * 0.25).cos();
                    @let cy = 50. + 50. * (1. - factor / two_sqrt) * (r + PI * 0.25).sin();
                    circle class="stroke-slate-500 stroke-1 fill-transparent"
                        r=((5)) cx=(cx) cy=(cy) { }
                }
            }
        }))
    };

    template(head, body)
}
