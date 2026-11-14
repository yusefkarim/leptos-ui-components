use leptos::prelude::*;
/// A handle for showing short messages at the bottom of the screen. Create
/// one per page, render its [`Toaster`], and call [`Toast::show`] from event
/// handlers. Only one message shows at a time; a new one replaces the last.
#[derive(Clone, Copy)]
pub struct Toast {
    // The id lets a newer message survive the timer of an older one.
    current: RwSignal<Option<(u32, String)>>,
}

impl Toast {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Toast {
            current: RwSignal::new(None),
        }
    }

    pub fn show(self, msg: impl Into<String>) {
        let current = self.current;
        let id = current.with_untracked(|c| c.as_ref().map_or(0, |c| c.0)) + 1;
        current.set(Some((id, msg.into())));
        hide_later(current, id);
    }

    /// The message on screen, if any.
    pub fn message(self) -> Option<String> {
        self.current.with(|c| c.as_ref().map(|c| c.1.clone()))
    }
}

#[cfg(feature = "hydrate")]
fn hide_later(current: RwSignal<Option<(u32, String)>>, id: u32) {
    // How long a toast stays up.
    const SHOW_FOR: std::time::Duration = std::time::Duration::from_millis(2800);
    set_timeout(
        move || {
            if current.with_untracked(|c| c.as_ref().map(|c| c.0)) == Some(id) {
                current.set(None);
            }
        },
        SHOW_FOR,
    );
}

// Toasts come from event handlers, which only run in the browser.
#[cfg(not(feature = "hydrate"))]
fn hide_later(_current: RwSignal<Option<(u32, String)>>, _id: u32) {}

/// Where a [`Toast`]'s messages appear: fixed at the bottom centre, and
/// announced to screen readers as they change.
#[component]
pub fn Toaster(toast: Toast) -> impl IntoView {
    view! {
        <div class="ui-toaster" aria-live="polite">
            {move || toast.message().map(|m| view! { <p class="ui-toast">{m}</p> })}
        </div>
    }
}

#[cfg(all(test, feature = "ssr"))]
mod tests {
    use super::*;

    #[test]
    fn a_new_message_replaces_the_last() {
        let toast = Toast::new();
        assert_eq!(toast.message(), None);
        toast.show("one");
        toast.show("two");
        assert_eq!(toast.message().as_deref(), Some("two"));
    }
}
