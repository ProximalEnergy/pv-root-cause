use leptos::prelude::*;

const GITHUB_NEW_ISSUE_URL: &str = "https://github.com/ProximalEnergy/pv-root-cause/issues/new";

#[component]
pub fn SubmitPage() -> impl IntoView {
    view! {
        <section class="submit-page">
            <p class="submit-page__eyebrow">"Community submissions"</p>
            <h1 class="submit-page__title">"Submit a Root Cause"</h1>
            <p class="submit-page__intro">
                "Root-cause posts in this compendium are content entries: a title, category, tags, "
                "field evidence, detection method, impact, mitigation, and supporting references. "
                "You do not need coding knowledge to suggest one."
            </p>

            <section class="submit-page__panel" aria-labelledby="submit-page-github-title">
                <h2 id="submit-page-github-title" class="submit-page__section-title">
                    "Use GitHub Issues"
                </h2>
                <p class="submit-page__text">
                    "Open a GitHub issue with the root cause you want to add. Include whatever you have: "
                    "photos or diagrams, symptoms seen in the field, how the condition was detected, "
                    "the expected performance impact, recommended mitigation, and any public sources."
                </p>
                <p class="submit-page__text">
                    "A maintainer can turn the issue into the site content format, so the submission can "
                    "start as plain language notes."
                </p>
                <a class="submit-page__button" href=GITHUB_NEW_ISSUE_URL>
                    "Open a GitHub Issue"
                </a>
            </section>
        </section>
    }
}
