mod card;
mod progress;
mod button;

// Essential re-exports only to avoid conflicts
pub use button::{primary_text, secondary_text, destructive_text};

// Convenience function for progress bar
pub fn progress(value: f32) -> progress::ProgressBar {
    progress::ProgressBar::new(value)
} 