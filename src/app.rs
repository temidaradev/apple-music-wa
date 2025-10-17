use leptos::prelude::*;
use leptos::web_sys;

#[component]
pub fn App() -> impl IntoView {
    Effect::new(move |_| {
        if let Some(window) = web_sys::window() {
            let _ = window.location().set_href("https://music.apple.com/");
        }
    });

    view! {
        <div style="display: flex; align-items: center; justify-content: center; height: 100vh; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;">
            <p>"Loading Apple Music..."</p>
        </div>
    }
}
