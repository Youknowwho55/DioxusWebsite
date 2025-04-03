use crate::auth::Supabase;
use reqwest::Client;
use crate::env;

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