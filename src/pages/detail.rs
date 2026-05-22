use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

#[component]
pub fn CauseDetailPage() -> impl IntoView {
    let params = use_params_map();
    let cause_id = move || {
        params
            .read()
            .get("id")
            .unwrap_or_else(|| "unknown".to_string())
    };

    view! {
        <section class="cause-detail">
            <p class="cause-detail__eyebrow">"Cause reference"</p>
            <h1 class="cause-detail__title">{cause_id}</h1>
            <p class="cause-detail__body">
                "Markdown-backed cause metadata and rendered body content will be connected in the content parser task."
            </p>
        </section>
    }
}
