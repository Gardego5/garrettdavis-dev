use {
    crate::http::components::{
        layout::{header, margins},
        template::template,
    },
    axum::{
        http::StatusCode,
        response::{IntoResponse, Response},
    },
    maud::{html, Markup, Render},
};

#[derive(Debug)]
pub struct ErrorPage<'a> {
    pub title: &'a str,
    pub code: StatusCode,
    pub image_src: &'a str,
    pub image_alt: &'a str,
    pub message: Markup,
}

impl Render for ErrorPage<'_> {
    fn render(&self) -> Markup {
        let head = html! {
            title { (self.title) }
        };

        let body = html! {
            (header(self.title))
            (margins(html! {
                div class="grid gap-6 text-center" {
                    div class="flex items-center justify-center gap-3 font-mono italic" {
                        span class="px-2" { "<Error>" }
                        span class="text-5xl" { (self.code.as_u16()) }
                        span class="px-2" { "</Error>" }
                    }

                    img class="m-auto w-[75%] rounded-full" alt=(self.image_alt)
                        src=(self.image_src) {}

                    (self.message)
                }
            }))
        };

        template(head, body)
    }
}

impl Into<(StatusCode, Markup)> for ErrorPage<'_> {
    fn into(self) -> (StatusCode, Markup) {
        (self.code, self.render())
    }
}

impl IntoResponse for ErrorPage<'_> {
    fn into_response(self) -> Response {
        let resp: (StatusCode, Markup) = self.into();
        resp.into_response()
    }
}

impl Into<Response> for ErrorPage<'_> {
    fn into(self) -> Response {
        self.into_response()
    }
}

pub fn not_found() -> ErrorPage<'static> {
    ErrorPage {
        title: "Oops... I couldn't find that page",
        code: StatusCode::NOT_FOUND,
        image_src: "/static/img/coffee_flower.jpg",
        image_alt: "latte with orchid",
        message: html! {
            p class="text-lg italic" {
                "That's no good, and I'm sorry. Maybe this page will exist in "
                "the future!" br; "For now, enjoy a cup of coffee :)" br; "Or, "
                a href="/contact" { "drop me a line" } "! I'd love to hear "
                "from you!"
            }
        },
    }
}

pub fn internal_server_error<'a, E: std::fmt::Debug>(err: E) -> ErrorPage<'a> {
    ErrorPage {
        title: "Ouch, something went wrong",
        code: StatusCode::INTERNAL_SERVER_ERROR,
        image_src: "/static/img/crash.jpg",
        image_alt: "something makes a big splash",
        message: html! {
            (format!("{:?}", err))
        },
    }
}
