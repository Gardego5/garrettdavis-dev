use {
    crate::{
        http::{
            components::{
                layout::{header, margins},
                template::template,
            },
            context::{Ctx, ValidBody},
            errors,
        },
        utils::markdown::{get_raw_markdown, Markdown, RenderWithContent},
    },
    axum::{extract::Path, response::IntoResponse, routing::get, Router},
    chrono::{DateTime, Utc},
    maud::{html, Markup, Render},
    serde::Deserialize,
};

pub fn router<B: ValidBody>() -> Router<Ctx, B> {
    Router::new().route("/*path", get(blog_page))
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct FrontMatter {
    title: String,
    author: String,
    description: String,
    live: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl RenderWithContent for FrontMatter {
    fn render(&self, content: &Markup) -> Markup {
        let head = html! {
            title { (self.title) }

            meta name="description" content=(self.description);
            meta name="author" content=(self.author);

            link rel="preload" href="/static/js/3p/prism@1.29.0.min.js" as="script";
            script defer src="/static/js/3p/prism@1.29.0.min.js"
                onload="Prism.plugins.autoloader.languages_path='/static/js/3p/prism/';" { }
        };

        let body = html! {
            (header(self.title.as_str()))
            (margins(html! { section class="markdown" { (content) }}))
        };

        template(head, body)
    }
}

async fn blog_page(Path((path,)): Path<(String,)>) -> Result<Markup, impl IntoResponse> {
    let raw_markdown = match get_raw_markdown("blog", &path) {
        Some(raw_md) => raw_md,
        None => return Err(errors::not_found()),
    };

    match Markdown::<FrontMatter>::parse(raw_markdown.as_ref()) {
        Ok(doc) => {
            if doc.metadata.live {
                Ok(doc.render())
            } else {
                Err(errors::not_found())
            }
        }
        Err(err) => Err(errors::internal_server_error(err)),
    }
}
