use crate::models::{collect_tag_counts, SearchRecord};
use leptos::prelude::*;
use leptos_router::components::A;
use std::collections::BTreeMap;

const SEARCH_INDEX_JSON: &str = include_str!("../../public/search_index.json");

#[component]
pub fn TagsPage() -> impl IntoView {
    let records: Vec<SearchRecord> = serde_json::from_str(SEARCH_INDEX_JSON).unwrap_or_default();
    let tag_groups = grouped_tag_index(&records);

    view! {
        <section class="tags-page">
            <h1 class="tags-page__title">"Tag Index"</h1>
            <p class="tags-page__body">
                "Browse taxonomy labels alphabetically and jump to matching causes."
            </p>

            <div class="tags-page__index" aria-label="Tag index">
                {tag_groups.into_iter().map(|group| {
                    let letter = group.letter;

                    view! {
                        <section class="tags-page__group" aria-labelledby=format!("tag-group-{letter}")>
                            <h2 class="tags-page__letter" id=format!("tag-group-{letter}")>
                                {letter.to_string()}
                            </h2>
                            <ul class="tags-page__list">
                                {group.entries.into_iter().map(|entry| {
                                    let count_label = format!("({})", entry.count);

                                    view! {
                                        <li class="tags-page__item">
                                            <A
                                                attr:class="tags-page__link"
                                                href=format!("/?tag={}", query_encode(&entry.label))
                                            >
                                                <span class="tags-page__name">{entry.label}</span>
                                                <span class="tags-page__count">{count_label}</span>
                                            </A>
                                        </li>
                                    }
                                }).collect::<Vec<_>>()}
                            </ul>
                        </section>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </section>
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct TagIndexGroup {
    letter: char,
    entries: Vec<TagIndexEntry>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct TagIndexEntry {
    label: String,
    count: usize,
}

fn grouped_tag_index(records: &[SearchRecord]) -> Vec<TagIndexGroup> {
    let mut groups = BTreeMap::<char, Vec<TagIndexEntry>>::new();

    for tag_count in collect_tag_counts(records) {
        let letter = tag_count
            .tag
            .chars()
            .find(|character| character.is_ascii_alphanumeric())
            .map(|character| character.to_ascii_uppercase())
            .unwrap_or('#');

        groups.entry(letter).or_default().push(TagIndexEntry {
            label: tag_count.tag,
            count: tag_count.count,
        });
    }

    groups
        .into_iter()
        .map(|(letter, entries)| TagIndexGroup { letter, entries })
        .collect()
}

fn query_encode(value: &str) -> String {
    value.replace(' ', "%20")
}
