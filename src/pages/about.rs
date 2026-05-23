use leptos::prelude::*;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <section class="about-page">
            <h1 class="about-page__title">"About This Compendium"</h1>

            <section class="about-page__section">
                <h2 class="about-page__section-title">"Purpose"</h2>
                <p class="about-page__text">
                    "This static reference catalogues verified photovoltaic (PV) underperformance root causes "
                    "for field-diagnostic use. Each entry is derived from published field studies, "
                    "manufacturer documentation, and inspection standards."
                </p>
            </section>

            <section class="about-page__section">
                <h2 class="about-page__section-title">"Diagnostic Workflow"</h2>
                <p class="about-page__text">
                    "Causes are organised by the following diagnostic workflow stages:"
                </p>
                <ol class="about-page__list">
                    <li class="about-page__list-item">
                        <strong>"Data review:"</strong>
                        " Analyse operational data (I-V curves, string currents, yield ratios) to identify "
                        "underperforming circuits."
                    </li>
                    <li class="about-page__list-item">
                        <strong>"Thermal survey:"</strong>
                        " Deploy drone or handheld IR thermography to locate temperature anomalies "
                        "indicating cell damage, bypass diode activation, or connection faults."
                    </li>
                    <li class="about-page__list-item">
                        <strong>"Visual inspection:"</strong>
                        " Examine modules, wiring, and connectors for physical damage, soiling, "
                        "delamination, or corrosion."
                    </li>
                    <li class="about-page__list-item">
                        <strong>"Electrical testing:"</strong>
                        " Perform insulation resistance (IR) and I-V curve tracing to quantify "
                        "mismatch, shunt resistance loss, and series resistance increase."
                    </li>
                    <li class="about-page__list-item">
                        <strong>"Root cause classification:"</strong>
                        " Correlate findings against this compendium to assign a verified root cause "
                        "and recommended mitigation."
                    </li>
                </ol>
            </section>

            <section class="about-page__section">
                <h2 class="about-page__section-title">"Field Validation Reference"</h2>
                <p class="about-page__text">
                    "All causes listed here meet the field-verification criteria outlined in the "
                    "IEC 62446 series (PV systems — Requirements for testing, documentation and "
                    "maintenance) and relevant IEC 61215 / IEC 61730 qualification standards."
                </p>
                <p class="about-page__text">
                    "Each cause includes a detection method that references a specific field-validated "
                    "technique, ensuring that every entry can be confirmed or ruled out through "
                    "standard inspection or electrical testing procedures."
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
                <h2 class="about-page__section-title">"Applicable Standards"</h2>
                <ul class="about-page__list">
                    <li class="about-page__list-item">
                        <strong>"IEC 62446-1:"</strong>
                        " Grid-connected PV systems — Documentation, commissioning tests and inspection."
                    </li>
                    <li class="about-page__list-item">
                        <strong>"IEC 62446-2:"</strong>
                        " Grid-connected PV systems — Main PV system maintenance."
                    </li>
                    <li class="about-page__list-item">
                        <strong>"IEC 61215:"</strong>
                        " Terrestrial photovoltaic modules — Design qualification and type approval."
                    </li>
                    <li class="about-page__list-item">
                        <strong>"IEC 61730:"</strong>
                        " Photovoltaic module safety qualification."
                    </li>
                    <li class="about-page__list-item">
                        <strong>"IEC 60904:"</strong>
                        " Photovoltaic devices — Measurement principles for I-V characterisation."
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
