#![allow(non_snake_case)]

use dioxus::prelude::*;
use serde_json::Value;
use reqwest::Client;

use crate::{
    client::auth::User,
    components::ui::{
        button::Button,
        input::{PasswordInput, TextInput},
    },
};

use serde::{Deserialize, Serialize};

use http::HeaderMap;

use crate::env;

#[component]
pub fn Auth(on_success: EventHandler<User>) -> Element {
    let mut email = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());
    let context = use_context::<crate::AppContext>();

    let mut auth_output = use_signal(|| match context.supabase_client.read().clone().user.clone() {
        Some(user) => format!("Logged in with {}", user.email),
        None => "Logged out".to_string(),
    });

    let sign_in = move |_| {
        let on_success = on_success.clone();
        spawn(async move {
            let supabase = context.supabase_client.read().clone();
            match supabase.sign_in_password(&email.read(), &password.read()).await {
                Ok((user, _)) => {
                    // Update client with user info
                    let mut client_clone = context.supabase_client.read().clone();
                    client_clone.user = Some(user.clone());
                    context.supabase_client.set(client_clone);

                    // Store user in local storage
                    context
                        .store
                        .write()
                        .set("user", &user)
                        .expect("Failed to store user");

                    auth_output.set(format!("Logged in with {}", user.email));
                    on_success.call(user);
                }
                Err(err) => {
                    auth_output.set(format!("Login failed: {}", err));
                }
            }
        });
    };

    let sign_up = move |_| {
        spawn(async move {
            let supabase = context.supabase_client.read().clone();
            match supabase.signup_email_password(&email.read(), &password.read()).await {
                Ok(email) => {
                    auth_output.set(format!(
                        "Signed up {}, confirm this email and then sign in!",
                        email
                    ));
                }
                Err(err) => {
                    auth_output.set(format!("Error signing up: {}", err));
                }
            }
        });
    };

    let sign_out = move |_| {
        spawn(async move {
            let supabase = context.supabase_client.read().clone();
            match supabase.logout().await {
                Ok(_) => {
                    // Update client to remove user
                    let mut client_clone = context.supabase_client.read().clone();
                    client_clone.user = None;
                    context.supabase_client.set(client_clone);

                    // Clear storage
                    context
                        .store
                        .write()
                        .clear()
                        .expect("Failed to clear storage");

                    auth_output.set("Logged out".to_string());
                }
                Err(err) => {
                    auth_output.set(format!("An error occurred while logging out: {}", err));
                }
            }
        });
    };

    rsx! {
        label {
            "Email"
            TextInput {
                i_value: email,
                on_input: move |event: FormEvent| {
                    email.set(event.value());
                },
            }
        }
        label {
            "Password"
            PasswordInput {
                i_value: password,
                on_input: move |event: FormEvent| {
                    password.set(event.value());
                },
            }
        }
        div { class: "flex gap-2",
            Button { text: "Login", on_click: sign_in }
            Button { text: "Sign Up", on_click: sign_up }
            Button { text: "Logout", on_click: sign_out }
        }
        p { "{auth_output}" }
    }
}



#[derive(Clone, Debug)]
pub struct Supabase {
    client: Client,
    url: String,
    api_key: String,
    pub user: Option<User>,
}

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct RefreshToken {
    refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub exp: usize,
}


impl Supabase {
    // Helper function to create an auth request with common headers
    fn create_auth_request(&self, endpoint: &str) -> RequestBuilder {
        let request_url = format!("{}/auth/v1/{}", self.url, endpoint);
        self.client
            .post(&request_url)
            .header("apikey", &self.api_key)
            .header("Content-Type", "application/json")
    }

    pub async fn sign_in_password(&self, email: &str, password: &str) -> Result<(User, Value), String> {
        let response = self
            .create_auth_request("token?grant_type=password")
            .json(&Credentials {
                email: email.to_string(),
                password: password.to_string(),
            })
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;
        
        let json_response: Value = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        // Extract user info from response
        let access_token = json_response["access_token"]
            .as_str()
            .ok_or_else(|| "Invalid credentials".to_string())?;
            
        let refresh_token = json_response["refresh_token"]
            .as_str()
            .ok_or_else(|| "Missing refresh token".to_string())?;
            
        let email = json_response["user"]["email"]
            .as_str()
            .ok_or_else(|| "Missing email".to_string())?;

        let user = User {
            bearer_token: access_token.to_string(),
            refresh_token: refresh_token.to_string(),
            email: email.to_string(),
        };

        Ok((user, json_response))
    }

