use dioxus::prelude::{Router as DRouter, *};

use super::{home::Home, login::Login, protected::Protected, callback::Callback,
blog::Blog, products::Products, calculator::Calculator, about::About, team::Team};
use crate::components::navbar::NavLink;

use crate::{client::auth::get_user, components::Navbar};

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Routes {
    #[layout(Wrapper)]
    #[route("/")]
    Home {},
    #[route("/login")]
    Login {},
    #[route("/protected")]
    Protected {},
    #[route("/callback")]
    Callback {},
    #[route("/blog/:id")]
    Blog { id: i32 },
    #[route("/products/:id")]
    Products { id: i32 },
    #[route("/calculator/:id")]
    Calculator { id: i32 },
    #[route("/about")]
    About {},
    #[route("/team")]
    Team {},
}




#[component]
pub fn Wrapper() -> Element {
    rsx! {
        header { class: "absolute inset-x-0 top-0 z-50",
            Navbar {
                nav_items: vec![
                    rsx! {
                        NavLink { to : NavigationTarget::from(Routes::Home {}), text : "Home"
                        .to_string() }
                    },
                    rsx! {
                        NavLink { to : NavigationTarget::from(Routes::About {}), text : "About"
                        .to_string() }
                    },
                    rsx! {
                        NavLink { to : NavigationTarget::from(Routes::Team {}), text : "Team"
                        .to_string() }
                    },
                    rsx! {
                        NavLink { to : NavigationTarget::from(Routes::Blog { id : 1 }), text : "Blog"
                        .to_string() }
                    },
                    rsx! {
                        NavLink { to : NavigationTarget::from(Routes::Products { id : 1 }), text :
                        "Products".to_string() }
                    },
                    rsx! {
                        NavLink { to : NavigationTarget::from(Routes::Calculator { id : 1 }), text :
                        "Calculator".to_string() }
                    },
                    rsx! {
                        NavLink { to : NavigationTarget::from(Routes::Protected {}), text :
                        "Protected".to_string() }
                    },
                ],
            }
        }
        div { class: "relative isolate pt-16", Outlet::<Routes> {} }
    }
}

/// Register the protected state of routes here
fn is_guarded(current: Routes) -> bool {
    // guard routes
    match current {
        Routes::Home {} => false,
        Routes::Login {} => false,
        Routes::Blog {id: i32} => false,
        Routes::Products {id: i32} => false,
        Routes::Calculator {id: i32} => false,
        Routes::About {} => false,
        Routes::Team {} => false,
        Routes::Protected {} => true,
        Routes::Callback { .. } => false,
    }
}

#[component]
pub fn Router() -> Element {
    rsx! {
        DRouter::<Routes> {
            config: || {
                RouterConfig::default()
                    .on_update(|state| {
                        if is_guarded(state.current()) {
                            on_not_authorized(move |_| {
                                GuardContext::set_next(state.current());
                            });
                        }
                        None
                    })
            },
        }
    }
}

#[derive(Default)]
pub struct GuardContext {
    next: Option<Routes>,
}

impl GuardContext {
    pub fn set_next(next: Routes) {
        let mut guard = use_context::<Signal<GuardContext>>();
        guard.write().next = Some(next);
    }

    pub fn redirect_next_or_home() {
        let nav = navigator();
        let mut guard = use_context::<Signal<GuardContext>>();
        let next_maybe = guard.write().next.take();
        if let Some(next) = next_maybe {
            nav.push(next);
        } else {
            nav.push(Routes::Home {});
        }
    }
}


fn on_not_authorized<F>(f: F)
where
    F: Fn(()) + 'static,
{
    #[cfg(target_arch = "wasm32")]
    {
        spawn(async move {
            let user = get_user().await;
            if user.is_err() {
                f(());
            }
        });
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        // For non-WASM targets, either always consider the user unauthorized
        // or implement alternative behavior
        f(());
    }
}


/// Declare a page view protected
///
/// Automatically redirect users to login and back to the page on auth success

pub fn protected(redirect: Routes, next: Routes) {
    #[cfg(target_arch = "wasm32")]
    {
        spawn(async move {
            let user = get_user().await;
            if user.is_err() {
                GuardContext::set_next(next);
                let nav = navigator();
                nav.replace(redirect);
            }
        });
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Do nothing or implement alternative behavior for non-WASM targets
        // This ensures the function doesn't crash on non-WASM platforms
    }
}