//! Shared Leptos components with self-contained styles.
//!
//! Components here carry no project content and need no build step in the
//! app. Their styles ship inside the crate and are emitted by [`UiStyles`];
//! colors and fonts come from CSS custom properties the app defines. See the
//! README for the full list.

mod checkbox;
mod hash_scroll;
mod hooks;
mod layout;
mod modal;
mod prose;
mod styles;
mod theme;
mod toast;

pub use checkbox::Checkbox;
pub use hash_scroll::HashScroll;
pub use hooks::use_now;
pub use layout::{Card, Eyebrow};
pub use modal::Modal;
pub use prose::{paragraphs, single_line, Prose};
pub use styles::UiStyles;
pub use theme::{ThemeScript, ThemeToggle};
pub use toast::{Toast, Toaster};
