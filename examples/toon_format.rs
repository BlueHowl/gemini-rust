//! Example demonstrating requesting TOON format output from Gemini
//!
//! This example shows how to use the `.using_toon()` method to ask the Gemini API
//! to generate responses in TOON (Token-Oriented Object Notation) format.
//!
//! TOON is a compact, human-readable format using indentation-based structure.
//!
//! To run this example with the toon_wip feature enabled:
//! ```bash
//! cargo run --example toon_format --features toon_wip
//! ```
//!
//! Make sure to set your API key:
//! ```bash
//! export GEMINI_API_KEY=your_api_key_here
//! ```

use gemini_rust::prelude::*;
use gemini_rust::Part;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    // Get the API key from environment variable
    let api_key = std::env::var("GEMINI_API_KEY")
        .expect("GEMINI_API_KEY environment variable must be set");

    // Create a new Gemini client
    let client = Gemini::new(&api_key).expect("unable to create Gemini API client");

    tracing::info!("starting content generation with toon format request");

    // Generate content asking for TOON format output using the .using_toon() method
    let response = client
        .generate_content()
        .using_toon()
        .with_user_message("Create a detailed user profile with name, age, email, active status, a roles array, and a settings object containing theme and notifications preferences.")
        .execute()
        .await?;

    // Display the response
    println!("\n=== Gemini Response (TOON Format) ===\n");
    if let Some(candidate) = response.candidates.first() {
        if let Some(parts) = &candidate.content.parts {
            for part in parts {
                if let Part::Text { text, .. } = part {
                    println!("{}\n", text);
                    
                    // Try to parse and validate the TOON format
                    #[cfg(feature = "toon_wip")]
                    {
                        use gemini_rust::common::format::toon;
                        
                        println!("\n=== Validating TOON Output ===\n");
                        
                        let toon_content = text.trim();
                        
                        // Try to parse it as a generic TOON value
                        match toon::from_str::<serde_json::Value>(toon_content) {
                            Ok(parsed) => {
                                println!("✓ Successfully parsed as TOON!");
                                println!("\nParsed structure:");
                                println!("{:#?}\n", parsed);
                                
                                // Demonstrate round-trip serialization
                                match toon::to_string(&parsed) {
                                    Ok(serialized) => {
                                        println!("\n=== Round-trip Serialization ===");
                                        println!("{}\n", serialized);
                                        println!("✓ TOON round-trip successful!");
                                    }
                                    Err(e) => {
                                        println!("✗ Failed to serialize back to TOON: {}", e);
                                    }
                                }
                            }
                            Err(e) => {
                                println!("✗ Failed to parse as TOON: {}\n", e);
                                println!("The model may not have followed TOON syntax exactly.");
                                println!("Raw output:\n{}", toon_content);
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

