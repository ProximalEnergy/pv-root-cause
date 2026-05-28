use leptos::prelude::*;

#[component]
pub fn ContributorsPage() -> impl IntoView {
    view! {
        <section class="about-page">
            <h1 class="about-page__title">"Contributors"</h1>

            <section class="about-page__section">
                <h2 class="about-page__section-title">"Content Contributors"</h2>
                <ul class="about-page__list">
                    <li class="about-page__list-item">
                        <a href="https://github.com/kurt-rhee">"Kurt Rhee"</a>
                    </li>
                </ul>
            </section>
        </section>
    }
}
