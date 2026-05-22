use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="site-header">
            <a class="site-header__brand" href="/">"PV Underperformance Compendium"</a>
            <nav class="site-header__nav" aria-label="Primary navigation">
                <A attr:class="site-header__link" href="/">"Dashboard"</A>
                <A attr:class="site-header__link" href="/tags">"Tags"</A>
                <A attr:class="site-header__link" href="/about">"About"</A>
            </nav>
        </header>
    }
}
