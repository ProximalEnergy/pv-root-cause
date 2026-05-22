use leptos::prelude::*;

#[component]
pub fn FilterPanel(
    categories: Vec<String>,
    active_category: ReadSignal<Option<String>>,
    set_active_category: WriteSignal<Option<String>>,
) -> impl IntoView {
    view! {
        <section class="filter-panel" aria-label="Category filters">
            <p class="filter-panel__label">"Category"</p>
            <div class="filter-panel__controls">
                <button
                    class=move || {
                        if active_category.get().is_none() {
                            "filter-panel__button filter-panel__button--active"
                        } else {
                            "filter-panel__button"
                        }
                    }
                    type="button"
                    on:click=move |_| set_active_category.set(None)
                >
                    "All"
                </button>
                <For each=move || categories.clone() key=|category| category.clone() let:category>
                    {
                        let button_category = category.clone();
                        let selected_category = category.clone();

                        view! {
                            <button
                                class=move || {
                                    if active_category.get().as_ref() == Some(&selected_category) {
                                        "filter-panel__button filter-panel__button--active"
                                    } else {
                                        "filter-panel__button"
                                    }
                                }
                                type="button"
                                on:click=move |_| {
                                    set_active_category.set(Some(button_category.clone()));
                                }
                            >
                                {category}
                            </button>
                        }
                    }
                </For>
            </div>
        </section>
    }
}
