use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[path = "../content_build.rs"]
mod content_build;

const VALID_CAUSE_A: &str = r#"---
id: duplicate-id
title: First Cause
category: Modules
tags:
  - thermal
severity: High
impact_factor: Localized field impact.
detection_method: Drone infrared inspection.
mitigation: Replace affected module.
images:
  - path: /assets/modules/hotspot-ir.svg
    caption: Thermal image showing a localized hotspot.
    alt: Thermal module diagram.
---

## Notes

Valid body content.
"#;

const VALID_CAUSE_B: &str = r#"---
id: second-id
title: Second Cause
category: Wiring
tags:
  - connector
severity: Low
impact_factor: Minor field impact.
detection_method: Visual inspection.
mitigation: Re-seat connector.
images:
  - path: /assets/modules/hotspot-ir.svg
    caption: Connector inspection reference image.
---

## Notes

Another valid body.
"#;

struct TestProject {
    root: PathBuf,
    content_dir: PathBuf,
    public_dir: PathBuf,
}

impl TestProject {
    fn new(name: &str) -> Self {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time")
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "pv-root-cause-{name}-{}-{unique}",
            std::process::id()
        ));
        let content_dir = root.join("content");
        let public_dir = root.join("public");
        fs::create_dir_all(content_dir.join("modules")).expect("create content directory");
        fs::create_dir_all(&public_dir).expect("create public directory");

        Self {
            root,
            content_dir,
            public_dir,
        }
    }

    fn write_content(&self, relative_path: &str, markdown: &str) {
        let path = self.content_dir.join(relative_path);
        let parent = path.parent().expect("content path parent");
        fs::create_dir_all(parent).expect("create content parent");
        fs::write(path, markdown).expect("write content");
    }
}

impl Drop for TestProject {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[test]
fn duplicate_ids_fail_build() {
    let project = TestProject::new("duplicate");
    project.write_content("modules/first.md", VALID_CAUSE_A);
    project.write_content("modules/second.md", VALID_CAUSE_A);

    let error = build_error(&project);

    assert!(error.contains("duplicated"));
    assert!(error.contains("duplicate-id"));
}

#[test]
fn missing_image_captions_fail_build() {
    let project = TestProject::new("caption");
    let missing_caption = VALID_CAUSE_A.replace(
        "    caption: Thermal image showing a localized hotspot.\n",
        "    caption: \"\"\n",
    );
    project.write_content("modules/missing-caption.md", &missing_caption);

    let error = build_error(&project);

    assert!(error.contains("images[0].caption"));
}

#[test]
fn invalid_severity_values_fail_build() {
    let project = TestProject::new("severity");
    let invalid_severity = VALID_CAUSE_A.replace("severity: High", "severity: Critical");
    project.write_content("modules/invalid-severity.md", &invalid_severity);

    let error = build_error(&project);

    assert!(error.contains("front matter parse error"));
    assert!(error.contains("severity"));
}

#[test]
fn search_index_contains_every_valid_cause_once() {
    let project = TestProject::new("search-index");
    project.write_content("modules/first.md", VALID_CAUSE_A);
    project.write_content("wiring/second.md", VALID_CAUSE_B);

    let records =
        content_build::build_content_from_paths(&project.content_dir, &project.public_dir)
            .expect("valid content builds");

    assert_eq!(records.len(), 2);
    assert_eq!(records[0].id, "duplicate-id");
    assert_eq!(records[1].id, "second-id");

    let index_path = project.public_dir.join("search_index.json");
    let index: Vec<content_build::SearchRecord> =
        serde_json::from_str(&fs::read_to_string(index_path).expect("read search index"))
            .expect("parse search index");

    assert_eq!(index, records);
}

fn build_error(project: &TestProject) -> String {
    content_build::build_content_from_paths(&project.content_dir, &project.public_dir)
        .expect_err("content build should fail")
        .to_string()
}

#[test]
fn repository_content_builds_and_writes_generated_json() {
    let project = TestProject::new("repository");
    copy_markdown_tree(Path::new("content"), &project.content_dir);

    let records =
        content_build::build_content_from_paths(&project.content_dir, &project.public_dir)
            .expect("repository content builds");
    let index_path = project.public_dir.join("search_index.json");
    let content_path = project.public_dir.join("cause_content.json");

    assert!(!records.is_empty());
    assert!(index_path.exists());
    assert!(content_path.exists());
}

fn copy_markdown_tree(source: &Path, destination: &Path) {
    for entry in fs::read_dir(source).expect("read content directory") {
        let entry = entry.expect("content entry");
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());

        if source_path.is_dir() {
            fs::create_dir_all(&destination_path).expect("create copied directory");
            copy_markdown_tree(&source_path, &destination_path);
        } else if source_path
            .extension()
            .is_some_and(|extension| extension == "md")
        {
            fs::copy(&source_path, &destination_path).expect("copy markdown");
        }
    }
}
