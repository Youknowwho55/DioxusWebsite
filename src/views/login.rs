use dioxus::prelude::*;

use crate::{
    components::layout::FlexStack,
    views::routes::GuardContext,
};

use crate::server::auth::Auth;

// use crate::client::auth::signin_with_google;
use crate::components::ui::button::Button;

#[component]
pub fn Login() -> Element {
    let google_login = move |_| {
        spawn(async move {
            // Ensure signin_with_google() is correctly defined
            // if it's an async function, await it inside spawn
            client! {
                // signin_with_google().await;
            }
        });
    };

    rsx! {
        div { class: "max-w-lg mx-auto px-2 h-dvh place-content-center",
            FlexStack { class: "border-2 p-8 rounded-md border-orange-800 bg-gray-900",
                h1 { class: "text-3xl", "SupaDioxus" }
                Auth {
                    on_success: move |_user| {
                        // If GuardContext::redirect_next_or_home() is async, use spawn
                        spawn(async move {
                            GuardContext::redirect_next_or_home();
                        });
                    },
                }
                div { class: "mx-auto place-content-center mt-3",
                    Button { text: "Sign In with Google", on_click: google_login }
                }
            }
        }
    }
}
