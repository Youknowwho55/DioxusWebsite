use dioxus::prelude::*;

#[component]
pub fn Navbar(nav_items: Vec<Element>) -> Element {
    rsx! {
        div { class: "flex flex-row justify-between items-center p-4 bg-gray-800",
            // Add site logo/title on the left
            div { class: "text-white text-xl font-bold", "My Dioxus App" }
            // Navigation links in the middle
            div { class: "flex space-x-4 text-black-600 mr-5 no-underline transition-colors duration-200 hover:text-blue-300",
                for nav in nav_items {
                    {nav}
                }
            }
        }
    }
}

#[component]
pub fn NavLink(to: NavigationTarget, text: String) -> Element {
    rsx! {
        Link {
            active_class: "text-white font-bold underline decoration-white",
            class: "text-gray-300 hover:bg-gray-700 hover:text-white rounded-md px-3 py-2 text-sm font-medium",
            to,
            {text}
        }
    }
}