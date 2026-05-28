use crate::components::TagPill;
use crate::models::{CauseContent, CauseContributor};
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

const CAUSE_CONTENT_JSON: &str = include_str!("../../public/cause_content.json");

#[component]
pub fn CauseDetailPage() -> impl IntoView {
    let params = use_params_map();

    let active_cause = move || {
        params
            .read()
            .get("id")
            .and_then(|cause_id| find_cause(&cause_id))
    };

    view! {
        {move || match active_cause() {
            Some(cause) => view! { <CauseDetail cause=cause/> }.into_any(),
            None => view! { <CauseDetailNotFound/> }.into_any(),
        }}
    }
}

#[component]
fn CauseDetail(cause: CauseContent) -> impl IntoView {
    let front_matter = cause.front_matter.clone();
    let contributors = if cause.front_matter.contributors.is_empty() {
        vec![CauseContributor {
            name: "Kurt Rhee".to_string(),
            url: "https://github.com/kurt-rhee".to_string(),
        }]
    } else {
        cause.front_matter.contributors.clone()
    };

    view! {
        <article class="cause-detail">
            <p class="cause-detail__eyebrow">"Cause reference"</p>
            <h1 class="cause-detail__title">{front_matter.title.clone()}</h1>
            <div class="cause-detail__tags" aria-label="Tags">
                <For each=move || front_matter.tags.clone() key=|tag| tag.clone() let:tag>
                    <TagPill label=tag/>
                </For>
            </div>

            <section class="cause-detail__metadata" aria-labelledby="cause-metadata-title">
                <h2 class="cause-detail__section-title" id="cause-metadata-title">
                    "Overview"
                </h2>
                <dl class="cause-detail__checklist">
                    <div class="cause-detail__checklist-item">
                        <dt class="cause-detail__checklist-key">"Category"</dt>
                        <dd class="cause-detail__checklist-value">{cause.front_matter.category.clone()}</dd>
                    </div>
                    <div class="cause-detail__checklist-item">
                        <dt class="cause-detail__checklist-key">"Severity"</dt>
                        <dd class=format!(
                            "cause-detail__checklist-value cause-detail__severity cause-detail__severity--{}",
                            cause.front_matter.severity.to_string().to_lowercase()
                        )>
                            {cause.front_matter.severity.to_string()}
                        </dd>
                    </div>
                    <div class="cause-detail__checklist-item">
                        <dt class="cause-detail__checklist-key">"Impact factor"</dt>
                        <dd class="cause-detail__checklist-value">
                            {cause.front_matter.impact_factor.clone()}
                        </dd>
                    </div>
                    <div class="cause-detail__checklist-item">
                        <dt class="cause-detail__checklist-key">"Detection method"</dt>
                        <dd class="cause-detail__checklist-value">
                            {cause.front_matter.detection_method.clone()}
                        </dd>
                    </div>
                    <div class="cause-detail__checklist-item">
                        <dt class="cause-detail__checklist-key">"Mitigation"</dt>
                        <dd class="cause-detail__checklist-value">
                            {cause.front_matter.mitigation.clone()}
                        </dd>
                    </div>
                    <div class="cause-detail__checklist-item">
                        <dt class="cause-detail__checklist-key">"Contributors"</dt>
                        <dd class="cause-detail__checklist-value">
                            <For
                                each=move || contributors.clone()
                                key=|contributor| contributor.url.clone()
                                let:contributor
                            >
                                <a href=contributor.url>{contributor.name}</a>
                            </For>
                        </dd>
                    </div>
                </dl>
            </section>

            <section class="cause-detail__content" aria-labelledby="cause-content-title">
                <h2 class="cause-detail__section-title" id="cause-content-title">
                    "Engineering Reference"
                </h2>
                <div class="cause-detail__prose" inner_html=cause.body_html.clone()></div>
            </section>
        </article>
    }
}

#[component]
fn CauseDetailNotFound() -> impl IntoView {
    view! {
        <section class="cause-detail cause-detail--missing">
            <p class="cause-detail__eyebrow">"Cause reference"</p>
            <h1 class="cause-detail__title">"Cause Not Found"</h1>
            <p class="cause-detail__body">
                "No Markdown-backed cause matches this slug."
            </p>
        </section>
    }
}

fn find_cause(cause_id: &str) -> Option<CauseContent> {
    load_causes()
        .into_iter()
        .find(|cause| cause.id() == cause_id)
}

fn load_causes() -> Vec<CauseContent> {
    serde_json::from_str(CAUSE_CONTENT_JSON).unwrap_or_default()
}
