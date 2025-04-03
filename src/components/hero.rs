use dioxus::{prelude::*};
use crate::components::ui::{InlineForm, InputSize, InputType};

const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Hero() -> Element {
    let on_form_submit = move |evt: FormEvent| {
        // Prevent the default form submission behavior if needed
        evt.prevent_default();
        // Handle the form submission data
        log::info!("Form submitted with data: {:?}", evt.data);
    };

    rsx! {
        div { class: "w-full bg-gradient-to-r from-secondary to-accent",

            div { id: "hero",
                img { src: HEADER_SVG, id: "header" }
            }
            div { class: "py-4 justify-beginning",
                h2 { class: "mb-1 px-2 text-xl font-extrabold leading-none tracking-tight text-gray-900 md:text-xl lg:text-2xl dark:text-white",
                    "Your Home"
                }
                h2 { class: "mb-1 px-2 text-xl font-extrabold leading-none tracking-tight text-gray-900 md:text-xl lg:text-2xl dark:text-white",
                    "Your Future"
                }
                h2 { class: "mb-1 px-2 text-xl font-extrabold leading-none tracking-tight text-gray-900 md:text-xl lg:text-2xl dark:text-white",
                    "Expertly Guided"
                }
            }
            div {
                p { class: "mb-6 px-4 text-lg font-normal text-gray-500 lg:text-xl dark:text-gray-400",
                    "Your Key to the Right Mortgage"
                }
            }
            div {
                InlineForm {
                    action: "/api/subscribe".to_string(),
                    input_name: "email".to_string(),
                    button_text: "Subscribe".to_string(),
                                // on_submit: Some(on_form_submit),
                }
            }
        }
    }}