// Prevent attempting to import scanner code
#[allow(unused_imports)]
mod app;
mod message;
mod style;
mod views;
mod widgets;

use iced::{Application, Settings as IcedSettings};
use std::error::Error;

// Re-export base types
pub use app::AstraApp;
pub use message::Message;

/// Run the GUI application - we'll run the standalone version
pub fn run() -> Result<(), Box<dyn Error>> {
    // Avoid referencing scanner code by keeping the implementation simple
    println!("Starting GUI mode - simplified version");
    
    // Create Iced settings
    let settings = IcedSettings {
        window: iced::window::Settings {
            size: (1280, 720),
            position: iced::window::Position::Centered,
            min_size: Some((1000, 600)),
            ..Default::default()
        },
        default_text_size: 20.0,
        // Use system font to avoid missing fonts
        default_font: iced::Font::DEFAULT,
        antialiasing: true,
        ..Default::default()
    };
    
    // Run the app
    app::AstraApp::run(settings).map_err(|e| e.into())
} 