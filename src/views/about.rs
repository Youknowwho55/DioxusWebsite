use crate::Route;
use dioxus::prelude::*;
use log::{info, trace, warn};


#[component]
pub fn About() -> Element {
    rsx! {

        div { id: "About",

            // Content
            h1 { "This is theAbout Section!" }
            p { "Info about us." }
            form { onsubmit: move |event| { log::info!("Submitted! {event:?}") },
                input { name: "name" }
                input { name: "age" }
                input { name: "date" }
                input { r#type: "submit" }
            }
        }
    }
}
