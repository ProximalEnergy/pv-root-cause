use leptos::prelude::*;

#[component]
pub fn TagPill(label: String) -> impl IntoView {
    view! {
        <span class="tag-pill">{label}</span>
    }
}
