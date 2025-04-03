use std::future::Future;
use std::env;

// At the top of your file
#[cfg(target_arch = "wasm32")]
use serde_wasm_bindgen::to_value;

use serde_json::to_value;
use dioxus::{
    prelude::{
        server_fn::{
            client::{browser::BrowserClient, Client},
            request::browser::BrowserRequest,
            response::browser::BrowserResponse,
        },
        ServerFnError,
    },
    signals::{GlobalSignal, Readable, Signal},
};
use gloo::storage::Storage;
use serde_json::Value;
use supabase_js_rs::*;
use tracing::debug;
use wasm_bindgen::{closure::Closure, JsValue};

// Load environment variables once at the module level
lazy_static::lazy_static! {
    static ref SUPABASE_URL: String = env::var("APP_PUBLIC_SUPABASE_URL").unwrap_or_else(|_| "".to_string());
    static ref SUPABASE_ANON_KEY: String = env::var("APP_PUBLIC_SUPABASE_ANON_KEY").unwrap_or_else(|_| "".to_string());
    static ref APP_PUBLIC_ID: String = env::var("APP_PUBLIC_ID").unwrap_or_else(|_| "app".to_string());
}

pub static CLIENT: GlobalSignal<SupabaseClient> = Signal::global(create_client);

// Custom web client to attach supabase bearer information onto requests
pub struct AuthorizedClient;

impl<CustErr> Client<CustErr> for AuthorizedClient {
    type Request = BrowserRequest;
    type Response = BrowserResponse;

    fn send(
        req: Self::Request,
    ) -> impl Future<Output = Result<Self::Response, ServerFnError<CustErr>>> + Send {
        let storage_key = format!("sb-{}-auth-token", &*APP_PUBLIC_ID);
        let res = gloo::storage::LocalStorage::get::<Session>(storage_key);

        if let Ok(session) = res {
            let headers = req.headers();
            headers.append("Authorization", &format!("Bearer {}", session.access_token));
        }
        BrowserClient::send(req)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub email: String,
    pub is_anonymous: Option<bool>,
    pub role: Option<String>,
    pub aud: String,
    pub bearer_token: String,
pub refresh_token: String,}




#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Session {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
}

// get_user types
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum AuthError {
    Unauthorized(String),
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct GetUserData {
    user: Option<User>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct GetUserResponse {
    data: GetUserData,
    error: Option<AuthError>,
}





pub async fn get_user() -> Result<User, AuthError> {
    #[cfg(target_arch = "wasm32")]
    {
        let auth = CLIENT.read().auth();
        if let Ok(res) = auth.get_user(None).await {
            let res = serde_wasm_bindgen::from_value::<GetUserResponse>(res).unwrap();
            if let Some(error) = res.error {
                return Err(error);
            }

            return Ok(res.data.user.unwrap());
        }
        // Handle the case where auth.get_user() fails
        return Err(AuthError::Unauthorized("Failed to get user".to_string()));
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        // Non-WASM fallback
        // Return an AuthError that matches your error type
        return Err(AuthError::Unauthorized("Authentication not available in non-WASM context".to_string()));
    }
}

pub async fn on_state_change() {
    let auth = CLIENT.read().auth();
    auth.on_auth_state_change(&Closure::new(move |event: JsValue, session: JsValue| {}));
}

// signin_with_password types
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct AuthResponseData {
    user: Option<User>,
    session: Option<Session>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct AuthResponseError {
    // Todo
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct AuthResponsePassword {
    data: AuthResponseData,
    error: Option<AuthResponseError>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct UserWithSession {
    pub user: User,
    session: Session,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    pub redirect_to: String,
}

pub async fn signin_with_password(
    credentials: Credentials,
) -> Result<UserWithSession, AuthResponseError> {
    let auth = CLIENT.read().auth();
    if let Ok(res) = auth.sign_in_with_password(credentials).await {
        let res = serde_wasm_bindgen::from_value::<AuthResponsePassword>(res).unwrap();

        if let Some(error) = res.error {
            return Err(error);
        }

        return Ok(UserWithSession {
            user: res.data.user.unwrap(),
            session: res.data.session.unwrap(),
        });
    }
    panic!("todo error")
}

// pub async fn signin_with_google() {
//     let auth = CLIENT.read().auth();
//     if let Ok(res) = auth
//         .sign_in_with_oauth(SignInWithOAuthCredentials {
//             provider: "google".to_string(),
//             options: to_value(&Options {
//                 redirect_to: "http://localhost:8081/callback".to_string(),
//             })
//             .unwrap(),
//         })
//         .await
//     {
//         debug!("{res:?}");
//         return;
//     }
//     panic!("todo error");
// }

pub async fn set_session(access_token: String, refresh_token: String) {
    let auth = CLIENT.read().auth();
    if let Ok(res) = auth
        .set_session(CurrentSession {
            access_token,
            refresh_token,
        })
        .await
    {
        debug!("{res:?}");
        return;
    }
    panic!("todo error");
}

pub async fn signout() -> Result<(), AuthResponseError> {
    let auth = CLIENT.read().auth();
    if let Ok(res) = auth.sign_out().await {
        let res = serde_wasm_bindgen::from_value::<Value>(res).unwrap();
        debug!("{res:?}");
        return Ok(());
    }
    Err(AuthResponseError {})
}

fn create_client() -> SupabaseClient {

    supabase_js_rs::create_client(
        &SUPABASE_URL,
        &SUPABASE_ANON_KEY
    )
}
