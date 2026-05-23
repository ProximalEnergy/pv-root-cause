use crate::components::{CauseCard, FilterPanel, SearchBar};
use crate::models::SearchRecord;
use crate::search::search_causes;
use leptos::prelude::*;
use leptos_router::hooks::use_query_map;
use std::collections::BTreeSet;

const SEARCH_INDEX_JSON: &str = include_str!("../../public/search_index.json");

#[component]
pub fn HomePage() -> impl IntoView {
    let initial_records = load_search_records();
    let categories = unique_categories(&initial_records);
    let (records, _set_records) = signal(initial_records);
    let query_map = use_query_map();
    let (query, set_query) = signal(
        query_map.get().get("tag").unwrap_or_default(),
    );
    let (active_category, set_active_category) = signal(None::<String>);

    let filtered_records = move || {
        let search_text = query.get();
        let selected_category = active_category.get();

        let mut results = search_causes(&search_text, &records.get());

        if let Some(category) = selected_category {
            results.retain(|sc| sc.record.category == category);
        }

        results.into_iter().map(|sc| sc.record).collect::<Vec<_>>()
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
