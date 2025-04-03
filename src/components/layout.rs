#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn FlexStack(class: Option<String>, children: Element) -> Element {
    let class = format!(
        "flex flex-col space-y-4 {}",
        class.unwrap_or_else(|| "".to_string())
    );
    rsx! {
        div { class, {children} }
    }
}

