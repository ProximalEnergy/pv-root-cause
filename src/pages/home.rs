use crate::components::{CauseCard, FilterPanel, SearchBar};
use crate::models::SearchRecord;
use leptos::prelude::*;
use std::collections::BTreeSet;

const SEARCH_INDEX_JSON: &str = include_str!("../../public/search_index.json");

#[component]
pub fn HomePage() -> impl IntoView {
    let initial_records = load_search_records();
    let categories = unique_categories(&initial_records);
    let (records, _set_records) = signal(initial_records);
    let (query, set_query) = signal(String::new());
    let (active_category, set_active_category) = signal(None::<String>);

    let filtered_records = move || {
        let search_text = query.get().trim().to_lowercase();
        let selected_category = active_category.get();

        records
            .get()
            .into_iter()
            .filter(|record| {
                selected_category
                    .as_ref()
                    .is_none_or(|category| record.category == *category)
            })
            .filter(|record| search_text.is_empty() || record_matches_query(record, &search_text))
            .collect::<Vec<_>>()
    };

    view! {
        <section class="home-page">
            <p class="home-page__eyebrow">"Static reference dashboard"</p>
            <h1 class="home-page__title">"PV Underperformance Root Causes"</h1>
            <p class="home-page__intro">
                "Scan generated field-diagnostic references by category, severity, impact, and taxonomy labels."
            </p>
            <div class="home-page__controls">
                <SearchBar query=query set_query=set_query/>
                <FilterPanel
                    categories=categories
                    active_category=active_category
                    set_active_category=set_active_category
                />
            </div>
            <div class="home-page__summary" aria-live="polite">
                {move || match filtered_records().len() {
                    1 => "1 matching cause".to_string(),
                    count => format!("{count} matching causes"),
                }}
            </div>
            <Show
                when=move || !filtered_records().is_empty()
                fallback=|| {
                    view! {
                        <div class="home-page__empty" role="status">
                            <h2 class="home-page__empty-title">"No matching causes"</h2>
                            <p class="home-page__empty-body">
                                "Adjust the search terms or clear the category filter."
                            </p>
                        </div>
                    }
                }
            >
                <div class="home-page__grid">
                    <For each=filtered_records key=|record| record.id.clone() let:record>
                        <CauseCard record=record/>
                    </For>
                </div>
            </Show>
        </section>
    }
}

fn load_search_records() -> Vec<SearchRecord> {
    serde_json::from_str(SEARCH_INDEX_JSON).unwrap_or_default()
}

fn unique_categories(records: &[SearchRecord]) -> Vec<String> {
    records
        .iter()
        .map(|record| record.category.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn record_matches_query(record: &SearchRecord, search_text: &str) -> bool {
    let title = record.title.to_lowercase();
    let category = record.category.to_lowercase();

    title.contains(search_text)
        || category.contains(search_text)
        || record
            .tags
            .iter()
            .any(|tag| tag.to_lowercase().contains(search_text))
}
