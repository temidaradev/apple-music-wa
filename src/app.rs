use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        cx,
        <iframe
            src="https://music.apple.com/"
            style="width: 100%; height: 100vh; border: none;"
        />
    }
}
