use dioxus::prelude::*;
use crate::components::ui::{Card, CardBody, CardTitle};

// Corrected variable name and asset handling
const HOME_SVG: &str = "./assets/home.svg"; // If asset! is correct, ensure it's properly imported

#[component]
pub fn Steps() -> Element {
   rsx!{
    div {
        h2 { "Everything should be this easy." }
    }

    div { class: " grid grid-cols-3 gap-4 w-full",
        Card {
            image_src: Some(HOME_SVG.to_string()), // Ensure Card supports `image_src`
            image_alt: Some("Card header image".to_string()),
            CardTitle { "Noteworthy technology acquisitions 2021" }
            CardBody {
                "Lorem ipsum dolor sit amet consectetur. Adipiscing imperdiet bibendum in in vestibulum. "
            }
        }
        Card {
            CardTitle { "Need to Remove <a>" }
            CardBody { "Content for the second card." }
        }
        Card {
            CardTitle { "Need to Remove <a>" }
            CardBody { "Content for the third card." }
        }
    }
}
}
