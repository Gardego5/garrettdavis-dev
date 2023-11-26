use crate::http::{
    components::{
        layout::{header, margins},
        template::template,
    },
    context::ValidBody,
    errors::AppError,
};
use aws_sdk_sesv2::types::{Body, Content, Destination, EmailContent, Message as EmailMessage};
use axum::extract::State;
use axum::response::IntoResponse;
use axum_extra::extract::{cookie::Cookie, CookieJar};
use maud::DOCTYPE;
use pulldown_cmark::CowStr;
use serde_json::json;
use {
    crate::http::context::Ctx,
    axum::{
        routing::{get, post},
        Form, Router,
    },
    maud::{html, Markup},
    serde::{Deserialize, Serialize},
};

pub fn router<B: ValidBody>() -> Router<Ctx, B>
where
    <B as axum::body::HttpBody>::Error: Send + Sync + Into<axum::BoxError>,
    <B as axum::body::HttpBody>::Data: Send,
{
    Router::new()
        .route("/", get(contact))
        .route("/", post(post_message))
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct Message {
    pub first: String,
    pub last: String,
    pub email: String,
    pub message: String,
}

fn automated_email(message: &Message) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta http-equiv="Content-Type" content="text/html; charset=utf-8";
                meta name="viewport" content="width=device-width";
                title { "New message from contact form" }
            }
            body {
                div id="email" style="width: 600px" {
                    p { "You have a new message from the contact form on garrettdavis.dev!" }
                    p { "Name: " (message.first) " " (message.last) }
                    p {
                        "Email: "
                        a href={"mailto:" (message.email)} {
                            (message.email)
                        }
                    }
                    p { "Message: " (message.message) }
                }
            }
        }
    }
}

fn thanks_message(data: &Message) -> Markup {
    html! {
        div class="rounded border border-slate-500 bg-gray-800 p-4 text-center" {
            "Thanks for reaching out, " (data.first) "! I'll get back to you soon :)"
        }
    }
}

async fn post_message(
    State(ctx): State<Ctx>,
    jar: CookieJar,
    Form(data): Form<Message>,
) -> Result<impl IntoResponse, AppError> {
    let subject = Content::builder()
        .data("[garrettdavis.dev alert] - message")
        .charset("UTF-8")
        .build()
        .expect("building subject_content");
    let body = Content::builder()
        .data(automated_email(&data))
        .charset("UTF-8")
        .build()
        .expect("building body_content");

    let msg = EmailMessage::builder()
        .subject(subject)
        .body(Body::builder().html(body).build())
        .build();

    let email_content = EmailContent::builder().simple(msg).build();

    let dest = Destination::builder()
        .to_addresses(&ctx.cfg.alerts_to_email)
        .build();

    ctx.ses
        .send_email()
        .from_email_address("contact-form@noreply.garrettdavis.dev")
        .destination(dest)
        .content(email_content)
        .send()
        .await?;

    Ok((
        jar.add(Cookie::new(
            "ContactMessageForm",
            CowStr::from(json!(data).to_string()),
        )),
        thanks_message(&data),
    ))
}

async fn contact(cookies: CookieJar) -> Markup {
    let head = html! {
        title { "Contact Garrett" }
        meta name="description" content="Contact Garrett for any reason.";
    };

    let main = match cookies
        .get("ContactMessageForm")
        .and_then(|s| serde_json::from_str::<Message>(s.value()).ok())
    {
        None => contact_form(),
        Some(data) => thanks_message(&data),
    };

    let body = html! {
        (header("Contact Garrett"))
        (margins(main))
    };

    template(head, body)
}

fn contact_form() -> Markup {
    html! {
        form class="relative grid gap-2 rounded border border-slate-500 bg-gray-800 p-4 md:grid-cols-4"
            hx-post="/contact" hx-swap="outerHTML"
        {
            h2 class="text-2xl font-semibold md:col-span-4" { "I'd love to hear from you!" }
            input class="px-2 py-1 rounded border bg-zinc-900 border-slate-500"
                required type="text" name="first" placeholder="First Name";
            input class="px-2 py-1 rounded border bg-zinc-900 border-slate-500"
                required type="text" name="last" placeholder="Last Name";
            input class="px-2 py-1 rounded border bg-zinc-900 border-slate-500 col-span-2"
                required type="email" name="email" placeholder="Email";
            textarea class="px-2 py-1 rounded border bg-zinc-900 border-slate-500 col-span-full min-h-[20vh] leading-5 md:col-span-4"
                required name="message" placeholder="Send me a message :)" {}
            button class="absolute -bottom-1.5 right-8 rounded border border-slate-500 bg-zinc-900 px-4 py-1 text-sm hover:bg-slate-800"
                type="submit" { "Send" };
        }
    }
}
