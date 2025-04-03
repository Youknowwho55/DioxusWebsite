// use crate::Route;
// use crate::components::Button;
// use crate::components::ui::button::{ButtonSize, ButtonScheme, ButtonType};
// use dioxus::prelude::*;

// #[component]
// pub fn Navbar() -> Element {
//     rsx! {
//         div { class: "flex flex-row justify-end items-center",
//             Link {
//                 class: "text-black-600 mr-5 no-underline transition-colors duration-200 hover:text-blue-300",
//                 to: Route::Home {},
//                 "Home"
//             }
//             Link {
//                 class: "text-black-600 mr-5 no-underline transition-colors duration-200 hover:text-blue-300",
//                 to: Route::Blog { id: 1 },
//                 "Blog"
//             }
//             Link {
//                 class: "text-black-600 mr-5 no-underline transition-colors duration-200 hover:text-blue-300",
//                 to: Route::Products { id: 1 },
//                 "Products"
//             }
//             Link {
//                 class: "text-black-600 mr-5 no-underline transition-colors duration-200 hover:text-blue-300",
//                 to: Route::About {},
//                 "About"
//             }
//             Link {
//                 class: "text-black-600 mr-5 no-underline transition-colors duration-200 hover:text-blue-300",
//                 to: Route::Team {},
//                 "Team"
//             }
//             Link {
//                 class: "text-black-600 mr-5 no-underline transition-colors duration-200 hover:text-blue-300",
//                 to: Route::Calculator { id: 1 },
//                 "Calculator"
//             }

//             Button {
//                 button_scheme: ButtonScheme::Custom,
//                 button_size: ButtonSize::Default,
//                 button_type: ButtonType::Submit,
//                 class: "mx-5 mt-2 custom-class".to_string(),
//                 id: "my-button".to_string(),
//                 disabled: Some(false),
//                 children: rsx! { "Contact Us" },
//             }
//         }

//         Outlet::<Route> {}
//     }
// }
