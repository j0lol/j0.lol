use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use comrak::{
    format_html_with_plugins,
    nodes::{AstNode, NodeValue},
    parse_document,
    plugins::syntect::SyntectAdapterBuilder,
    Arena, Options, Plugins,
};
use itertools::Itertools;
use miette::miette;
use miette::IntoDiagnostic;
use minijinja::{context, Environment};
use select::document::Document;
use select::predicate::{Name, Predicate};
use serde::Serialize;
use std::{fmt::Display, str::from_utf8};
use syntect::highlighting::ThemeSet;
use thiserror::Error;

fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &mut F)
where
    F: FnMut(&'a AstNode<'a>),
{
    f(node);
    for c in node.children() {
        iter_nodes(c, f);
    }
}

struct HtmlDate(DateTime<Utc>);
impl Display for HtmlDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.format("%B %e, %Y at %l:%M %p").fmt(f)
    }
}
impl HtmlDate {
    fn relative(&self) -> String {
        let duration = Utc::now() - self.0;

        if duration.num_days() > 365 {
            format!("on {}", self.0.format("%B %e, %Y"))
        } else if duration.num_days() > 30 {
            format!("{} months ago", duration.num_days() / 30)
        } else if duration.num_days() == 30 {
            "a month ago".to_string()
        } else if duration.num_days() > 7 {
            format!("{} weeks ago", duration.num_days() / 7)
        } else if duration.num_days() == 7 {
            "a week ago".to_string()
        } else if duration.num_days() > 1 {
            format!("{} days ago", duration.num_days())
        } else if duration.num_days() == 1 {
            "yesterday".to_string()
        } else if duration.num_hours() > 1 {
            format!("{} hours ago", duration.num_hours())
        } else if duration.num_hours() == 1 {
            "an hour ago".to_string()
        } else if duration.num_minutes() > 1 {
            format!("{} minutes ago", duration.num_minutes())
        } else {
            "just now".to_string()
        }
    }
}

struct HtmlDuration(Duration);
impl Display for HtmlDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("{} minute", self.0.num_minutes()).fmt(f)
    }
}

// #[derive(Template)]
// #[template(path = "post.html")]
// struct PostTemplate {
//     date: HtmlDate,
//     read_time: HtmlDuration,
//     content: String,
//     headers: Vec<Heading>,
//     title: String,
// }

#[derive(Serialize)]
struct Heading {
    level: u8,
    name: String,
    url: String,
}

#[derive(Debug, Error)]
pub enum PostRenderError {
    #[error("Post not published!")]
    PostNotPublished,

    #[error("Published post without date!")]
    PublishedPostWithoutDate,

