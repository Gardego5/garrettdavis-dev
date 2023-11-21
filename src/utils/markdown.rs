use maud::{Markup, PreEscaped, Render};
use pulldown_cmark::{html, Event, LinkType, MetadataBlockKind, Options, Parser, Tag, TagEnd};
use serde::de::DeserializeOwned;
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

const OPTIONS: Options = Options::empty()
    .union(Options::ENABLE_HEADING_ATTRIBUTES)
    .union(Options::ENABLE_SMART_PUNCTUATION)
    .union(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);

static TAG_ATTRIBUTES: LazyLock<HashMap<&'static str, HashSet<&'static str>>> =
    LazyLock::new(|| {
        hashmap! {
            "code" => hashset!["class"],
            "h1" => hashset!["id"],
            "h2" => hashset!["id"],
            "h3" => hashset!["id"],
            "h4" => hashset!["id"],
            "h5" => hashset!["id"],
            "h6" => hashset!["id"],
            "a" => hashset!["href"],
        }
    });

#[derive(Debug)]
pub struct Markdown<Metadata> {
    pub doc: Markup,
    pub metadata: Metadata,
}

#[derive(Debug)]
pub enum MarkdownParseError {
    Yaml(serde_yaml::Error),
    NoMetadata,
}

impl<Metadata> Markdown<Metadata>
where
    Metadata: DeserializeOwned,
{
    pub fn parse(raw_doc: &str) -> Result<Self, MarkdownParseError> {
        // state for yaml metadata
        let mut in_yaml = false;
        let mut metadata: Option<Result<Metadata, serde_yaml::Error>> = None;

        // state for heading links and auto ids
        let mut unsafe_html = String::new();
        let mut in_heading = Vec::new();
        let mut heading_text = String::new();

        let parser = Parser::new_ext(raw_doc, OPTIONS).filter_map(|event| match event {
            // capture yaml metadata, expecting it to be in the specified format
            Event::Start(Tag::MetadataBlock(MetadataBlockKind::YamlStyle)) => {
                in_yaml = true;
                None
            }
            Event::End(TagEnd::MetadataBlock(MetadataBlockKind::YamlStyle)) => {
                in_yaml = false;
                None
            }
            Event::Text(raw_yaml) if in_yaml => match metadata {
                // TODO: implement a function that abstracts this, so that when
                // metadata is something like a Vec, we can push to it instead
                // of just panicing. Right now we expect there to be only one
                // metadata block.
                Some(_) => panic!("Found multiple yaml metadata blocks in markdown document"),
                None => {
                    let _ = metadata.insert(serde_yaml::from_str(raw_yaml.as_ref()));
                    None
                }
            },

            // heading links and auto ids
            Event::Start(Tag::Heading { .. }) => {
                in_heading.push(event);
                None
            }
            Event::End(TagEnd::Heading(_)) => match in_heading.first() {
                Some(Event::Start(Tag::Heading { id: Some(id), .. })) => {
                    let (open_tag, contents) = in_heading.split_at(1);
                    let mut result = String::new();
                    html::push_html(&mut result, open_tag.iter().map(ToOwned::to_owned));
                    html::push_html(&mut result, Self::h_anchor(id, &heading_text).into_iter());
                    html::push_html(&mut result, contents.iter().map(ToOwned::to_owned));
                    html::push_html(&mut result, std::iter::once(event)); // close tag

                    in_heading.clear();
                    heading_text.clear();

                    Some(Event::Html(result.into()))
                }
                Some(Event::Start(Tag::Heading {
                    level,
                    classes,
                    id: None,
                    ..
                })) => {
                    heading_text = heading_text
                        .chars()
                        .filter_map(|ch| match ch {
                            'a'..='z' | 'A'..='Z' | '0'..='9' => Some(ch),
                            ' ' | '.' => Some('-'),
                            _ => None,
                        })
                        .collect::<String>()
                        .to_lowercase();

                    let mut result = format!("<{level} id=\"{heading_text}\"");

                    if classes.is_empty() {
                        result.push('>');
                    } else {
                        result.push_str(" class=\"");
                        for class in classes {
                            result.push_str(class.as_ref());
                            result.push(' ');
                        }
                        result.push_str("\">");
                    }

                    result.push_str(format!("<a href=\"#{heading_text}\">#</a>").as_str());
                    html::push_html(
                        &mut result,
                        in_heading.iter().skip(1).map(ToOwned::to_owned),
                    );
                    result.push_str(format!("</{level}>").as_str());

                    in_heading.clear();
                    heading_text.clear();

                    Some(Event::Html(result.into()))
                }
                _ => unreachable!(
                    "Expected first event in heading to be heading open tag. Found: {:?}",
                    event
                ),
            },
            Event::Text(text) if in_heading.is_empty() == false => {
                heading_text.push_str(text.as_ref());
                in_heading.push(Event::Text(text));
                None
            }
            Event::Code(text) if in_heading.is_empty() == false => {
                heading_text.push_str(text.as_ref());
                in_heading.push(Event::Code(text));
                None
            }
            event if in_heading.is_empty() == false => {
                in_heading.push(event);
                None
            }

            _ => Some(event),
        });

        html::push_html(&mut unsafe_html, parser);

        match metadata {
            None => Err(MarkdownParseError::NoMetadata),
            Some(Err(err)) => Err(MarkdownParseError::Yaml(err)),
            Some(Ok(metadata)) => Ok(Self {
                doc: PreEscaped(Self::sanitize(&unsafe_html)),
                metadata,
            }),
        }
    }

    fn sanitize(unsafe_html: &str) -> String {
        ammonia::Builder::default()
            .tag_attributes(TAG_ATTRIBUTES.clone())
            .clean(unsafe_html)
            .to_string()
    }

    fn h_anchor<'a: 'b, 'b>(id: &'a str, title: &'a str) -> [Event<'b>; 3] {
        [
            Event::Start(Tag::Link {
                link_type: LinkType::Inline,
                dest_url: format!("#{id}").into(),
                title: title.into(),
                id: "".into(),
            }),
            Event::Text("#".into()),
            Event::End(TagEnd::Link),
        ]
    }
}

pub trait RenderWithContent {
    fn render(&self, content: &Markup) -> Markup;
}

impl<Metadata: RenderWithContent> Render for Markdown<Metadata> {
    fn render(&self) -> Markup {
        self.metadata.render(&self.doc)
    }
}

pub fn get_raw_markdown(prefix: &str, path: &str) -> Option<String> {
    std::fs::read_to_string(format!("markdown/{prefix}/{path}.md")).ok()
}
