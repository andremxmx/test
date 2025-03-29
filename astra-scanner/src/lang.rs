use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use serde_json::{Value, Map};
use anyhow::{Result, Context};

#[allow(dead_code)]
pub struct LanguageManager {
    current_lang: String,
    strings: HashMap<String, Value>,
}

impl LanguageManager {
    pub fn new(lang_code: &str) -> Self {
        let strings = Self::load_strings(lang_code).unwrap_or_default();
        Self {
            current_lang: lang_code.to_string(),
            strings,
        }
    }
    
    fn load_strings(lang_code: &str) -> Result<HashMap<String, Value>> {
        let lang_path = PathBuf::from(format!("lang/{}.json", lang_code));
        
        // If language file doesn't exist, try to create directory and provide default
        if !lang_path.exists() {
            fs::create_dir_all("lang")?;
            
            // If it's English, create a default language file
            if lang_code == "en" {
                Self::create_default_english_file()?;
            }
        }
        
        let mut file = File::open(&lang_path)
            .with_context(|| format!("Failed to open language file: {:?}", lang_path))?;
            
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .with_context(|| format!("Failed to read language file: {:?}", lang_path))?;
            
        let json: Value = serde_json::from_str(&contents)
            .with_context(|| format!("Failed to parse language file: {:?}", lang_path))?;
            
        if let Value::Object(map) = json {
            Ok(map.into_iter().collect())
        } else {
            Err(anyhow::anyhow!("Language file is not a JSON object"))
        }
    }
    
    fn create_default_english_file() -> Result<()> {
        let path = Path::new("lang/en.json");
        let mut file = File::create(path)
            .with_context(|| "Failed to create default English language file")?;
            
        let mut root = Map::new();
        
        // Menu strings
        let mut menu = Map::new();
        menu.insert("title".to_string(), Value::String("Astra Scanner".to_string()));
        menu.insert("main_menu".to_string(), Value::String("Main Menu:".to_string()));
        menu.insert("scan_asn".to_string(), Value::String("1. Scan ASN ranges by country".to_string()));
        menu.insert("scan_astra".to_string(), Value::String("2. Scan for Astra servers".to_string()));
        menu.insert("change_lang".to_string(), Value::String("3. Change language".to_string()));
        menu.insert("exit".to_string(), Value::String("4. Exit".to_string()));
        menu.insert("select_option".to_string(), Value::String("Select an option: ".to_string()));
        root.insert("menu".to_string(), Value::Object(menu));
        
        // ASN scanner strings
        let mut asn = Map::new();
        asn.insert("start_msg".to_string(), Value::String("Starting ASN scanning...".to_string()));
        asn.insert("country_prompt".to_string(), Value::String("Enter country code (e.g., US, ES): ".to_string()));
        asn.insert("no_asn_found".to_string(), Value::String("No ASNs found for the specified country.".to_string()));
        asn.insert("no_ranges_found".to_string(), Value::String("No IP ranges found.".to_string()));
        root.insert("asn".to_string(), Value::Object(asn));
        
        // Astra scanner strings
        let mut astra = Map::new();
        let mut scan = Map::new();
        scan.insert("starting".to_string(), Value::String("Starting scan with {} IPs and {} ports...".to_string()));
        astra.insert("scan".to_string(), Value::Object(scan));
        root.insert("astra".to_string(), Value::Object(astra));
        
        // Error messages
        let mut errors = Map::new();
        errors.insert("no_ip_file".to_string(), Value::String("IP file not found. Please create pool/ip.txt".to_string()));
        errors.insert("no_ports_file".to_string(), Value::String("Ports file not found. Please create pool/ports.txt".to_string()));
        errors.insert("port_convert".to_string(), Value::String("Error converting port: {}".to_string()));
        errors.insert("no_files_found".to_string(), Value::String("Required files not found.".to_string()));
        errors.insert("interrupted".to_string(), Value::String("Operation interrupted.".to_string()));
        errors.insert("partial_results".to_string(), Value::String("Partial results may have been saved.".to_string()));
        root.insert("errors".to_string(), Value::Object(errors));
        
        let json = serde_json::to_string_pretty(&Value::Object(root))
            .with_context(|| "Failed to serialize language data")?;
            
        use std::io::Write;
        file.write_all(json.as_bytes())
            .with_context(|| "Failed to write language file")?;
            
        Ok(())
    }
    
    pub fn get(&self, key: &str) -> String {
        let key_parts: Vec<&str> = key.split('.').collect();
        
        let mut current = self.strings.get(key_parts[0]);
        
        for &part in key_parts.iter().skip(1) {
            match current {
                Some(Value::Object(map)) => {
                    current = map.get(part);
                },
                _ => return key.to_string(),
            }
        }
        
        match current {
            Some(Value::String(s)) => s.clone(),
            _ => key.to_string(),
        }
    }
    
    pub fn get_available_languages() -> Result<Vec<String>> {
        let lang_dir = Path::new("lang");
        
        if !lang_dir.exists() {
            fs::create_dir_all(lang_dir)?;
            // Create default English file if it doesn't exist
            Self::create_default_english_file()?;
        }
        
        let mut languages = Vec::new();
        
        for entry in fs::read_dir(lang_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                if let Some(file_stem) = path.file_stem() {
                    if let Some(lang_code) = file_stem.to_str() {
                        languages.push(lang_code.to_string());
                    }
                }
            }
        }
        
        Ok(languages)
    }
    
    #[allow(dead_code)]
    pub fn change_language(&mut self, lang_code: &str) -> Result<()> {
        let strings = Self::load_strings(lang_code)?;
        self.current_lang = lang_code.to_string();
        self.strings = strings;
        Ok(())
    }
    
    #[allow(dead_code)]
    pub fn current_language(&self) -> &str {
        &self.current_lang
    }
} 