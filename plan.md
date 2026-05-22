# Implementation Plan: PV Underperformance Compendium Static Site

## Overview
Build a Leptos SSG static site for PV underperformance root causes. Content comes from local Markdown with YAML front matter, parsed at build time into static pages and a client-side search index.

## Architecture Decisions
- Leptos SSG, not DB/API backed. spec requires local Markdown.
- True SSG required. Routes and content should be generated as static output, not runtime server-rendered.
- SSG standard is Leptos native `<StaticRoute>` pre-rendering with `cargo-leptos` plus a quick server-export run to emit real `.html` files.
- Markdown front matter is source of truth. `build.rs` validates metadata, renders HTML, emits `public/search_index.json`.
- One component per file under `src/components/`; one page per file under `src/pages/`.
- One global stylesheet at `public/styles.css`; BEM names only; no CSS framework.
- Search runs client-side in WASM from generated JSON; no runtime server dependency.
- Search index hard cap is 1.0 MB uncompressed. If exceeded, pivot to chunked indexing such as Pagefind.
- Markdown content and technical images are provided by the project owner.

## Dependency Graph
- Content schema (`src/models.rs`)
  - Markdown content files (`content/**`)
  - Build pipeline (`build.rs`)
    - Generated search index (`public/search_index.json`)
    - Static route data
      - Pages (`src/pages/**`)
        - Components (`src/components/**`)
          - CSS (`public/styles.css`)

## Task List

### Phase 1: Foundation
- [x] Task 1: Project structure and app shell
- [ ] Task 2: Content schema and sample cause
- [ ] Task 3: Build-time content parser

### Phase 2: Core Features
- [ ] Task 4: Home dashboard with category filtering
- [ ] Task 5: Cause detail route
- [ ] Task 6: Client-side fuzzy search
- [ ] Task 7: Taxonomy tags page
- [ ] Task 8: About methodology page

### Checkpoint: Core Features
- [ ] Static build generates pages and search index
- [ ] User can search, filter, open a cause, and browse tags

### Phase 3: Polish
- [ ] Task 9: Global BEM styling
- [ ] Task 10: Content validation and build checks
- [ ] Task 11: Performance and accessibility pass

### Checkpoint: Complete
- [ ] All acceptance criteria met
- [ ] Ready for review


## Task 2: Content schema and sample cause

**Description:** Define Rust data structures for front matter, rendered cause content, image metadata, severity, tags, and search records. Add one representative Markdown cause to prove the schema.

**Acceptance criteria:**
- [ ] Schema covers all required spec fields
- [ ] Missing required fields fail validation
- [ ] One sample Markdown file includes metadata, body, image captions, and tags
- [ ] Slug/id maps cleanly to `/cause/:id`

**Dependencies:** Task 1

**Files likely touched:**
- `src/models.rs`
- `content/modules/example.md`
- `public/assets/`

**Estimated scope:** Small: 1-2 files

## Task 3: Build-time content parser

**Description:** Add `build.rs` pipeline to scan `content/`, parse YAML front matter, render Markdown to HTML, validate records, and write a compact JSON search index.

**Acceptance criteria:**
- [ ] Build scans nested category folders under `content/`
- [ ] Front matter parse errors identify file and field
- [ ] Markdown body renders to clean HTML
- [ ] `public/search_index.json` is generated at build time
- [ ] Build re-runs when content changes

**Dependencies:** Task 2

**Files likely touched:**
- `build.rs`
- `Cargo.toml`
- `src/models.rs`
- `public/search_index.json`

**Estimated scope:** Medium: 3-5 files

## Task 4: Home dashboard with category filtering

**Description:** Implement `/` as a scannable dashboard that loads generated search data, tracks query/category state, and lists matching causes as cards.

**Acceptance criteria:**
- [ ] Home page loads generated JSON on init
- [ ] Category filter updates visible cards
- [ ] Cause cards show title, category, severity, impact factor, and tags
- [ ] Empty state appears when no cause matches

**Dependencies:** Task 3

**Files likely touched:**
- `src/pages/home.rs`
- `src/components/search_bar.rs`
- `src/components/filter_panel.rs`
- `src/components/cause_card.rs`
- `src/components/tag_pill.rs`

**Estimated scope:** Medium: 3-5 files

## Task 5: Cause detail route

**Description:** Implement `/cause/:id` as the engineering reference page with metadata checklist, rendered body HTML, and image gallery.

**Acceptance criteria:**
- [ ] Detail page resolves cause by slug
- [ ] Metadata renders as clear key-value checklist
- [ ] Rendered Markdown body displays below metadata
- [ ] Image gallery displays path, caption, and missing-image fallback
- [ ] Unknown slug returns a not-found view

**Dependencies:** Task 3

**Files likely touched:**
- `src/pages/detail.rs`
- `src/components/image_gallery.rs`
- `src/components/tag_pill.rs`
- `src/models.rs`

