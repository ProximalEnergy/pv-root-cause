use crate::models::SearchRecord;

pub struct ScoredCause {
    pub record: SearchRecord,
    pub score: u32,
}

pub fn search_causes(query: &str, records: &[SearchRecord]) -> Vec<ScoredCause> {
    let query = query.trim();
    if query.is_empty() {
        return records
            .iter()
            .map(|r| ScoredCause {
                record: r.clone(),
                score: 0,
            })
            .collect();
    }

    let query_lower = query.to_lowercase();
    let mut scored: Vec<ScoredCause> = Vec::new();

    for record in records {
        let mut score: u32 = 0;

        if let Some(s) = fuzzy_match(&record.title, &query_lower) {
            score = score.saturating_add(s.saturating_mul(100));
        }

        for tag in &record.tags {
            if let Some(s) = fuzzy_match(tag, &query_lower) {
                score = score.saturating_add(s.saturating_mul(50));
            }
        }

        if let Some(s) = fuzzy_match(&record.category, &query_lower) {
            score = score.saturating_add(s.saturating_mul(25));
        }

        if score > 0 {
            scored.push(ScoredCause {
                record: record.clone(),
                score,
            });
        }
    }

    scored.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then(a.record.title.cmp(&b.record.title))
    });

    scored
}

fn fuzzy_match(text: &str, query: &str) -> Option<u32> {
    if query.is_empty() {
        return Some(0);
    }

    let text_lower = text.to_lowercase();
    let query_lower = query.to_lowercase();
    let tc: Vec<char> = text_lower.chars().collect();
    let qc: Vec<char> = query_lower.chars().collect();

    if qc.len() > tc.len() {
        return None;
    }

    let mut qi = 0;
    let mut first_pos = None;
    let mut consecutive: u32 = 0;
    let mut max_consecutive: u32 = 0;
    let mut last_match_pos: Option<usize> = None;

    for (ti, &ch) in tc.iter().enumerate() {
        if ch == qc[qi] {
            if qi == 0 {
                first_pos = Some(ti);
            }
            if let Some(prev) = last_match_pos {
                if ti == prev + 1 {
                    consecutive += 1;
                } else {
                    consecutive = 0;
                }
            }
            max_consecutive = max_consecutive.max(consecutive);
            last_match_pos = Some(ti);
            qi += 1;
            if qi == qc.len() {
                let start = first_pos.unwrap_or(0) as u32;
                let base = 1000u32;
                let position_bonus = if start == 0 {
                    500
                } else {
                    500u32.saturating_sub(start * 10)
                };
                let consecutive_bonus = max_consecutive * 50;
                return Some(base + position_bonus + consecutive_bonus);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Severity;

    fn record(id: &str, title: &str, category: &str, tags: &[&str]) -> SearchRecord {
        SearchRecord {
            id: id.to_string(),
            title: title.to_string(),
            category: category.to_string(),
            tags: tags.iter().map(|s| s.to_string()).collect(),
            severity: Severity::Medium,
            impact_factor: String::new(),
            route: format!("/cause/{id}"),
        }
    }

    #[test]
    fn exact_match_succeeds() {
        assert!(fuzzy_match("Hotspot", "hotspot").is_some());
    }

    #[test]
    fn case_insensitive_match() {
        assert!(fuzzy_match("Module Hotspot", "HOTSPOT").is_some());
    }

    #[test]
    fn partial_in_order_match() {
        assert!(fuzzy_match("Module Hotspot from Cell Damage", "hotspot").is_some());
    }

    #[test]
    fn no_match_returns_none() {
        assert!(fuzzy_match("Hotspot", "xyz").is_none());
    }

    #[test]
    fn out_of_order_chars_dont_match() {
        assert!(fuzzy_match("hotspot", "sthop").is_none());
    }

    #[test]
    fn empty_query_matches() {
        assert_eq!(fuzzy_match("anything", ""), Some(0));
    }

    #[test]
    fn empty_query_returns_all() {
        let records = vec![
            record("a", "Hotspot", "Modules", &[]),
            record("b", "Diode", "Modules", &[]),
        ];
        let results = search_causes("", &records);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn title_ranks_above_tags() {
        let records = vec![
            record("a", "Thermal Runaway", "Modules", &["yield loss"]),
            record("b", "Yield Loss Analysis", "Modules", &["thermal"]),
        ];
        let results = search_causes("thermal", &records);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].record.id, "a");
    }

    #[test]
    fn tags_rank_above_category() {
        let records = vec![
            record("a", "Something Else", "Modules", &["thermal anomaly"]),
            record("b", "Other Thing", "Thermal Analysis", &["wiring"]),
        ];
        let results = search_causes("thermal", &records);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].record.id, "a");
    }

    #[test]
    fn no_match_returns_empty() {
        let records = vec![record("a", "Hotspot", "Modules", &["thermal"])];
        let results = search_causes("nonexistent", &records);
        assert!(results.is_empty());
    }

    #[test]
    fn case_insensitive_search() {
        let records = vec![record("a", "Module Hotspot", "Modules", &[])];
        let results = search_causes("HOTSPOT", &records);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn query_changes_affect_results() {
        let records = vec![
            record("a", "Hotspot", "Modules", &["thermal"]),
            record("b", "Bypass Diode", "Electrics", &["wiring"]),
        ];

        let hotspot_results = search_causes("hot", &records);
        assert_eq!(hotspot_results.len(), 1);
        assert_eq!(hotspot_results[0].record.id, "a");

        let wiring_results = search_causes("wiring", &records);
        assert_eq!(wiring_results.len(), 1);
        assert_eq!(wiring_results[0].record.id, "b");
    }
}
