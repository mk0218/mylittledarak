use leptos::*;

stylance::import_style!(#[allow(dead_code)] styles, "container.module.css");

#[component]
pub fn Container() -> impl IntoView {
    let contents = vec![
        "000001.JPG",
        "000007.JPG",
        "000008.JPG",
        "000009.JPG",
        "000010.JPG",
        "000012.JPG",
    ];

    let path = |filename: &str| {
        format!("/static/contents/pictures/ultramax400/{}", filename)
    };

    let pictures = contents.into_iter().map(|f| {
        view! {
            <img class=styles::picture src=path(f) />
        }
    }).collect_view();

    view! {
        <div class=styles::container>
            {pictures}
        </div>
    }
}