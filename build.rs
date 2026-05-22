use pulldown_cmark::{Event, Options, Parser, html};
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

#[allow(dead_code)]
mod models {
    include!("src/models.rs");
}

use models::{CauseContent, CauseFrontMatter, SearchRecord};

fn main() {
    if let Err(error) = build_content() {
        panic!("content build failed: {error}");
    }
}

fn build_content() -> Result<(), Box<dyn Error>> {
    let content_dir = Path::new("content");
    let public_dir = Path::new("public");
    let search_index_path = public_dir.join("search_index.json");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", content_dir.display());

    let markdown_files = markdown_files(content_dir)?;
    let mut causes = Vec::with_capacity(markdown_files.len());
    let mut ids = HashSet::new();

    for path in markdown_files {
        println!("cargo:rerun-if-changed={}", path.display());

        let cause = parse_cause_file(&path)?;
        if !ids.insert(cause.id().to_string()) {
            return Err(format!(
                "{}: id `{}` is duplicated by another content file",
                path.display(),
                cause.id()
            )
            .into());
        }

        causes.push(cause);
    }

    causes.sort_by(|left, right| left.id().cmp(right.id()));

    let search_records: Vec<SearchRecord> = causes.iter().map(SearchRecord::from).collect();
    fs::create_dir_all(public_dir)?;
    fs::write(
        &search_index_path,
        serde_json::to_string_pretty(&search_records)?,
    )?;

    Ok(())
}

fn markdown_files(root: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut files = Vec::new();
    collect_markdown_files(root, &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_markdown_files(path: &Path, files: &mut Vec<PathBuf>) -> Result<(), Box<dyn Error>> {
    if !path.exists() {
        return Err(format!("{} does not exist", path.display()).into());
    }

    println!("cargo:rerun-if-changed={}", path.display());

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            collect_markdown_files(&path, files)?;
        } else if path.extension().is_some_and(|extension| extension == "md") {
            files.push(path);
        }
    }

    Ok(())
}

fn parse_cause_file(path: &Path) -> Result<CauseContent, Box<dyn Error>> {
    let source = fs::read_to_string(path)?;
    let (front_matter_source, body_markdown) = split_front_matter(path, &source)?;
    let front_matter: CauseFrontMatter = serde_yaml::from_str(front_matter_source)
        .map_err(|error| format!("{}: front matter parse error: {error}", path.display()))?;

    front_matter
        .validate()
        .map_err(|error| format!("{}: {}", path.display(), error))?;

    let body_markdown = body_markdown.trim().to_string();
    let body_html = render_markdown(&body_markdown);
    let cause = CauseContent {
        front_matter,
        body_markdown,
        body_html,
        source_path: path.display().to_string(),
    };

    cause
        .validate()
        .map_err(|error| format!("{}: {}", path.display(), error))?;

    Ok(cause)
}

fn split_front_matter<'a>(
    path: &Path,
    source: &'a str,
) -> Result<(&'a str, &'a str), Box<dyn Error>> {
    let Some(remainder) = source.strip_prefix("---\n") else {
        return Err(format!(
            "{}: missing YAML front matter opening `---`",
            path.display()
        )
        .into());
    };

    let Some((front_matter, body)) = remainder.split_once("\n---\n") else {
        return Err(format!(
            "{}: missing YAML front matter closing `---`",
            path.display()
        )
        .into());
    };

    Ok((front_matter, body))
}

fn render_markdown(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(markdown, options)
        .filter(|event| !matches!(event, Event::Html(_) | Event::InlineHtml(_)));

    let mut output = String::new();
    html::push_html(&mut output, parser);
    output
}
