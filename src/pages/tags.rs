use crate::components::TagPill;
use crate::models::{SearchRecord, collect_tag_counts};
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

const SEARCH_INDEX_JSON: &str = include_str!("../../public/search_index.json");

#[component]
pub fn TagsPage() -> impl IntoView {
    let records: Vec<SearchRecord> =
        serde_json::from_str(SEARCH_INDEX_JSON).unwrap_or_default();
    let tag_counts = collect_tag_counts(&records);
    let navigate = use_navigate();

    view! {
        <section class="tags-page">
            <h1 class="tags-page__title">"Taxonomy Tags"</h1>
            <p class="tags-page__body">
                "Browse the cause catalog by taxonomy label."
            </p>
            <div class="tags-page__list">
                {tag_counts.into_iter().map(|tc| {
                    let label = tc.tag.clone();
                    let label_handler = tc.tag.clone();
                    let label_for_title = tc.tag;
                    let nav = navigate.clone();
                    view! {
                        <button
                            class="tags-page__tag"
                            on:click={move |_| {
                                let encoded = label_handler.replace(' ', "%20");
                                nav(&format!("/?tag={encoded}"), Default::default());
                            }}
                            title={format!("View causes tagged with \"{label_for_title}\"")}
                        >
                            <TagPill label={label}/>
                            <span class="tags-page__count">{tc.count}</span>
                        </button>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </section>
    }
}
