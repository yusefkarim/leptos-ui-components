use leptos::prelude::*;

// Current wall-clock time in milliseconds since the Unix epoch. On the client
// we read the browser clock; during SSR we read the system clock. The interval
// that drives the countdown only ever runs on the client (effects don't run on
// the server), so the `js_sys` path is wasm-only.
#[cfg(feature = "hydrate")]
fn now_ms() -> f64 {
    js_sys::Date::now()
}

#[cfg(not(feature = "hydrate"))]
fn now_ms() -> f64 {
    time::OffsetDateTime::now_utc().unix_timestamp_nanos() as f64 / 1_000_000.0
}

// The interval can freeze for two reasons. Chrome (since v88) throttles
// background-tab timers to roughly once a minute after the tab has been hidden
// a few minutes, and energy-saver modes can stop them entirely; this surfaces
// as a stale countdown that snaps back once the tab is shown (visibilitychange).
// The back/forward cache is worse: the browser freezes the page with timers
// paused and the old time still painted, then restores it later. That restore
// fires `pageshow` (with `persisted = true`), and unlike visibilitychange it is
// reliable across browsers, so without it some users see a stuck timer until a
// hard refresh. Re-read the clock on both so the display resyncs immediately.
// Only meaningful on the client.
#[cfg(feature = "hydrate")]
fn refresh_when_visible(now: RwSignal<f64>) {
    use send_wrapper::SendWrapper;
    use wasm_bindgen::closure::Closure;
    use wasm_bindgen::JsCast;

    let doc = document();
    let win = window();

    let on_visible = Closure::<dyn FnMut()>::new(move || now.set(now_ms()));
    doc.add_event_listener_with_callback("visibilitychange", on_visible.as_ref().unchecked_ref())
        .expect("failed to register visibilitychange listener");

    let on_show = Closure::<dyn FnMut()>::new(move || now.set(now_ms()));
    win.add_event_listener_with_callback("pageshow", on_show.as_ref().unchecked_ref())
        .expect("failed to register pageshow listener");

    // The wasm `Closure` is not `Send + Sync`, but `on_cleanup` requires it.
    // We only ever touch it on the (single) browser thread, so wrapping it is
    // sound; this mirrors how leptos itself manages event listeners.
    let guard = SendWrapper::new((doc, win, on_visible, on_show));
    on_cleanup(move || {
        let (doc, win, on_visible, on_show) = guard.take();
        let _ = doc.remove_event_listener_with_callback(
            "visibilitychange",
            on_visible.as_ref().unchecked_ref(),
        );
        let _ =
            win.remove_event_listener_with_callback("pageshow", on_show.as_ref().unchecked_ref());
    });
}

#[cfg(not(feature = "hydrate"))]
fn refresh_when_visible(_now: RwSignal<f64>) {}

/// A signal holding the current time in milliseconds, ticking once a second on
/// the client and resyncing whenever the tab comes back. Call it once per view
/// that needs a clock and share the signal, so two countdowns on the same page
/// cost one interval and stay in step.
pub fn use_now() -> RwSignal<f64> {
    let now = RwSignal::new(now_ms());

    // Tick once a second on the client. The effect (and therefore the interval)
    // never runs during SSR, and we clear it when the component unmounts.
    Effect::new(move |_| {
        let handle =
            set_interval_with_handle(move || now.set(now_ms()), std::time::Duration::from_secs(1))
                .expect("failed to start countdown interval");
        on_cleanup(move || handle.clear());

        // The interval alone is not enough: browsers throttle it in the
        // background, so also resync whenever the tab is shown again.
        refresh_when_visible(now);
    });

    now
}
