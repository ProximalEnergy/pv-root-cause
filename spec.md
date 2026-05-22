Technical Specification: PV Underperformance Compendium Static Site
This concise specification defines the architecture, data pipeline, and layout for a high-performance, pre-rendered compendium tracking photovoltaic (PV) site underperformance root causes.

1. Project Overview & Tech Stack
Framework: Leptos (Rust) using Static Site Generation (SSG) 

Content Source: Local Markdown files with YAML Front Matter metadata.

Styling: A single global plain CSS file (public/styles.css) using strict BEM conventions. No CSS frameworks or scoped-CSS crates.

Search: Client-side fuzzy search using a pre-compiled JSON index loaded into WebAssembly (WASM).

Architecture Rule: Every UI component must strictly reside in its own separate file.

2. Directory Layout
Plaintext
pv-compendium/
├── Cargo.toml               # Project configuration
├── build.rs                 # Compile-time markdown parser & search index generator
├── content/                 # Markdown file repository (organized by category folders)
├── public/
│   ├── assets/              # Technical diagrams, EL images, and IV curves
│   ├── search_index.json    # Generated at build time for client-side search
│   └── styles.css           # Monolithic global stylesheet
└── src/
    ├── main.rs              # App entry point
    ├── lib.rs               # Router configuration and base layout
    ├── models.rs            # Shared Rust data structures
    ├── components/          # Isolated UI components (one per file)
    │   ├── mod.rs, header.rs, footer.rs, search_bar.rs, 
    │   └── filter_panel.rs, cause_card.rs, tag_pill.rs, image_gallery.rs
    └── pages/               # Top-level standalone views
        ├── mod.rs, home.rs, about.rs, detail.rs, tags.rs
3. Data Schema & Content Engine
Metadata Fields (YAML Front Matter)
Each Markdown file requires:

id: Unique URL slug string.

title: Name of the underperformance root cause.

category: Broad classification (e.g., Modules, Inverters, BOS).

tags: Array of taxonomy labels for sub-filtering.

severity: Systemic impact level (Low, Medium, High).

impact_factor: Short summary of potential yield loss.

detection_method: Required diagnostic tools (e.g., Drone IR, EL imaging).

mitigation: Recommended corrective actions.

images: Array of file paths and matching captions.

Build-Time Pipeline (build.rs)
Scans the content/ directory at compile time.

Extracts metadata and renders raw Markdown body text into clean HTML strings.

Packs all document metadata arrays into a single, highly compressed public/search_index.json file.

Binds the pre-rendered HTML content directly into Leptos static route templates.

4. Routing & Page Architecture
/ (Home Dashboard): Fetches the static search index JSON upon initialization. Holds active search state and text queries in reactive signals. Maps filtered results out to a scannable dashboard grid.

/about (Methodology): A purely static layout outlining standard field diagnostic criteria (e.g., IEC 62446 verification pathways).

/tags (Taxonomy Index): An alphabetical directory listing every unique tag with an associated item counter. Clicking a tag links back to the home view with filters pre-loaded.

/cause/:id (Detail Viewer): The primary engineering reference page. Formats structural metadata into a clear key-value checklist, followed by the pre-rendered description text and data charts.

5. Client-Side Fuzzy Search Algorithm
Execution: Powered by a fast Rust matching crate compiled to WASM.

Target Fields: Matches user input against a weighted data string: Title (Highest weight), Tags (Medium weight), and Category (Lowest weight).

Performance Optimization: Search logic executes lazily within a derived selector. Computations only re-run when the search input value or active category selection actively mutates.

6. Global CSS Blueprint (public/styles.css)
Layout Isolation: Uses explicit Block-Element-Modifier (BEM) class naming matching component file names (e.g., .search-wrapper__input, .cause-card__title) to simulate encapsulation and prevent style collisions.

Render Safety: Relies on standard block styling, clearfixes, inline formatting, and absolute positioning templates. Layout architecture avoids flexbox or grid implementations at global parent layout containers to prevent rendering anomalies during static page compilation sweeps.
