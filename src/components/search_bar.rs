use leptos::prelude::*;

#[component]
pub fn SearchBar(query: ReadSignal<String>, set_query: WriteSignal<String>) -> impl IntoView {
    let on_input = move |event| set_query.set(event_target_value(&event));

    view! {
        <label class="search-bar">
            <span class="search-bar__label">"Search root causes"</span>
            <input
                class="search-bar__input"
                type="search"
                placeholder="Search title, category, or tag"
                prop:value=move || query.get()
                on:input=on_input
            />
        </label>
    }
}
