use std::io::{self, Write};
use colored::*;
use anyhow::{Result, Context};
use std::process;

use crate::lang::LanguageManager;
use crate::config::Config;
use crate::asn::ASNScanner;
use crate::scanner::AstraScanner;

/// Displays the main menu and handles user input
pub async fn main_menu(lang: &LanguageManager, config: &Config) -> Result<()> {
    loop {
        println!("\n{}", lang.get("menu.title").cyan().bold());
        println!("\n{}", lang.get("menu.main_menu"));
        println!("{}", lang.get("menu.scan_asn"));
        println!("{}", lang.get("menu.scan_astra"));
        println!("{}", lang.get("menu.change_lang"));
        println!("4. Interfaz gráfica de terminal");
        println!("5. {}", lang.get("menu.exit"));
        
        let input = prompt_input(&lang.get("menu.select_option"))?;
        
        match input.as_str() {
            "1" => {
                let country = prompt_input(&lang.get("asn.country_prompt"))?;
                if !country.is_empty() {
                    let asn_scanner = ASNScanner::new(lang, config);
                    asn_scanner.process_country(&country).await?;
                }
            },
            "2" => {
                let mut astra_scanner = AstraScanner::new(lang, config);
                astra_scanner.scan().await?;
            },
            "3" => {
                let new_lang = language_menu(lang).await?;
                if !new_lang.is_empty() {
                    println!("{}", "Language changed. Restart application to apply.".green());
                    break;
                }
            },
            "4" => {
                // Iniciar la interfaz TUI
                crate::ui::run_tui(lang, config).await?;
            },
            "5" | "exit" | "q" => {
                println!("Adiós!");
                process::exit(0);
            },
            _ => println!("{}", "Invalid option, please try again.".red()),
        }
    }
    
    Ok(())
}

/// Displays the language selection menu
pub async fn language_menu(_lang: &LanguageManager) -> Result<String> {
    // Using _lang with an underscore to indicate it's intentionally unused
    println!("\n{}", "Available languages:".cyan().bold());
    
    let languages = LanguageManager::get_available_languages()?;
    
    if languages.is_empty() {
        println!("{}", "No language files found.".yellow());
        return Ok(String::new());
    }
    
    for (i, lang_code) in languages.iter().enumerate() {
        println!("{}. {}", i + 1, lang_code);
    }
    
    let choice = prompt_input("\nSelect language (number): ")?;
    
    if choice.is_empty() {
        return Ok(String::new());
    }
    
    if let Ok(num) = choice.parse::<usize>() {
        if num > 0 && num <= languages.len() {
            return Ok(languages[num - 1].clone());
        }
    }
    
    println!("{}", "Invalid selection.".red());
    Ok(String::new())
}

/// Helper function to prompt for user input
pub fn prompt_input(prompt: &str) -> Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .with_context(|| "Failed to read user input")?;
        
    Ok(input.trim().to_string())
} 