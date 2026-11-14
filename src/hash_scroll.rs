use leptos::prelude::*;
use leptos_router::hooks::use_location;

// `leptos_router` intercepts `<a>` clicks for client-side navigation, which
// updates `location.hash` but skips the browser's native scroll-to-anchor and
// may reset scroll position on navigation. This effect reacts to hash changes
// and scrolls to the target element, after the router settles.
/// Restores in-page anchor behavior for every `<a href="#...">` after
/// hydration. Render it once, inside the `<Router>`.
#[component]
pub fn HashScroll() -> impl IntoView {
    let location = use_location();
    Effect::new(move |_| {
        let hash = location.hash.get();
        let id = hash.trim_start_matches('#').to_string();
        if id.is_empty() {
            return;
        }
        request_animation_frame(move || {
            if let Some(el) = document().get_element_by_id(&id) {
                el.scroll_into_view();
            }
        });
    });
}
