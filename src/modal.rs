use leptos::prelude::*;

// Shared modal shell. Renders nothing while `open` is false; when true, shows a
// centered overlay that closes on click-outside, the "x" button, or Escape. The
// body is provided via children, so callers control the inner markup.
#[component]
pub fn Modal(open: RwSignal<bool>, children: ChildrenFn) -> impl IntoView {
    register_escape(open);

    view! {
        {move || open.get().then(|| view! {
            <div
                role="dialog"
                aria-modal="true"
                on:click=move |_| open.set(false)
                class="ui-modal"
            >
                <div
                    on:click=|ev: leptos::ev::MouseEvent| ev.stop_propagation()
                    class="ui-modal__panel"
                >
                    <button
                        type="button"
                        aria-label="Close details"
                        on:click=move |_| open.set(false)
                        class="ui-modal__close"
                    >
                        "x"
                    </button>
                    {children()}
                </div>
            </div>
        })}
    }
}

// Escape closes the modal. Registered for the component's lifetime and gated to
// the client build so it never touches `window()` during SSR.
#[cfg(feature = "hydrate")]
fn register_escape(open: RwSignal<bool>) {
    let handle = window_event_listener(leptos::ev::keydown, move |ev| {
        if ev.key() == "Escape" && open.get() {
            open.set(false);
        }
    });
    on_cleanup(move || handle.remove());
}

#[cfg(not(feature = "hydrate"))]
fn register_escape(_open: RwSignal<bool>) {}
