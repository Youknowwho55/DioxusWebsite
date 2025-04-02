use crate::Route;
use dioxus::prelude::*;


#[component]
pub fn Calculator(id: i32) -> Element {
    rsx! {

        div {
            id: "calculator",

            // Content
            h1 { "This is calculator section #{id}!" }
            p { "In calc #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components." }

            // Navigation links
            Link {
                to: Route::Blog { id: id - 1 },
                "Previous"
            }
            span { " <---> " }
            Link {
                to: Route::Blog { id: id + 1 },
                "Next"
            }
        }
    }
}
