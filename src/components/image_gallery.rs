use crate::models::CauseImage;
use leptos::prelude::*;

#[component]
pub fn ImageGallery(images: Vec<CauseImage>) -> impl IntoView {
    if images.is_empty() {
        return ().into_any();
    }

    view! {
        <section class="image-gallery" aria-labelledby="image-gallery-title">
            <h2 class="image-gallery__title" id="image-gallery-title">"Image Gallery"</h2>
            <div class="image-gallery__items">
                <For each=move || images.clone() key=|image| image.path.clone() let:image>
                    <GalleryImage image=image/>
                </For>
            </div>
        </section>
    }
    .into_any()
}

#[component]
fn GalleryImage(image: CauseImage) -> impl IntoView {
    let (image_failed, set_image_failed) = signal(false);
    let alt_text = image.alt.clone().unwrap_or_else(|| image.caption.clone());
    let image_src = image.path.clone();
    let image_path = image.path.clone();
    let caption = image.caption.clone();

    view! {
        <figure class="image-gallery__item">
            <Show
                when=move || !image_failed.get()
                fallback=|| {
                    view! {
                        <div class="image-gallery__fallback" role="img" aria-label="Image unavailable">
                            "Image unavailable"
                        </div>
                    }
                }
            >
                <img
                    class="image-gallery__image"
                    src=image_src.clone()
                    alt=alt_text.clone()
                    loading="lazy"
                    on:error=move |_| set_image_failed.set(true)
                />
            </Show>
            <figcaption class="image-gallery__caption">
                <span class="image-gallery__caption-text">{caption}</span>
                <code class="image-gallery__path">{image_path}</code>
            </figcaption>
        </figure>
    }
}
