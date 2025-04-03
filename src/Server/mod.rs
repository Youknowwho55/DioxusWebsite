pub mod auth;
pub mod database;


use reqwest::{Client, Error, Response};
use serde::{Deserialize, Serialize};


pub use auth::Supabase;
pub use auth::Auth;
