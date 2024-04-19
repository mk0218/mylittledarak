use leptos::*;
use leptos::ev::MouseEvent;
use super::overlay::*;
use crate::{Theme, UIButton};

enum Icon {
    Menu,
    Theme,
    Filter,
}

impl Icon {
    fn filename<'a>(&'a self, theme: Theme) -> &'a str {
        match (self, theme) {
            (Icon::Menu, Theme::Light) => "menu_light.svg",
            (Icon::Menu, Theme::Dark) => "menu_dark.svg",
            (Icon::Theme, Theme::Light) => "light.svg",
            (Icon::Theme, Theme::Dark) => "dark.svg",
            (Icon::Filter, Theme::Light) => "filter_light.svg",
            (Icon::Filter, Theme::Dark) => "filter_dark.svg",
        }
    }
}

stylance::import_style!(styles, "nav.module.css");

#[component]
pub fn Nav(#[prop(into)] switch_theme: Callback<MouseEvent>) -> impl IntoView {
    let (showmenu, set_showmenu) = create_signal(false);
    let (showfilter, set_showfilter) = create_signal(false);

    let theme = use_context::<ReadSignal<Theme>>()
        .expect("The root App component must provide theme context.");

    let path = |filename: &str| format!("/static/assets/{}", filename);

    let icon_src = move |icon: Icon| {
        theme.with(|theme| path(&icon.filename(*theme)))
    };

    view! {
        <div class=styles::nav>
            <span>
                {move || view! {
                    <UIButton
                        icon_src=icon_src(Icon::Menu)
                        on_click=move |_| set_showmenu(true)
                    />
                }}
            </span>
            <span>
                {move || view! {
                    <UIButton
                        icon_src=icon_src(Icon::Theme)
                        on_click=switch_theme
                    />
                    <UIButton
                        icon_src=icon_src(Icon::Filter)
                        on_click=move |_| set_showfilter(true)
                    />
                }}
            </span>
            <Show when=showmenu>
                <Overlay on_close=move |_| set_showmenu(false)>
                    menu
                </Overlay>
            </Show>
            <Show when=showfilter>
                <Overlay on_close=move |_| set_showfilter(false)>
                    filter
                </Overlay>
            </Show>
        </div>
    }
}