use maud::{html, Markup, Render};

pub struct PieceOfInfo {
    title: String,
    content: String,
}

impl Render for PieceOfInfo {
    fn render(&self) -> Markup {
        html! {
            div class="relative inline-block bg-zinc-900 pb-2 text-slate-300" {
                span class="absolute bottom-0 right-0 text-xs text-slate-600" { (self.title) }
                (self.content)
            }
        }
    }
}

