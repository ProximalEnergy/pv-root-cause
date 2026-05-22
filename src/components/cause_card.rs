use crate::components::TagPill;
use crate::models::SearchRecord;
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn CauseCard(record: SearchRecord) -> impl IntoView {
    view! {
        <article class="cause-card">
            <div class="cause-card__meta">
                <span class="cause-card__category">{record.category.clone()}</span>
                <span class=format!(
                    "cause-card__severity cause-card__severity--{}",
                    record.severity.to_string().to_lowercase()
                )>
                    {record.severity.to_string()}
                </span>
            </div>
            <h2 class="cause-card__title">
                <A attr:class="cause-card__link" href=record.route.clone()>
                    {record.title.clone()}
                </A>
            </h2>
            <p class="cause-card__impact">{record.impact_factor.clone()}</p>
            <div class="cause-card__tags" aria-label="Tags">
                <For each=move || record.tags.clone() key=|tag| tag.clone() let:tag>
                    <TagPill label=tag/>
                </For>
            </div>
        </article>
    }
}
