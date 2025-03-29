use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use anyhow::{Result, Context};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub scanner: ScannerConfig,
    pub asn: ASNConfig,
    pub language: LanguageConfig,
    pub app: AppConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScannerConfig {
    pub max_workers: usize,
    pub timeout: f64,
    pub chunk_size: usize,
    pub max_retries: usize,
    pub workers: usize,
    pub batch_size: usize,
    pub connection_timeout: f64,
    pub playlist_timeout: usize,
    pub channel_timeout: usize,
    pub pool_connections: usize,
    pub pool_maxsize: usize,
}

/// Simple config structure for GUI mode
#[derive(Clone, Debug)]
pub struct SimpleScannerConfig {
    pub threads: u32,
    pub timeout: Duration,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ASNConfig {
    pub max_workers: usize,
    pub api_timeout: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LanguageConfig {
    pub default: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub lang: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            scanner: ScannerConfig {
                max_workers: 3000,
                timeout: 0.8,
                chunk_size: 10,
                max_retries: 0,
                workers: 200,
                batch_size: 1000,
                connection_timeout: 0.5,
                playlist_timeout: 5,
                channel_timeout: 2,
                pool_connections: 50,
                pool_maxsize: 50,
            },
            asn: ASNConfig {
                max_workers: 20,
                api_timeout: 5,
            },
            language: LanguageConfig {
                default: "en".to_string(),
            },
            app: AppConfig {
                lang: "en".to_string(),
            },
        }
    }
}

impl Default for SimpleScannerConfig {
    fn default() -> Self {
        Self {
            threads: 4,
            timeout: Duration::from_secs(5),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        // Ensure pool directory exists
        fs::create_dir_all("pool")?;
        
        let config_path = Path::new("pool/config.json");
        
        if !config_path.exists() {
            // Create default config if it doesn't exist
            let default_config = Config::default();
            default_config.save()?;
            return Ok(default_config);
        }
        
        let mut file = File::open(config_path)
            .with_context(|| format!("Failed to open config file: {:?}", config_path))?;
            
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .with_context(|| "Failed to read config file")?;
            
        serde_json::from_str(&contents)
            .with_context(|| "Failed to parse config.json")
    }
    
    pub fn save(&self) -> Result<()> {
        // Ensure pool directory exists
        fs::create_dir_all("pool")?;
        
        let config_path = Path::new("pool/config.json");
        let mut file = File::create(config_path)
            .with_context(|| format!("Failed to create config file: {:?}", config_path))?;
            
        let json = serde_json::to_string_pretty(self)
            .with_context(|| "Failed to serialize config to JSON")?;
            
        file.write_all(json.as_bytes())
            .with_context(|| "Failed to write config.json")?;
            
        Ok(())
    }
} 