    #[error("Front matter parse error!")]
    FrontMatterParseError(#[from] knuffel::Error),

    #[error("Template parse error!")]
    TemplateParseError(#[from] minijinja::Error),
    #[error("Ansi Parse error!")]
    Err(#[from] ansi_to_html::Error),
}

#[derive(Debug, knuffel::Decode)]
pub struct KdlFrontMatter {
    #[knuffel(child, unwrap(argument))]
    title: String,
    #[knuffel(child, unwrap(argument))]
    date: Option<String>,
    #[knuffel(child, unwrap(argument))]
    draft: Option<bool>,
}

pub fn read_post(content: String, env: &Environment) -> Result<String, PostRenderError> {
    let mut options = Options::default();
    options.render.unsafe_ = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    options.extension.header_ids = Some("".to_string());
    options.extension.footnotes = true;
    options.extension.front_matter_delimiter = Some("---".to_string());
    options.render.github_pre_lang = true;

    let adapter = SyntectAdapterBuilder::new()
        .theme_set(ThemeSet::load_from_folder(".").unwrap())
        .theme("Catppuccin Mocha")
        .build();

    let mut plugins = Plugins::default();
    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    // let content = markdown_to_html(&content, &options);
    // println!("{}", content);

    let arena = Arena::new();
    let root = parse_document(&arena, &content, &options);

    let mut front_matter: String = "".into();

    iter_nodes(root, &mut |node| match &mut node.data.borrow_mut().value {
        &mut NodeValue::FrontMatter(ref mut text) => {
            front_matter = text.to_owned();
        }
        node => {
            if let NodeValue::CodeBlock(text) = node {
                if text.info == *"ansi" {
                    *node = NodeValue::HtmlInline(format!(
                        "<pre lang=\"ansi\">{}</pre>",
                        ansi_to_html::convert(&text.literal).unwrap()
                    ))
                }
            }
        }
    });

    // let mut front_matter =
    //     knuffel::parse::<KdlFrontMatter>("front_matter.kdl", &front_matter)?;

    dbg!(&front_matter);
    front_matter = front_matter.replace("---", "");

    let mut front_matter = match knuffel::parse::<KdlFrontMatter>("front_matter.kdl", &front_matter)
    {
        Ok(front_matter) => front_matter,
        Err(e) => {
            println!("{:?}", e);
            std::process::exit(1);
        }
    };

    if front_matter.draft.is_some() {
        if std::env::var("SITE_ENV").is_ok_and(|v| v == "PROD") {
            return Err(PostRenderError::PostNotPublished);
        } else {
            front_matter.title += " (DRAFT)";
        }
    }

    let mut html = vec![];
    format_html_with_plugins(root, &options, &mut html, &plugins).unwrap();

    let content = from_utf8(&html).unwrap().to_string();

    let html_doc: Document = select::document::Document::from(&content as &str);

    let headers = html_doc
        .find(
            Name("h1")
                .or(Name("h2"))
                .or(Name("h3"))
                .or(Name("h4"))
                .or(Name("h5"))
                .or(Name("h6")),
        )
        .filter_map(|node| {
            node.find(Name("a"))
                .next()
                .and_then(|node| node.attr("href").map(|node| node.to_owned()))
                .map(|url| Heading {
                    name: node.text(),
                    level: match node.name() {
                        Some("h1") => 1,
                        Some("h2") => 2,
                        Some("h3") => 3,
                        Some("h4") => 4,
                        Some("h5") => 5,
                        Some("h6") => 6,
                        _ => unimplemented!(),
                    },
                    url,
                })
        })
        .collect_vec();

    // Ok(PostTemplate {
    //     date: HtmlDate(
    //         str_to_date(front_matter.date, front_matter.draft)
    //     ),
    //     read_time: HtmlDuration(Duration::try_minutes(5).unwrap()),
    //     content,
    //     headers,
    //     title: front_matter.title,
    // })
    let date = HtmlDate(str_to_date(front_matter.date, front_matter.draft));
    Ok(env.get_template("post.html")?.render(context! {
    date => date.to_string(),
    rel_date => date.relative(),
    read_time => HtmlDuration(Duration::try_minutes(5).unwrap()).to_string(),
    content,
    headers,
    title => front_matter.title,
    })?)
}

#[derive(Serialize)]
pub struct PostItem {
    title: String,
    date: String,
    path: String,
}

#[derive(Error, Debug)]
pub enum FrontMatterError {
    #[error("KDL Parse error")]
    KdlParseError(#[from] knuffel::Error),

    #[error("Io Error")]
    IoError(#[from] std::io::Error),
}
pub fn front_matter<T: AsRef<std::path::Path>>(
    path: T,
) -> Result<KdlFrontMatter, FrontMatterError> {
    let content = std::fs::read_to_string(path)?;

    let mut options = Options::default();
    options.extension.front_matter_delimiter = Some("---".to_string());
    let arena = Arena::new();
    let root = parse_document(&arena, &content, &options);
    let mut front_matter: String = "".into();

    iter_nodes(root, &mut |node| match &mut node.data.borrow_mut().value {
        &mut NodeValue::FrontMatter(ref mut text) => {
            front_matter = text.to_owned();
        }
        node => {
            if let NodeValue::CodeBlock(text) = node {
                if text.info == *"ansi" {
                    *node = NodeValue::HtmlInline(format!(
                        "<pre lang=\"ansi\">{}</pre>",
                        ansi_to_html::convert(&text.literal).unwrap()
                    ))
                }
            }
        }
    });
    front_matter = front_matter.replace("---", "");

    knuffel::parse::<KdlFrontMatter>("front_matter.kdl", &front_matter).map_err(|e| e.into())
}

pub fn str_to_date(str: Option<String>, draft: Option<bool>) -> DateTime<Utc> {
    if let Some(str) = str {
        NaiveDateTime::parse_from_str(&str, "%Y-%m-%d %H:%M")
            .unwrap()
            .and_utc()
    } else {
        if draft.is_none() || draft.is_some_and(|b| !b) {
            panic!("{}", PostRenderError::PublishedPostWithoutDate)
        }
        Utc::now()
    }
}

pub fn list_posts() -> Result<Vec<PostItem>, miette::Error> {
    let dirs = std::fs::read_dir("./posts/")
        .into_diagnostic()?
        .filter_map(Result::ok)
        .collect_vec();
    let mut post_items: Vec<PostItem> = vec![];

    for dir in dirs {
        let path = dir.path();
        let name = path.file_stem().ok_or(miette!("No file stem?"))?;
        if let Ok(front) = front_matter(dir.path()) {
            if front.draft.is_some_and(|b| b) {
                continue;
            }
            post_items.push(PostItem {
                title: front.title,
                date: HtmlDate(str_to_date(front.date, front.draft)).to_string(),
                path: name.to_string_lossy().to_string(),
            })
        }
    }

    // TODO: fix
    // post_items.sort_by_key(|post| post.date);
    post_items.reverse();

    Ok(post_items)
}
