use leptos::prelude::*;

#[component]
pub fn TagsPage() -> impl IntoView {
    view! {
        <section class="tags-page">
            <h1 class="tags-page__title">"Tags"</h1>
            <p class="tags-page__body">"The taxonomy index will be generated from Markdown front matter."</p>
        </section>
    }
}