    pub async fn refresh_session(&self) -> Result<(User, Value), String> {
        let refresh_token = self.user
            .as_ref()
            .map(|u| u.refresh_token.clone())
            .ok_or_else(|| "No user session to refresh".to_string())?;
            
        let response = self
            .create_auth_request("token?grant_type=refresh_token")
            .json(&RefreshToken { refresh_token })
            .send()
            .await
            .map_err(|e| format!("Refresh request failed: {}", e))?;
            
        let json_response: Value = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
            
        // Extract user info from response
        let access_token = json_response["access_token"]
            .as_str()
            .ok_or_else(|| "Invalid refresh token".to_string())?;
            
        let refresh_token = json_response["refresh_token"]
            .as_str()
            .ok_or_else(|| "Missing refresh token".to_string())?;
            
        let email = json_response["user"]["email"]
            .as_str()
            .ok_or_else(|| "Missing email".to_string())?;

        let user = User {
            bearer_token: access_token.to_string(),
            refresh_token: refresh_token.to_string(),
            email: email.to_string(),
        };

        Ok((user, json_response))
    }

    pub async fn logout(&self) -> Result<(), String> {
        let token = self.user
            .as_ref()
            .map(|u| u.bearer_token.clone())
            .ok_or_else(|| "Not logged in".to_string())?;
            
        let response = self
            .create_auth_request("logout")
            .bearer_auth(token)
            .send()
            .await
            .map_err(|e| format!("Logout request failed: {}", e))?;
            
        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("Logout failed: {}", error_text));
        }
        
        Ok(())
    }

    pub async fn signup_email_password(&self, email: &str, password: &str) -> Result<String, String> {
        let response = self
            .create_auth_request("signup")
            .json(&Credentials {
                email: email.to_string(),
                password: password.to_string(),
            })
            .send()
            .await
            .map_err(|e| format!("Signup request failed: {}", e))?;
            
        let json_response: Value = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
            
        let email = json_response["email"]
            .as_str()
            .ok_or_else(|| {
                format!(
                    "Error signing up: {}",
                    json_response["msg"].as_str().unwrap_or("Unknown error")
                )
            })?;
            
        Ok(email.to_string())
    }
}


;

pub struct SupabaseClient {
    client: postgrest::Postgrest,
    token: String,
}

impl SupabaseClient {
    pub fn table(&self, table: &str) -> postgrest::Builder {
        self.client.from(table).auth(self.token.clone())
    }
}

#[derive(Debug)]
pub struct NotFoundError;

impl std::fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl std::error::Error for NotFoundError {}

impl FromServerContext for SupabaseClient {
    type Rejection = NotFoundError;

    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn from_request<'life0, 'async_trait>(
        req: &'life0 dioxus::prelude::DioxusServerContext,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<Self, Self::Rejection>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            let headers = HeaderMap::from_request(req)
                .await
                .map_err(|_| NotFoundError)?;

            let bearer = headers
                .get(&format!("Authorization"))
                .ok_or(NotFoundError)?;

            // TODO this could fail in two ways
            let bearer = bearer.to_str().unwrap().split(" ").collect::<Vec<_>>()[1];

            let url = format!("{}/rest/v1", env::APP_PUBLIC_SUPABASE_URL);
            let client = postgrest::Postgrest::new(url)
                .insert_header("apikey", env::APP_PUBLIC_SUPABASE_ANON_KEY);

            Ok(SupabaseClient {
                client,
                token: bearer.into(),
            })
        })
    }
}

impl Supabase {
    pub fn new() -> Self {
        let client = Client::new();
        let url = env::APP_PUBLIC_SUPABASE_URL;
        let api_key = env::APP_PUBLIC_SUPABASE_ANON_KEY;

        
        Supabase {
            client,
            url,
            api_key,
            user: None,
        }
    }
}