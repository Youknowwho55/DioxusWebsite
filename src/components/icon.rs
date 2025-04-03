use dioxus::prelude::*;


#[component]
pub fn Icon(name: String, class: String) -> Element {
    rsx! {
        span { class: ["material-icons", &class].join(" "), {name} }
    }
}