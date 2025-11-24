//! Example demonstrating bidirectional TOON format with Gemini
//!
//! This example shows how to:
//! 1. Send data to Gemini in TOON format using `.with_toon_message()`
//! 2. Request responses in TOON format using `.using_toon()`
//! 3. Parse the TOON response back into Rust structures
//!
//! TOON (Token-Oriented Object Notation) is a compact, human-readable format
//! using indentation-based structure similar to YAML.
//!
//! To run this example:
//! ```bash
//! cargo run --example toon_bidirectional --features toon_wip
//! ```
//!
//! Make sure to set your API key:
//! ```bash
//! export GEMINI_API_KEY=your_api_key_here
//! ```

use gemini_rust::prelude::*;
use gemini_rust::Part;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct UserProfile {
    name: String,
    age: u32,
    email: String,
    active: bool,
    roles: Vec<String>,
    settings: Settings,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Settings {
    theme: String,
    notifications: bool,
    language: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    // Get the API key from environment variable
    let api_key = std::env::var("GEMINI_API_KEY")
        .expect("GEMINI_API_KEY environment variable must be set");

    // Create a new Gemini client
    let client = Gemini::new(&api_key).expect("unable to create Gemini API client");

    println!("\n=== Bidirectional TOON Format Example ===\n");

    // Example 1: Sending TOON data to Gemini
    println!("--- Example 1: Sending TOON Data ---\n");
    
    let user_data = UserProfile {
        name: "Alice Johnson".to_string(),
        age: 28,
        email: "alice@example.com".to_string(),
        active: true,
        roles: vec!["developer".to_string(), "reviewer".to_string()],
        settings: Settings {
            theme: "dark".to_string(),
            notifications: true,
            language: "en".to_string(),
        },
    };

    println!("Sending user profile to Gemini:");
    #[cfg(feature = "toon_wip")]
    {
        use gemini_rust::common::format::toon;
        let user_toon = toon::to_string(&user_data)?;
        println!("{}\n", user_toon);
    }

    let response = client
        .generate_content()
        .with_toon_message(&user_data)?
        .with_user_message("Analyze this user profile and suggest 3 improvements. Keep your response brief.")
        .execute()
        .await?;

    if let Some(candidate) = response.candidates.first() {
        if let Some(parts) = &candidate.content.parts {
            for part in parts {
                if let Part::Text { text, .. } = part {
                    println!("Gemini's analysis:");
                    println!("{}\n", text);
                }
            }
        }
    }

    // Example 2: Requesting TOON format output
    println!("\n--- Example 2: Requesting TOON Output ---\n");

    let response = client
        .generate_content()
        .using_toon()
        .with_user_message("Create a new user profile for a senior engineer named Bob Smith, age 35, with email bob@example.com")
        .execute()
        .await?;

    if let Some(candidate) = response.candidates.first() {
        if let Some(parts) = &candidate.content.parts {
            for part in parts {
                if let Part::Text { text, .. } = part {
                    println!("Gemini's TOON response:");
                    println!("{}\n", text);

                    // Parse the TOON response
                    #[cfg(feature = "toon_wip")]
                    {
                        use gemini_rust::common::format::toon;
                        let toon_content = text.trim();
                        match toon::from_str::<UserProfile>(toon_content) {
                            Ok(parsed_profile) => {
                                println!("✓ Successfully parsed into UserProfile struct!");
                                println!("\nParsed data:");
                                println!("  Name: {}", parsed_profile.name);
                                println!("  Age: {}", parsed_profile.age);
                                println!("  Email: {}", parsed_profile.email);
                                println!("  Active: {}", parsed_profile.active);
                                println!("  Roles: {:?}", parsed_profile.roles);
                                println!("  Theme: {}", parsed_profile.settings.theme);
                                println!("  Notifications: {}", parsed_profile.settings.notifications);
                                println!("  Language: {}", parsed_profile.settings.language);
                            }
                            Err(e) => {
                                println!("Note: Could not parse as UserProfile struct: {}", e);
                                println!("Trying as generic JSON value...");
                                
                                match toon::from_str::<serde_json::Value>(toon_content) {
                                    Ok(parsed) => {
                                        println!("✓ Parsed as generic structure!");
                                        println!("{:#?}", parsed);
                                    }
                                    Err(e) => {
                                        println!("✗ Failed to parse TOON: {}", e);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("\n=== Example Complete ===\n");
    Ok(())
}
