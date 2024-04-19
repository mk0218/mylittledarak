mod component;
mod layout;

use component::*;
use layout::*;

use leptos::*;
use stylance::classes;

#[derive(Clone, Copy)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn class<'a>(&'a self) -> &'a str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }

    pub fn switch(&self) -> Self {
        match self {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        }
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (theme, set_theme) = create_signal(Theme::Light);

    provide_context(theme);
    // let theme_class = move || theme.with(|theme| theme.class());


    let switch_theme = move |_| set_theme.update(|theme| {
        *theme = theme.switch()
    });

    
    view! {
        <div class=move || classes!("app", theme().class())>
            <Nav switch_theme=switch_theme />
            <Container />
        </div>
    }
}
