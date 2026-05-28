use crate::models::{CauseContent, CauseContributor};
use leptos::prelude::*;
use std::collections::BTreeMap;

const CAUSE_CONTENT_JSON: &str = include_str!("../../public/cause_content.json");

#[component]
pub fn ContributorsPage() -> impl IntoView {
    let contributors = contributor_summaries();

    view! {
        <section class="about-page">
            <h1 class="about-page__title">"Contributors"</h1>

            <section class="about-page__section">
                <h2 class="about-page__section-title">"Content Contributors"</h2>
                <div class="contributors-page__list">
                    <For
                        each=move || contributors.clone()
                        key=|contributor| contributor.url.clone()
                        let:contributor
                    >
                        <ContributorAccordion contributor=contributor/>
                    </For>
                </div>
            </section>
        </section>
    }
}

#[component]
fn ContributorAccordion(contributor: ContributorSummary) -> impl IntoView {
    let contribution_count = contributor.contributions.len();
    let contributions = contributor.contributions.clone();

    view! {
        <details class="contributors-page__item">
            <summary class="contributors-page__summary">
                <a class="contributors-page__name" href=contributor.url>
                    {contributor.name}
                </a>
                <span class="contributors-page__count">" ("{contribution_count}")"</span>
            </summary>
            <ul class="contributors-page__contributions">
                <For
                    each=move || contributions.clone()
                    key=|contribution| contribution.route.clone()
                    let:contribution
                >
                    <li class="contributors-page__contribution">
                        <a href=contribution.route>{contribution.title}</a>
                    </li>
                </For>
            </ul>
        </details>
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ContributorSummary {
    name: String,
    url: String,
    contributions: Vec<ContributionLink>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ContributionLink {
    title: String,
    route: String,
}

fn contributor_summaries() -> Vec<ContributorSummary> {
    let causes: Vec<CauseContent> = serde_json::from_str(CAUSE_CONTENT_JSON).unwrap_or_default();
    let mut contributors: BTreeMap<(String, String), Vec<ContributionLink>> = BTreeMap::new();

    for cause in causes {
        let cause_contributors = if cause.front_matter.contributors.is_empty() {
            vec![default_contributor()]
        } else {
            cause.front_matter.contributors.clone()
        };

        for contributor in cause_contributors {
            contributors
                .entry((contributor.name, contributor.url))
                .or_default()
                .push(ContributionLink {
                    title: cause.front_matter.title.clone(),
                    route: cause.route_path(),
                });
        }
    }

    let mut summaries: Vec<ContributorSummary> = contributors
        .into_iter()
        .map(|((name, url), mut contributions)| {
            contributions.sort_by(|left, right| left.title.cmp(&right.title));
            ContributorSummary {
                name,
                url,
                contributions,
            }
        })
        .collect();

    summaries.sort_by(|left, right| {
        right
            .contributions
            .len()
            .cmp(&left.contributions.len())
            .then_with(|| left.name.cmp(&right.name))
    });

    summaries
}

fn default_contributor() -> CauseContributor {
    CauseContributor {
        name: "Kurt Rhee".to_string(),
        url: "https://github.com/kurt-rhee".to_string(),
    }
}
