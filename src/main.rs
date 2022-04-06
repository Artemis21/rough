//! Rough is a very simple and very opinionated static site generator.
use pulldown_cmark as md;
use std::path::{Path, PathBuf};
use std::{fs, io};
use tera::Context;

mod args;
mod errors;
mod inline_imgs;
mod yaml;

use errors::Error;

/// A result for any error type.
type Result<T> = std::result::Result<T, Error>;

/// Copy a directory recursively.
fn copy_dir_all(from: PathBuf, to: &Path) -> io::Result<()> {
    fs::create_dir_all(to)?;
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            copy_dir_all(entry.path(), &to.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), to.join(entry.file_name()))?;
        }
    }
    Ok(())
}

/// Render a template called `name` which `tera` has already parsed. Use
/// `context` as context for rendering and write the rendered file to `to`.
fn render_template(
    tera: &tera::Tera,
    name: &str,
    to: PathBuf,
    context: &serde_yaml::Value,
) -> Result<()> {
    let context = &Context::from_serialize(context).expect("context serialisation failed");
    let rendered = tera.render(name, context)?;
    if let Some(parent) = to.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(to, rendered)?;
    Ok(())
}

/// Render the source project file at `from`, and write the rendered file to
/// `to`, using `tera` for template rendering. `tera` should be pre-populated
/// with a template called `project.html`.
fn render_project(tera: &tera::Tera, from: PathBuf, to: PathBuf) -> Result<serde_yaml::Value> {
    let (metadata, content) = yaml::parse(&fs::read_to_string(from)?)?;
    let parser = md::Parser::new_ext(&content, md::Options::all());
    let parser = inline_imgs::InlineImages::new(parser);
    let mut content = String::new();
    md::html::push_html(&mut content, parser);
    let context = yaml::MapBuilder::default()
        .set("meta", metadata.clone())
        .set("content", content)
        .build();
    render_template(tera, "project.html", to, &context)?;
    Ok(metadata)
}

/// Render all source project files within `from` to their corresponding
/// files within `to`, using `tera` for template rendering. `tera` should
/// be pre-populated with a template called `project.html`.
fn render_projects(tera: &tera::Tera, from: PathBuf, to: &Path) -> Result<Vec<serde_yaml::Value>> {
    fs::read_dir(from)?
        .map(|file| {
            let file = file?;
            if file.file_type()?.is_file() {
                let mut dest = to.join(file.file_name());
                dest.set_extension("html");
                Ok(Some(render_project(tera, file.path(), dest)?))
            } else {
                Ok(None)
            }
        })
        .collect::<Result<Vec<_>>>()
        .map(|ctxs| ctxs.into_iter().flatten().collect())
}

/// Render a full Rough site from the directory `from` to the directory `to`.
fn render_site(from: &Path, to: &Path) -> Result<()> {
    let from = std::fs::canonicalize(from)?;
    fs::create_dir_all(&from)?;
    copy_dir_all(from.join("static"), &to.join("static"))?;
    let mut tera = tera::Tera::new(
        from.join("*.html")
            .to_str()
            .expect("could not decode source path"),
    )?;
    tera.autoescape_on(vec![]);
    let projects = render_projects(&tera, from.join("projects"), &to.join("projects"))?;
    let context = yaml::MapBuilder::default()
        .set("projects", projects)
        .build();
    render_template(&tera, "index.html", to.join("index.html"), &context)
}

/// Parse CLI args and generate a site.
fn main() {
    if let Some((from, to)) = args::parse() {
        if from.is_dir() {
            render_site(&from, &to).unwrap();
        } else {
            println!("'{}' is not a directory", from.display());
        }
    }
}
