use crate::Route;
use dioxus::prelude::*;


#[component]
pub fn About() -> Element {
    rsx! {

        div {
            id: "About",

            // Content
            h1 { "This is theAbout Section!" }
            p { "Info about us." }


        }
    }
}
