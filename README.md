# Yusef's Leptos UI Components

Shared Leptos 0.8 components with self-contained styles. The crate needs no build step in the
app and doesn't depend on the app's Tailwind setup: its CSS is compiled into the crate and
emitted by `<UiStyles/>`.

| Item          | What it is                                                                                                                            |
| ------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| `UiStyles`    | The stylesheet for everything below. Render once in `<head>`.                                                                         |
| `ThemeScript` | Pre-paint script that sets `data-theme` on `<html>` from localStorage or the OS preference. Render in `<head>` before any stylesheet. |
| `ThemeToggle` | Button that flips `data-theme` between light and dark and remembers the choice.                                                       |
| `Modal`       | Overlay driven by an `RwSignal<bool>`; closes on click-outside, the x button, or Escape.                                              |
| `Card`        | Bordered surface panel. `class` adds the app's own utilities.                                                                         |
| `Eyebrow`     | Small uppercase label above a heading.                                                                                                |
| `Prose`       | One `<p>` per blank-line-separated paragraph. `paragraphs` and `single_line` are the underlying helpers.                              |
| `HashScroll`  | Restores scroll-to-anchor for `#` links under `leptos_router`. Render once inside `<Router>`.                                         |
| `use_now`     | A signal holding the current time in ms, ticking each second and resyncing when the tab is shown again.                               |

## Using it

```toml
[dependencies]
leptos-ui-components = { git = "https://github.com/yusefkarim/leptos-ui-components.git", tag = "v0.1.0" }

[features]
hydrate = ["leptos/hydrate", "ui/hydrate", ...]
ssr = ["leptos/ssr", "ui/ssr", ...]
```

The app pins `wasm-bindgen`; this crate accepts any `0.2`.

In the shell:

```rust
<head>
    <ThemeScript/>
    // ... stylesheet, MetaTags, etc.
    <UiStyles/>
</head>
```

## What the app must provide

The components read these CSS custom properties and define none of them:

| Property          | Used for                         |
| ----------------- | -------------------------------- |
| `--color-paper`   | page background (modal backdrop) |
| `--color-surface` | card and modal panel background  |
| `--color-line`    | borders                          |
| `--color-mute`    | secondary text, icon color       |
| `--color-accent`  | eyebrow text, hover states       |
| `--font-mono`     | eyebrow and close button         |
| `--shadow-soft`   | modal panel shadow               |

Define them on `:root` with the dark values, and put the light values under
`:root[data-theme="light"]`. `ThemeScript` always sets `data-theme`, and the toggle's icons
switch on `html[data-theme="dark"]`.

With Tailwind v4, declare them in `@theme static`, not plain `@theme`. Plain `@theme` only
emits a variable when one of the app's own utilities uses it, so a token used only by this
crate (such as `--shadow-soft`) would silently go missing.

```css
@import "tailwindcss";

@theme static {
  --color-paper: #11141a;
  /* ... */
}

:root[data-theme="light"] {
  --color-paper: #f7f5f0;
  /* ... */
}

/* Recommended, so Tailwind's dark: variant follows the toggle rather than the OS. */
@custom-variant dark (&:where(html[data-theme="dark"] *));
```

## Styling and layers

`ui.css` places its rules in Tailwind's `components` layer and declares the
`theme, base, components, utilities` layer order itself. That keeps it above preflight
wherever `<UiStyles/>` appears, and lets any utility the app passes (`<Card class="p-4">`)
override the component's base style.

## Developing

```bash
cargo clippy --features ssr --all-targets -- -D warnings
cargo clippy --features hydrate --target wasm32-unknown-unknown -- -D warnings
cargo test --features ssr
```
