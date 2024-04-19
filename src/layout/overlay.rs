use leptos::*;
use leptos::ev::MouseEvent;

use crate::{Theme, UIButton};

stylance::import_style!(styles, "overlay.module.css");

#[component]
pub fn Overlay(
    children: Children,
    #[prop(into)]
    on_close: Callback<MouseEvent>,
) -> impl IntoView {
    let theme = use_context::<ReadSignal<Theme>>()
        .expect("The root App component must provide theme context.");

    let icon_src = move || theme.with(|&theme| {
        match theme {
            Theme::Light => "/static/assets/close_light.svg",
            Theme::Dark => "/static/assets/close_dark.svg",
        }
    });

    view! {
        <div class=styles::overlay>
            <div class=styles::top_area>
                {move || view! {
                    <UIButton icon_src=icon_src() on:click=on_close />
                }}
            </div>
            <div class=styles::content_area>
                {children()}
            </div>
        </div>
    }
}