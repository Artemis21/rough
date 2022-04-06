//! Rough is a very simple and very opinionated static site generator.
use clap::Parser;
use pulldown_cmark as md;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::{fs, io};
use tera::Context;
use yaml_front_matter::{Document, YamlFrontMatter};

mod inline_imgs;

/// A result for any error type.
type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
/// Render a Rough site.
struct Args {
    /// The path to the folder containing the site source.
    src: PathBuf,
    /// The path to a folder to write the compiled site to.
    out: PathBuf,
}

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
    let rendered = tera.render(name, &Context::from_serialize(context)?)?;
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
    let doc: Document<serde_yaml::Value> = YamlFrontMatter::parse(&fs::read_to_string(from)?)?;
    let parser = md::Parser::new_ext(&doc.content, md::Options::all());
    let parser = inline_imgs::InlineImages::new(parser);
    let mut content = String::new();
    md::html::push_html(&mut content, parser);
    let mut context = serde_yaml::Mapping::new();
    context.insert("meta".into(), doc.metadata.clone());
    context.insert("content".into(), content.into());
    let context = context.into();
    render_template(tera, "project.html", to, &context)?;
    Ok(doc.metadata)
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
    fs::create_dir_all(from)?;
    copy_dir_all(from.join("static"), &to.join("static"))?;
    let mut tera = tera::Tera::new(from.join("*.html").to_str().ok_or("could not parse path")?)?;
    tera.autoescape_on(vec![]);
    let projects = render_projects(&tera, from.join("projects"), &to.join("projects"))?;
    let mut context = serde_yaml::Mapping::new();
    context.insert("projects".into(), projects.into());
    render_template(&tera, "index.html", to.join("index.html"), &context.into())
}

/// Parse CLI args and generate a site.
fn main() {
    let args = Args::parse();
    if args.src.is_dir() {
        render_site(&args.src, &args.out).unwrap();
    } else {
        println!("'{}' is not a directory", args.out.display());
    }
}
