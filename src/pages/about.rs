use leptos::prelude::*;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <section class="about-page">
            <h1 class="about-page__title">"About This Compendium"</h1>

            <section class="about-page__section">
                <h2 class="about-page__section-title">"Purpose"</h2>
                <p class="about-page__text">
                    "This website catalogues photovoltaic (PV) underperformance root causes "
                    "for diagnostic purposes. "
                    "It is an open-source, project aimed at helping the industry.  "
                    "The community is invited to contribute by adding
                    new root cause entries and improving existing ones."
                </p>
            </section>

            <section class="about-page__section">
                <h2 class="about-page__section-title">"Entry Structure"</h2>
                <p class="about-page__text">
                    "Every root cause entry provides:"
                </p>
                <ul class="about-page__list">
                    <li class="about-page__list-item">
                        <strong>"Category:"</strong>
                        " Physical location or subsystem (e.g. Modules, Wiring, Inverter)."
                    </li>
                    <li class="about-page__list-item">
                        <strong>"Severity:"</strong>
                        " Relative impact rating — Low (cosmetic or minor yield loss), "
                        "Medium (notable degradation), High (critical failure or safety risk)."
                    </li>
                    <li class="about-page__list-item">
                        <strong>"Impact factor:"</strong>
                        " Description of the performance or reliability consequence."
                    </li>
                    <li class="about-page__list-item">
                        <strong>"Detection method:"</strong>
                        " Field technique used to identify the condition."
                    </li>
                    <li class="about-page__list-item">
                        <strong>"Mitigation:"</strong>
                        " Recommended corrective or preventive action."
                    </li>
                    <li class="about-page__list-item">
                        <strong>"Tags:"</strong>
                        " Cross-reference taxonomy labels for multi-dimensional filtering."
                    </li>
                </ul>
            </section>

            <section class="about-page__section">
                <h2 class="about-page__section-title">"Content Sources"</h2>
                <p class="about-page__text">
                    "Entries are compiled from published field inspection reports, manufacturer technical "
                    "notes, and peer-reviewed PV reliability research. Each source is referenced in the "
                    "associated Markdown content file."
                </p>
            </section>
        </section>
    }
}
