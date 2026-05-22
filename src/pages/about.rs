use leptos::prelude::*;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <section class="about-page">
            <h1 class="about-page__title">"Methodology"</h1>
            <p class="about-page__body">
                "This static page will describe the field diagnostic workflow and verification criteria."
            </p>
        </section>
    }
}
