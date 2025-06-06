
use crate::components::{Hero, Steps};
use crate::views::routes::Routes;
use dioxus:: prelude::*;



#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
        Steps {}
        Link { to: Routes::Protected {}, "Protected" }
    }
}

