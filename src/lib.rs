#![recursion_limit = "256"]

pub mod components;
pub mod models;
pub mod pages;
pub mod search;

use components::{Footer, Header};
use leptos::prelude::*;
use leptos_meta::{MetaTags, Title, provide_meta_context};
use leptos_router::SsrMode;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
use leptos_router::static_routes::{StaticParamsMap, StaticRoute};
use models::SearchRecord;
use pages::{AboutPage, CauseDetailPage, HomePage, SubmitPage, TagsPage};

const SEARCH_INDEX_JSON: &str = include_str!("../public/search_index.json");

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link rel="stylesheet" href="/styles.css"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="PV Underperformance Compendium"/>
        <Router>
            <div class="app-shell">
                <Header/>
                <main class="app-shell__main">
                    <Routes fallback=not_found>
                        <Route
                            path=path!("/")
                            view=HomePage
                            ssr=SsrMode::Static(StaticRoute::new())
                        />
                        <Route
                            path=path!("/about")
                            view=AboutPage
                            ssr=SsrMode::Static(StaticRoute::new())
                        />
                        <Route
                            path=path!("/tags")
                            view=TagsPage
                            ssr=SsrMode::Static(StaticRoute::new())
                        />
                        <Route
                            path=path!("/submit")
                            view=SubmitPage
                            ssr=SsrMode::Static(StaticRoute::new())
                        />
                        <Route
                            path=path!("/cause/:id")
                            view=CauseDetailPage
                            ssr=SsrMode::Static(cause_static_route())
                        />
                    </Routes>
                </main>
                <Footer/>
            </div>
        </Router>
    }
}

fn cause_static_route() -> StaticRoute {
    StaticRoute::new().prerender_params(|| async {
        let mut params = StaticParamsMap::new();
        params.insert("id", cause_ids());
        params
    })
}

fn cause_ids() -> Vec<String> {
    serde_json::from_str::<Vec<SearchRecord>>(SEARCH_INDEX_JSON)
        .unwrap_or_default()
        .into_iter()
        .map(|record| record.id)
        .collect()
}

fn not_found() -> impl IntoView {
    view! {
        <section class="not-found">
            <h1 class="not-found__title">"Page Not Found"</h1>
            <p class="not-found__body">"The requested compendium page does not exist."</p>
        </section>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
