use leptos::prelude::*;

const CSS: &str = include_str!("../style/ui.css");

/// The stylesheet for every component in this crate, inlined as a `<style>`.
/// Render it once in the app shell's `<head>`.
#[component]
pub fn UiStyles() -> impl IntoView {
    // `inner_html` so the CSS goes out verbatim rather than HTML-escaped.
    view! { <style inner_html=CSS></style> }
}

#[cfg(all(test, feature = "ssr"))]
mod tests {
    use super::*;

    #[test]
    fn css_is_emitted_unescaped() {
        let html = view! { <UiStyles/> }.to_html();
        assert!(html.starts_with("<style>"), "{html}");
        assert!(html.contains("<UiStyles/>"), "comment text was escaped");
        assert!(!html.contains("&lt;"));
    }
}
