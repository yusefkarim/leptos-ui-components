use leptos::prelude::*;

/// The small uppercase label above a heading.
#[component]
pub fn Eyebrow(children: Children) -> impl IntoView {
    view! {
        <p class="ui-eyebrow">{children()}</p>
    }
}

/// A bordered surface panel.
///
/// `class` adds the app's own utilities on top of the base. It is
/// `&'static str` because Tailwind only emits classes it finds as literals in
/// the app's source, so extra classes have to be written out at the call site.
#[component]
pub fn Card(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    view! {
        <div class=format!("ui-card {class}")>
            {children()}
        </div>
    }
}
