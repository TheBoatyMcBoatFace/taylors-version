// src/config/settings.rs

use dotenv::dotenv;
use std::env;

// Struct to hold application settings
pub struct Settings {
    pub spotify_client_id: String,
    pub spotify_client_secret: String,
    pub firebase_credentials: String,
    pub redirect_uri: String, // URI where Spotify will redirect after authentication
                              // Add other configuration settings as needed
}

impl Settings {
    // Function to load settings from environment variables
    pub fn new() -> Self {
        // Load environment variables from .env file (if available)
        dotenv().ok();

        // Use environment variables to fetch settings
        let spotify_client_id =
            env::var("SPOTIFY_CLIENT_ID").expect("SPOTIFY_CLIENT_ID must be set");

        let spotify_client_secret =
            env::var("SPOTIFY_CLIENT_SECRET").expect("SPOTIFY_CLIENT_SECRET must be set");

        let firebase_credentials =
            env::var("FIREBASE_CREDENTIALS").expect("FIREBASE_CREDENTIALS must be set");

        let redirect_uri = env::var("REDIRECT_URI").expect("REDIRECT_URI must be set");

        Settings {
            spotify_client_id,
            spotify_client_secret,
            firebase_credentials,
            redirect_uri,
        }
    }
}
