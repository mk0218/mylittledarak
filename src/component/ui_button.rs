use leptos::*;
use leptos::ev::MouseEvent;
use stylance::classes;

stylance::import_style!(styles, "ui_button.module.css");

#[component]
pub fn UIButton(
    #[prop(into)]
    icon_src: String,
    #[prop(optional)]
    class: &'static str,
    #[prop(into, default = (|_| {}).into())]
    on_click: Callback<MouseEvent>,
) -> impl IntoView {
    let classes = classes!(class, styles::button);
    
    let handle_click = move |ev: MouseEvent| {
        ev.stop_propagation();
        on_click(ev);
    };

    view! {
        <button type="button" class=classes on:click=handle_click>
            <img src=icon_src />
        </button>
    }
}
