use leptos::*;

stylance::import_style!(#[allow(dead_code)] styles, "layout.module.css");

#[component]
pub fn Categories() -> impl IntoView {
    view! {
        <div class=styles::column />
    }
}