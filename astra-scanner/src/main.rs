use std::env;
use std::error::Error;

pub mod asn;
pub mod config;
pub mod gui;
pub mod lang;
pub mod scanner;
pub mod ui;
pub mod channel {
    // Empty module to fix imports
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    
    // Check for command
    if args.len() > 1 {
        match args[1].as_str() {
            "gui" => run_gui()?,
            _ => {
                println!("Unknown command: {}", args[1]);
                println!("Available commands:");
                println!("  gui - Launch the graphical interface");
            }
        }
    } else {
        println!("Astra Scanner - A tool for discovering Astra servers");
        println!("Run with 'gui' to launch the graphical interface");
    }
    
    Ok(())
}

/// Starts the GUI interface
fn run_gui() -> Result<(), Box<dyn Error>> {
    // Just run the GUI directly
    gui::run()?;
    Ok(())
} 