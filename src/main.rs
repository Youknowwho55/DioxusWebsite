use dioxus::prelude::*;



// use components::Navbar;
use views::{Blog, Home, About, Calculator, Team, Products};
mod components;
mod views;



// #[cfg(feature = "server")]
mod server;
pub mod client;

use tracing::info;

use client::auth::AuthorizedClient;
use crate::views::routes::{GuardContext, Router as AppRouter};


#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    // #[layout(Navbar)]
    #[route("/")]
    Home {},
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

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");



fn main() {
    dioxus::logger::initialize_default();

    server_only!({
        dotenv::dotenv().ok();
        info!("loaded env variables");
    });
    dioxus::launch(App);
}


#[component]
fn App() -> Element {
    let _guard_context = use_context_provider(|| Signal::new(GuardContext::default()));

    // Build cool things 
    rsx! {
        head {
            script { src: "https://cdn.jsdelivr.net/npm/@supabase/supabase-js@2" }
        }
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        // WILL NEED TO SEE WHAT ROUTER TO USE
        AppRouter {}
    }
}



#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    info!("Server received: {}", data);
    Ok(())
}

#[server(name = GetServerData, client=AuthorizedClient)]
async fn get_server_data() -> Result<String, ServerFnError> {
    use server::auth::SupabaseClient;
    let client: SupabaseClient = extract().await?;

    let resp = client
        .table("created_tasks")
        .select("*")
        .execute()
        .await?;

    info!("{:#?}", resp.text().await);
    Ok("Hello from the server!".to_string())
}