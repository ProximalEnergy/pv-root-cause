# pv-root-cause

Static Leptos compendium for photovoltaic underperformance root causes.

## Useful Commands

- Build and validate content: `cargo test`
- Build the static site assets: `cargo leptos build`
- Serve locally: `mise deploy`

## Content Authoring

Root-cause entries live under `content/**/*.md`. Each Markdown file must start with YAML front matter:

```yaml
---
id: module-hotspot
title: Module Hotspot from Cell Damage
category: Modules
tags:
  - thermal anomaly
severity: High
impact_factor: Localized heating can reduce string output.
detection_method: Drone infrared inspection.
mitigation: Replace affected module after electrical confirmation.
images:
  - path: /assets/modules/hotspot-ir.svg
    caption: Representative infrared frame showing a localized hotspot.
    alt: Simplified infrared module diagram with one hot cell.
---
```

Allowed severity values are `Low`, `Medium`, and `High`. Each entry must have a unique `id` and at least one tag. Image references are optional; when included, every image must include a non-empty `caption`. Image `alt` text is optional but recommended when the caption is not descriptive enough for non-visual use.

After the front matter, write the engineering reference in Markdown. Raw HTML is stripped during rendering.

## Build Checks

The build script parses all Markdown content, validates metadata, renders Markdown to HTML, and writes:

- `public/cause_content.json`
- `public/search_index.json`

Invalid content fails fast during `cargo test`, `cargo check`, or `cargo leptos build`. The validation suite covers duplicate IDs, missing image captions, invalid severity values, and verifies that every valid cause appears in the generated search index exactly once.

## Why Rust

Static pages and server-side HTML generation keep the compendium fast and simple to deploy.