**Estimated scope:** Medium: 3-5 files

## Task 6: Client-side fuzzy search

**Description:** Add weighted fuzzy matching over title, tags, and category, exposed through reactive derived state so filtering only recomputes when query or filters change.

**Acceptance criteria:**
- [ ] Title matches rank above tag matches
- [ ] Tag matches rank above category matches
- [ ] Search is case-insensitive
- [ ] Query changes update dashboard without page reload
- [ ] Uncompressed search index stays under 1.0 MB
- [ ] If index exceeds 1.0 MB, Pagefind/chunked index decision is documented before proceeding
- [ ] Filtering logic is covered by unit tests

**Dependencies:** Task 4

**Files likely touched:**
- `src/pages/home.rs`
- `src/components/search_bar.rs`
- `src/models.rs`
- `Cargo.toml`
- `tests/search.rs`

**Estimated scope:** Medium: 3-5 files

## Task 7: Taxonomy tags page

**Description:** Implement `/tags` as an alphabetical tag directory with item counts and links back to home with the selected tag preloaded.

**Acceptance criteria:**
- [ ] Tags are deduplicated across all causes
- [ ] Tags sort alphabetically
- [ ] Each tag displays accurate count
- [ ] Clicking tag opens home with corresponding filter active

**Dependencies:** Task 3, Task 4

**Files likely touched:**
- `src/pages/tags.rs`
- `src/components/tag_pill.rs`
- `src/models.rs`

**Estimated scope:** Small: 1-2 files

## Task 8: About methodology page

**Description:** Implement `/about` as static methodology content covering field diagnostics and verification criteria.

**Acceptance criteria:**
- [ ] About page has static sections for diagnostic workflow
- [ ] IEC 62446 and field validation references are represented
- [ ] Page uses shared header/footer layout
- [ ] No dynamic data fetch required

**Dependencies:** Task 1

**Files likely touched:**
- `src/pages/about.rs`
- `src/components/header.rs`
- `src/components/footer.rs`

**Estimated scope:** Small: 1-2 files

## Task 9: Global BEM styling

**Description:** Add the single global stylesheet using component-aligned BEM classes, responsive layouts, readable technical typography, and no framework/scoped CSS.

**Acceptance criteria:**
- [ ] All component classes follow BEM naming
- [ ] No CSS framework is added
- [ ] Layout avoids global parent flex/grid per spec
- [ ] Home, detail, tags, and about pages are usable on mobile and desktop
- [ ] Text does not overflow cards, buttons, or filters

**Dependencies:** Task 4, Task 5, Task 7, Task 8

**Files likely touched:**
- `public/styles.css`
- `src/components/*.rs`
- `src/pages/*.rs`

**Estimated scope:** Medium: 3-5 files

## Task 10: Content validation and build checks

**Description:** Harden build-time validation so invalid content fails fast and contributors can verify the site with one command.

**Acceptance criteria:**
- [ ] Duplicate ids fail build
- [ ] Missing image captions fail build
- [ ] Invalid severity values fail build
- [ ] Search index contains every valid cause exactly once
- [ ] README documents build, check, and content authoring flow

**Dependencies:** Task 3

**Files likely touched:**
- `build.rs`
- `README.md`
- `content/**`
- `tests/content_validation.rs`

**Estimated scope:** Medium: 3-5 files

## Task 11: Performance and accessibility pass

**Description:** Verify static output, search payload size, keyboard navigation, semantic structure, and image alt/caption behavior before review.

**Acceptance criteria:**
- [ ] Static build output renders without runtime server assumptions
- [ ] Search index is measured uncompressed and stays under 1.0 MB
- [ ] Pagefind/chunked index fallback is documented if 1.0 MB cap is exceeded
- [ ] Keyboard users can reach search, filters, cards, and nav links
- [ ] Pages have useful titles and headings
- [ ] Images include accessible captions or alt text

**Dependencies:** Task 9, Task 10

**Files likely touched:**
- `src/pages/*.rs`
- `src/components/*.rs`
- `public/styles.css`
- `README.md`

**Estimated scope:** Medium: 3-5 files

## Risks and Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| Spec asks for static Markdown, user rule mentions DB CRUD files that do not exist | Med | Treat DB/API as out of scope unless spec changes |
| Leptos SSG setup needs extra tooling beyond current minimal `Cargo.toml` | Med | Standardize on `cargo-leptos` plus server-export run |
| Build-time generated Rust/static data can become awkward | Med | Keep models shared and parser output simple JSON/HTML |
| Search payload grows past 1.0 MB uncompressed | High | Generate compact records first; pivot to Pagefind/chunked index if cap is exceeded |
| Global CSS can collide | Low | Strict component-prefixed BEM class names |

## Open Questions
- None
