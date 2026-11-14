use leptos::prelude::*;

// Sets `data-theme` on <html> from localStorage, falling back to the OS
// preference, before first paint so there is no flash of the wrong theme.
const THEME_INIT_SCRIPT: &str = "(function(){try{var s=localStorage.getItem('theme');var t=(s==='light'||s==='dark')?s:(window.matchMedia('(prefers-color-scheme: light)').matches?'light':'dark');document.documentElement.setAttribute('data-theme',t)}catch(e){document.documentElement.setAttribute('data-theme','dark')}})();";

/// Inline pre-paint theme script. Render it in the shell's `<head>`, before
/// any stylesheet, so the theme is settled before the page is drawn.
#[component]
pub fn ThemeScript() -> impl IntoView {
    view! { <script inner_html=THEME_INIT_SCRIPT></script> }
}

/// Flips `data-theme` between light and dark and remembers the choice.
#[component]
pub fn ThemeToggle() -> impl IntoView {
    let on_click = move |_| {
        #[cfg(feature = "hydrate")]
        flip_theme();
    };

    view! {
        <button
            type="button"
            on:click=on_click
            aria-label="Toggle light and dark theme"
            title="Toggle theme"
            class="ui-theme-toggle"
        >
            // Sun: shown in dark mode (target = switch to light).
            <svg
                viewBox="0 0 24 24"
                class="ui-theme-toggle__sun"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                aria-hidden="true"
            >
                <circle cx="12" cy="12" r="4"/>
                <path d="M12 2v2M12 20v2M4.93 4.93l1.41 1.41M17.66 17.66l1.41 1.41M2 12h2M20 12h2M4.93 19.07l1.41-1.41M17.66 6.34l1.41-1.41"/>
            </svg>
            // Moon: shown in light mode (target = switch to dark).
            <svg
                viewBox="0 0 24 24"
                class="ui-theme-toggle__moon"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                aria-hidden="true"
            >
                <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>
            </svg>
        </button>
    }
}

#[cfg(feature = "hydrate")]
fn flip_theme() {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };
    let Some(html) = document.document_element() else {
        return;
    };
    let next = match html.get_attribute("data-theme").as_deref() {
        Some("light") => "dark",
        _ => "light",
    };
    let _ = html.set_attribute("data-theme", next);
    if let Ok(Some(storage)) = window.local_storage() {
        let _ = storage.set_item("theme", next);
    }
}
