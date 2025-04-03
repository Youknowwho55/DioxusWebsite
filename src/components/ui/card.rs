#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CardProps {
    class: Option<String>,
    children: Element,
    href: Option<String>,
    image_src: Option<String>,
    image_alt: Option<String>,
    image_class: Option<String>,
    drawer_trigger: Option<String>,
    modal_trigger: Option<String>,
}

#[component]
pub fn Card(props: CardProps) -> Element {
    let base_class = "block max-w-sm bg-white border border-gray-200 rounded-lg shadow-sm hover:bg-gray-100 dark:bg-gray-800 dark:border-gray-700 dark:hover:bg-gray-700";
    let class = format!("{} {}", base_class, props.class.clone().unwrap_or_default());
    let content_class = if props.image_src.is_some() { "p-6" } else { "p-6" };
    
    rsx!(
        a {
            class: "{class}",
            href: props.href.clone().unwrap_or("#".to_string()),
            "data-drawer-target": props.drawer_trigger,
            "data-modal-target": props.modal_trigger,
            // Optional image
            {
                if let Some(src) = &props.image_src {
                    let img_class = format!(
                        "rounded-t-lg w-full {}",
                        props.image_class.clone().unwrap_or_default(),
                    );
                    rsx! {
                        img {
                            class: "{img_class}",
                            src: "{src}",
                            alt: props.image_alt.clone().unwrap_or("Card image".to_string()),
                        }
                    }
                } else {
                    rsx! {}
                }
            }
            // Content container
            div { class: "{content_class}", {props.children} }
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct CardTitleProps {
    class: Option<String>,
    children: Element,
}

#[component]
pub fn CardTitle(props: CardTitleProps) -> Element {
    let base_class = "mb-2 text-l font-bold tracking-tight text-gray-900 dark:text-white";
    let class = format!("{} {}", base_class, props.class.clone().unwrap_or_default());
    
    rsx!(
        h5 { class: "{class}", {props.children} }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct CardBodyProps {
    class: Option<String>,
    children: Element,
}

#[component]
pub fn CardBody(props: CardBodyProps) -> Element {
    let base_class = "font-normal text-gray-700 dark:text-gray-400";
    let class = format!("{} {}", base_class, props.class.clone().unwrap_or_default());
    
    rsx!(
        p { class: "{class}", {props.children} }
    )
}

// Example usage:
/*
fn app() -> Element {
    rsx!(
        Card {
            href: "https://example.com",
            image_src: Some("https://example.com/image.jpg"),
            image_alt: Some("Card header image"),
            CardTitle {
                "Noteworthy technology acquisitions 2021"
            }
            CardBody {
                "Here are the biggest enterprise technology acquisitions of 2021 so far, in reverse chronological order."
            }
        }
    )
}
*/