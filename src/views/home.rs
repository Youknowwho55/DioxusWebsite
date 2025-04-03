
use crate::components::{Hero, Steps};


use dioxus::{document::StyleProps, prelude::*};



#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
        Steps {}
    }
}
