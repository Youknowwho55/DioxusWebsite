use reqwest::{Client, Error, Response};
use serde::{Deserialize, Serialize};

mod client;

pub mod auth;
use auth::Supabase;