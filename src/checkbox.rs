use leptos::prelude::*;

/// A checkbox drawn from the app's tokens, matching both themes and taking
/// the accent of whatever it sits in. It pops when ticked.
///
/// Wrap it in a `<label>` with the text, which names it for screen readers
/// and makes the text clickable. `on_change` gets the new state.
#[component]
pub fn Checkbox(
    #[prop(into)] checked: Signal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
    on_change: Callback<bool>,
) -> impl IntoView {
    view! {
        // The attribute carries the state into server-rendered HTML; the
        // property keeps the live box in step after hydration.
        <input
            type="checkbox"
            class="ui-checkbox"
            checked=move || checked.get()
            prop:checked=move || checked.get()
            disabled=move || disabled.get()
            on:change=move |ev| on_change.run(event_target_checked(&ev))
        />
    }
}

#[cfg(all(test, feature = "ssr"))]
mod tests {
    use super::*;

    #[test]
    fn state_is_in_the_server_rendered_html() {
        let html = view! { <Checkbox checked=true disabled=true on_change=Callback::new(|_| ())/> }
            .to_html();
        assert!(html.contains("checked"), "{html}");
        assert!(html.contains("disabled"), "{html}");

        let html = view! { <Checkbox checked=false on_change=Callback::new(|_| ())/> }.to_html();
        assert!(!html.contains("checked"), "{html}");
        assert!(!html.contains("disabled"), "{html}");
    }
}
