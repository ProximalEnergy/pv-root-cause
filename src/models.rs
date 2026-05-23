use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CauseFrontMatter {
    pub id: String,
    pub title: String,
    pub category: String,
    pub tags: Vec<String>,
    pub severity: Severity,
    pub impact_factor: String,
    pub detection_method: String,
    pub mitigation: String,
    pub images: Vec<CauseImage>,
}

impl CauseFrontMatter {
    pub fn validate(&self) -> Result<(), ValidationError> {
        let mut errors = Vec::new();

        validate_slug("id", &self.id, &mut errors);
        validate_required("title", &self.title, &mut errors);
        validate_required("category", &self.category, &mut errors);
        validate_required("impact_factor", &self.impact_factor, &mut errors);
        validate_required("detection_method", &self.detection_method, &mut errors);
        validate_required("mitigation", &self.mitigation, &mut errors);

        if self.tags.is_empty() {
            errors.push("tags must include at least one taxonomy label".to_string());
        }

        if self.images.is_empty() {
            errors.push("images must include at least one referenced asset".to_string());
        }

        for (index, tag) in self.tags.iter().enumerate() {
            validate_required(&format!("tags[{index}]"), tag, &mut errors);
        }

        for (index, image) in self.images.iter().enumerate() {
            validate_required(&format!("images[{index}].path"), &image.path, &mut errors);
            validate_required(
                &format!("images[{index}].caption"),
                &image.caption,
                &mut errors,
            );
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(ValidationError { errors })
        }
    }

    pub fn route_path(&self) -> String {
        format!("/cause/{}", self.id)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CauseImage {
    pub path: String,
    pub caption: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alt: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CauseContent {
    pub front_matter: CauseFrontMatter,
    pub body_markdown: String,
    pub body_html: String,
    pub source_path: String,
}

impl CauseContent {
    pub fn validate(&self) -> Result<(), ValidationError> {
        let mut errors = match self.front_matter.validate() {
            Ok(()) => Vec::new(),
            Err(error) => error.errors,
        };

        validate_required("body_markdown", &self.body_markdown, &mut errors);
        validate_required("body_html", &self.body_html, &mut errors);
        validate_required("source_path", &self.source_path, &mut errors);

        if errors.is_empty() {
            Ok(())
        } else {
            Err(ValidationError { errors })
        }
    }

    pub fn id(&self) -> &str {
        &self.front_matter.id
    }

    pub fn route_path(&self) -> String {
        self.front_matter.route_path()
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Severity {
    Low,
    Medium,
    High,
}

impl fmt::Display for Severity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Low => formatter.write_str("Low"),
            Severity::Medium => formatter.write_str("Medium"),
            Severity::High => formatter.write_str("High"),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SearchRecord {
    pub id: String,
    pub title: String,
    pub category: String,
    pub tags: Vec<String>,
    pub severity: Severity,
    pub impact_factor: String,
    pub route: String,
}

impl From<&CauseContent> for SearchRecord {
    fn from(cause: &CauseContent) -> Self {
        let front_matter = &cause.front_matter;

        Self {
            id: front_matter.id.clone(),
            title: front_matter.title.clone(),
            category: front_matter.category.clone(),
            tags: front_matter.tags.clone(),
            severity: front_matter.severity,
            impact_factor: front_matter.impact_factor.clone(),
            route: front_matter.route_path(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TagCount {
    pub tag: String,
    pub count: usize,
}

pub fn collect_tag_counts(records: &[SearchRecord]) -> Vec<TagCount> {
    let mut counts = std::collections::BTreeMap::new();
    for record in records {
        for tag in &record.tags {
            *counts.entry(tag.clone()).or_insert(0) += 1;
        }
    }
    counts
        .into_iter()
        .map(|(tag, count)| TagCount { tag, count })
        .collect()
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationError {
    pub errors: Vec<String>,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.errors.join("; "))
    }
}

impl std::error::Error for ValidationError {}

fn validate_required(field: &str, value: &str, errors: &mut Vec<String>) {
    if value.trim().is_empty() {
        errors.push(format!("{field} is required"));
    }
}

fn validate_slug(field: &str, value: &str, errors: &mut Vec<String>) {
    validate_required(field, value, errors);

    if value.is_empty() {
        return;
    }

    let valid = value.chars().all(|character| {
        character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
    }) && !value.starts_with('-')
        && !value.ends_with('-')
        && !value.contains("--");

    if !valid {
        errors.push(format!(
            "{field} must be a lowercase URL slug using letters, numbers, and single hyphens"
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_front_matter_maps_to_cause_route() {
        let front_matter = CauseFrontMatter {
            id: "module-hotspot".to_string(),
            title: "Module hotspot".to_string(),
            category: "Modules".to_string(),
            tags: vec!["thermal".to_string()],
            severity: Severity::High,
            impact_factor: "Localized power loss and fire risk".to_string(),
            detection_method: "Drone IR inspection".to_string(),
            mitigation: "Replace affected module".to_string(),
            images: vec![CauseImage {
                path: "/assets/modules/hotspot-ir.svg".to_string(),
                caption: "IR image showing a hotspot".to_string(),
                alt: None,
            }],
        };

        assert_eq!(front_matter.validate(), Ok(()));
        assert_eq!(front_matter.route_path(), "/cause/module-hotspot");
    }

    #[test]
    fn missing_required_fields_fail_validation() {
        let front_matter = CauseFrontMatter {
            id: "Invalid Slug".to_string(),
            title: String::new(),
            category: "Modules".to_string(),
            tags: Vec::new(),
            severity: Severity::Medium,
            impact_factor: String::new(),
            detection_method: "  ".to_string(),
            mitigation: String::new(),
            images: vec![CauseImage {
                path: String::new(),
                caption: String::new(),
                alt: None,
            }],
        };

        let error = front_matter.validate().expect_err("invalid metadata");

        assert!(error.errors.iter().any(|message| message.contains("id")));
        assert!(error.errors.iter().any(|message| message.contains("title")));
        assert!(error.errors.iter().any(|message| message.contains("tags")));
        assert!(
            error
                .errors
                .iter()
                .any(|message| message.contains("images"))
        );
        assert!(
            error
                .errors
                .iter()
                .any(|message| message.contains("detection_method"))
        );
    }

    #[test]
    fn collect_tag_counts_deduplicates_and_sorts() {
        let records = vec![
            SearchRecord {
                id: "a".to_string(),
                title: "A".to_string(),
                category: "C".to_string(),
                tags: vec!["thermal".to_string(), "crack".to_string()],
                severity: Severity::High,
                impact_factor: String::new(),
                route: "/cause/a".to_string(),
            },
            SearchRecord {
                id: "b".to_string(),
                title: "B".to_string(),
                category: "C".to_string(),
                tags: vec!["thermal".to_string(), "bypass".to_string()],
                severity: Severity::Medium,
                impact_factor: String::new(),
                route: "/cause/b".to_string(),
            },
        ];
        let counts = collect_tag_counts(&records);
        assert_eq!(counts.len(), 3);
        assert_eq!(counts[0], TagCount { tag: "bypass".to_string(), count: 1 });
        assert_eq!(counts[1], TagCount { tag: "crack".to_string(), count: 1 });
        assert_eq!(counts[2], TagCount { tag: "thermal".to_string(), count: 2 });
    }

    #[test]
    fn collect_tag_counts_empty_records() {
        assert!(collect_tag_counts(&[]).is_empty());
    }
}
