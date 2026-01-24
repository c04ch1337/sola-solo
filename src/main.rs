//! Agentic Research Factory Demo
//!
//! This entry point demonstrates the usage of the ResearchSession module
//! for different research modes.

use std::env;
use std::error::Error;

mod agents;

use agents::researcher::ResearchSession;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logging
    env_logger::init();
    
    // Create a new research session
    let mut session = ResearchSession::new();
    
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    
    // Default query if none provided
    let query = if args.len() > 1 {
        args.get(1).unwrap().clone()
    } else {
        "artificial intelligence research papers".to_string()
    };
    
    // Default mode if none provided
    let mode = if args.len() > 2 {
        match args.get(2).unwrap().as_str() {
            "professional" => true,
            "personal" => false,
            _ => true, // default to professional
        }
    } else {
        true // default to professional mode
    };
    
    // Set the mode
    println!("Setting mode to: {}", if mode { "Professional" } else { "Personal" });
    session.set_mode(mode).await;
    
    // Perform research based on mode
    if mode {
        // Professional mode - academic research
        println!("Performing academic research for query: {}", query);
        let result = session.gather_academic_data(query).await?;
        
        println!("Research complete! Results:");
        println!("---------------------------");
        println!("Source: {}", result.source);
        println!("Layer: {}", result.layer);
        println!("Content: {} characters", result.content.len());
        println!("---------------------------");
        
        // Store in memory
        match session.inject_to_memory(result).await {
            Ok(_) => println!("Successfully stored in Layer 5 (Professional)"),
            Err(e) => println!("Failed to store: {}", e),
        }
    } else {
        // Personal mode - companion insights
        if !session.is_professional_mode().await {
            println!("Performing companion research for kink: {}", query);
            
            match session.gather_companion_insights(query).await {
                Ok(result) => {
                    println!("Research complete! Results:");
                    println!("---------------------------");
                    println!("Source: {}", result.source);
                    println!("Layer: {}", result.layer);
                    println!("Content: {} characters", result.content.len());
                    println!("---------------------------");
                    
                    // Store in memory
                    match session.inject_to_memory(result).await {
                        Ok(_) => println!("Successfully stored in Layer 7 (Personal)"),
                        Err(e) => println!("Failed to store: {}", e),
                    }
                },
                Err(e) => println!("Failed to gather companion insights: {}", e),
            }
        } else {
            println!("ERROR: Cannot perform companion research in professional mode");
        }
    }
    
    Ok(())
}