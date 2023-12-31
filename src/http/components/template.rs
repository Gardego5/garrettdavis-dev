use maud::{html, Markup, Render, DOCTYPE};

fn head<C: Render>(content: C) -> Markup {
    html! {
        head {
            (content)

            // Meta Tags
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1";

            // Tailwind CSS
            link rel="stylesheet" href="/static/css/tailwind.css";

            // Local Third-Party Scripts
            script src="/static/js/3p/htmx@0.9.6.min.js" {}
            link rel="preload" as="script" href="/static/js/3p/iconify-icon@1.0.8.min.js";
            script defer src="/static/js/3p/iconify-icon@1.0.8.min.js" {}
            // Even though we want to defer the execution of Alpine.js, we don't want to delay it's loading.
            link rel="preload" as="script" href="/static/js/3p/alpinejs-morph@3.13.1.min.js";
            link rel="preload" as="script" href="/static/js/3p/alpinejs@3.13.1.min.js";
            script defer src="/static/js/3p/alpinejs-morph@3.13.1.min.js" {}
            script defer src="/static/js/3p/alpinejs@3.13.1.min.js" {}

            // Google Fonts
            link rel="preconnect" href="https://fonts.googleapis.com";
            link rel="preconnect" href="https://fonts.gstatic.com" crossorigin;
            link rel="stylesheet"
                href="https://fonts.googleapis.com/css2?family=IBM+Plex+Mono:ital@0;1&family=Montserrat&display=block";
        }
    }
}

pub fn template<H: Render, B: Render>(head_content: H, body_content: B) -> Markup {
    html! { (DOCTYPE)
        html lang="en" {
            (head(head_content))
            body class="box-border bg-zinc-900 text-zinc-50" { (body_content) }
        }
    }
}
