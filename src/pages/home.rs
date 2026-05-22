use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <section class="home-page">
            <p class="home-page__eyebrow">"Static reference dashboard"</p>
            <h1 class="home-page__title">"PV Underperformance Root Causes"</h1>
            <p class="home-page__intro">
                "Search, filtering, and content-backed cause cards will be added after the build-time content pipeline is in place."
            </p>
        </section>
    }
}
