use dioxus::prelude::*;


#[component]
pub fn Team() -> Element {
    rsx! {

        div { id: "team",

            // Content
            h1 { "This is the Team Section!" }
            p { "Info about us." }
        

        }
    }
}
