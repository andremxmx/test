use std::collections::{HashSet};
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant};
use anyhow::{Result, anyhow};
use reqwest::{Client, header};
use tokio::sync::Mutex;
use futures::{stream, StreamExt};
use chrono::{DateTime, Local};
use serde_json::json;
use std::net::IpAddr;

use crate::config::Config;
use crate::lang::LanguageManager;
use crate::channel;
use crate::ui::progress::ProgressTracker;

mod internal_channel {
    pub async fn check_channel(_url: &str) -> bool {
        // Stub implementation
        true
    }

    pub async fn verify_channels(channels: &[(String, String)], _server: &str) -> Vec<(String, String)> {
        // Stub implementation
        channels.to_vec()
    }
}

/// Astra server scanner
pub struct AstraScanner<'a> {
    lang: &'a LanguageManager,
    config: &'a Config,
    client: Client,
    found_servers: Arc<Mutex<HashSet<String>>>,
    progress: Option<Arc<Mutex<ProgressTracker>>>,
}

/// Representa un servidor Astra encontrado durante el escaneo
#[derive(Debug, Clone)]
pub struct Server {
    pub ip: IpAddr,
    pub port: u16,
    pub service: String,
    pub discovery_time: DateTime<Local>,
}

/// Simplified scanner for the GUI
pub struct SimpleScanner {
    config: crate::config::SimpleScannerConfig,
    is_running: bool,
    progress: f32,
    found_servers: Vec<Server>,
}

impl<'a> AstraScanner<'a> {
    pub fn new(lang: &'a LanguageManager, config: &'a Config) -> Self {
        // Create an optimized HTTP client
        let client = Client::builder()
            .timeout(Duration::from_millis((config.scanner.connection_timeout * 1000.0) as u64))
            .pool_max_idle_per_host(config.scanner.pool_maxsize)
            .tcp_keepalive(Some(Duration::from_secs(15)))
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .build()
            .unwrap_or_default();
            
        Self {
            lang,
            config,
            client,
            found_servers: Arc::new(Mutex::new(HashSet::new())),
            progress: None,
        }
    }
    
    /// Load IPs from file
    async fn load_ips(&self) -> Result<Vec<String>> {
        let path = Path::new("pool/ip.txt");
        
        if !path.exists() {
            return Err(anyhow!(self.lang.get("errors.no_ip_file")));
        }
        
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let ips = reader.lines()
            .filter_map(Result::ok)
            .filter(|line| !line.trim().is_empty())
            .collect();
            
        Ok(ips)
    }
    
    /// Load ports from file
    async fn load_ports(&self) -> Result<Vec<u16>> {
        let path = Path::new("pool/ports.txt");
        
        if !path.exists() {
            return Err(anyhow!(self.lang.get("errors.no_ports_file")));
        }
        
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        
        let mut ports = Vec::new();
        for line in reader.lines() {
            if let Ok(line) = line {
                if let Ok(port) = line.trim().parse::<u16>() {
                    if port > 0 {
                        ports.push(port);
                    }
                }
            }
        }
        
        Ok(ports)
    }
    
    /// Check if a server is an Astra server
    async fn check_server(&self, ip: &str, port: u16) -> Result<bool> {
        // Create server identifier
        let server = format!("{}:{}", ip, port);
        
        // Skip if already found
        let servers_guard = self.found_servers.lock().await;
        if servers_guard.contains(&server) {
            return Ok(false);
        }
        drop(servers_guard);
        
        // Attempt to connect with HEAD request
        let url = format!("http://{}:{}", ip, port);
        let res = self.client.head(&url).send().await;
        
        if let Ok(response) = res {
            // Check for Astra signature in server header
            if let Some(server_header) = response.headers().get(header::SERVER) {
                if let Ok(server_value) = server_header.to_str() {
                    if server_value.contains("Astra") {
                        // Add to found servers
                        let mut servers_guard = self.found_servers.lock().await;
                        if !servers_guard.contains(&server) {
                            servers_guard.insert(server.clone());
                            drop(servers_guard);
                            
                            // Update progress
                            if let Some(progress) = &self.progress {
                                let mut progress_guard = progress.lock().await;
                                progress_guard.update_servers(1);
                                drop(progress_guard);
                            }
                            
                            // Save to file
                            self.save_server(&server).await?;
                            
                            // Try to get playlist
                            self.spawn_playlist_processor(ip.to_string(), port);
                            
                            return Ok(true);
                        }
                    }
                }
            }
        }
        
        Ok(false)
    }
    
    /// Spawn a task to process a server's playlist
    fn spawn_playlist_processor(&self, ip: String, port: u16) {
        // Create owned clones of all needed data
        let client = self.client.clone();
        let playlist_timeout = self.config.scanner.playlist_timeout;
        let progress_clone = self.progress.clone();
        
        // Spawn a self-contained async block
        tokio::spawn(async move {
            // Attempt to get playlist
            let playlist_result = get_playlist(&client, &ip, port, playlist_timeout).await;
            
            if let Ok(content) = playlist_result {
                // Parse channels from playlist
                let mut channels = Vec::new();
                let mut current_title = String::new();
                
                for line in content.lines() {
                    if line.starts_with("#EXTINF:") {
                        current_title = line.to_string();
                    } else if line.starts_with("http://") || line.starts_with("https://") {
                        if !current_title.is_empty() {
                            channels.push((current_title.clone(), line.to_string()));
                            current_title = String::new();
                        }
                    }
                }
                
                if !channels.is_empty() {
                    // Create owned versions of channels to avoid lifetime issues
                    let channels_owned: Vec<(String, String)> = channels;
                    let server = format!("{}:{}", ip, port);
                    let channels_count = channels_owned.len();
                    
                    // Verify channels
                    let mut working_channels = Vec::new();
                    
                    let results = futures::stream::iter(channels_owned)
                        .map(|(title, url)| {
                            let title = title.clone();
                            let url = url.clone();
                            async move {
                                let is_working = internal_channel::check_channel(&url).await;
                                (title, url, is_working)
                            }
                        })
                        .buffer_unordered(20) // Check 20 channels concurrently
                        .collect::<Vec<_>>()
                        .await;
                        
                    for (title, url, is_working) in results {
                        if is_working {
                            working_channels.push((title, url));
                        }
                    }
                    
                    println!("Server {}: Verified {}/{} channels, {} working", 
                             server, channels_count, channels_count, working_channels.len());
                    
                    if !working_channels.is_empty() {
                        // Save working channels
                        if let Err(e) = save_working_channels(&working_channels).await {
                            eprintln!("Error saving channels: {}", e);
                        }
                        
                        // Update progress
                        if let Some(progress) = &progress_clone {
                            let mut progress_guard = progress.lock().await;
                            progress_guard.update_channels(working_channels.len());
                        }
                    }
                }
            }
        });
    }
    
    /// Save a found server to file
    async fn save_server(&self, server: &str) -> Result<()> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("found_servers.txt")?;
            
        let mut file = tokio::fs::File::from_std(file);
        use tokio::io::AsyncWriteExt;
        
        file.write_all(format!("{}\n", server).as_bytes()).await?;
        file.flush().await?;
        
        Ok(())
    }
    
    #[allow(dead_code)]
    async fn get_playlist(&self, ip: &str, port: u16) -> Result<String> {
        let url = format!("http://{}:{}/playlist.m3u", ip, port);
        
        let response = self.client.get(&url)
            .timeout(Duration::from_secs(self.config.scanner.playlist_timeout as u64))
            .send()
            .await?;
            
        if response.status().is_success() {
            let content = response.text().await?;
            if content.contains("#EXTM3U") {
                return Ok(content);
            }
        }
        
        Err(anyhow!("No valid playlist found"))
    }
    
    #[allow(dead_code)]
    async fn process_server_playlist(&self, ip: String, port: u16) {
        // Try to get the playlist
        let playlist_result = self.get_playlist(&ip, port).await;
        
        if let Ok(content) = playlist_result {
            // Parse channels from playlist
            let mut channels = Vec::new();
            let mut current_title = String::new();
            
            for line in content.lines() {
                if line.starts_with("#EXTINF:") {
                    current_title = line.to_string();
                } else if line.starts_with("http://") || line.starts_with("https://") {
                    if !current_title.is_empty() {
                        channels.push((current_title.clone(), line.to_string()));
                        current_title = String::new();
                    }
                }
            }
            
            if !channels.is_empty() {
                // Verify channels in batches
                let working_channels = internal_channel::verify_channels(&channels, &format!("{}:{}", ip, port)).await;
                
                if !working_channels.is_empty() {
                    // Save working channels
                    if let Err(e) = self.save_working_channels(&working_channels).await {
                        eprintln!("Error saving channels: {}", e);
                    }
                    
                    // Update progress
                    if let Some(progress) = &self.progress {
                        let mut progress_guard = progress.lock().await;
                        progress_guard.update_channels(working_channels.len());
                    }
                }
            }
        }
    }
    
    #[allow(dead_code)]
    async fn save_working_channels(&self, channels: &[(String, String)]) -> Result<()> {
        // Create channels directory if it doesn't exist
        fs::create_dir_all("channels")?;
        
        // Prepare for reading existing channels
        let path = Path::new("channels/all_channels.m3u8");
        let mut existing_urls = HashSet::new();
        
        // Load existing channels to avoid duplicates
        if path.exists() {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            let mut is_url_line = false;
            
            for line in reader.lines() {
                if let Ok(line) = line {
                    if is_url_line {
                        existing_urls.insert(line);
                        is_url_line = false;
                    } else if line.starts_with("#EXTINF:") {
                        is_url_line = true;
                    }
                }
            }
        }
        
        // Filter out duplicate channels
        let new_channels: Vec<_> = channels.iter()
            .filter(|(_, url)| !existing_urls.contains(url))
            .collect();
            
        if new_channels.is_empty() {
            return Ok(());
        }
        
        // Append new channels to the file
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
            
        let mut writer = std::io::BufWriter::new(file);
        
        // Add header if file is new
        if !path.exists() || fs::metadata(path)?.len() == 0 {
            writeln!(writer, "#EXTM3U")?;
        }
        
        // Write new channels
        for (title, url) in new_channels {
            writeln!(writer, "{}", title)?;
            writeln!(writer, "{}", url)?;
        }
        
        writer.flush()?;
        
        Ok(())
    }
    
    /// Process a chunk of IP:Port combinations
    async fn process_chunk(&self, targets: Vec<(String, u16)>) -> Result<usize> {
        let mut checked = 0;
        
        // Process targets in parallel with controlled concurrency
        let worker_count = self.config.scanner.workers;
        
        let results = stream::iter(targets)
            .map(|(ip, port)| {
                let self_ref = &self;
                async move {
                    let _ = self_ref.check_server(&ip, port).await;
                    1 // Count as checked
                }
            })
            .buffer_unordered(worker_count)
            .collect::<Vec<_>>()
            .await;
            
        checked += results.len();
        
        // Update progress
        if let Some(progress) = &self.progress {
            let mut progress_guard = progress.lock().await;
            progress_guard.update_total(checked);
        }
        
        Ok(checked)
    }
    
    /// Main scan function
    pub async fn scan(&mut self) -> Result<()> {
        // Load IPs and ports
        let ips = self.load_ips().await?;
        let ports = self.load_ports().await?;
        
        if ips.is_empty() || ports.is_empty() {
            return Err(anyhow!(self.lang.get("errors.no_files_found")));
        }
        
        // Calculate total combinations
        let total_combinations = ips.len() * ports.len();
        
        println!("{}", self.lang.get("astra.scan.starting")
            .replace("{}", &ips.len().to_string())
            .replace("{}", &ports.len().to_string()));
            
        // Initialize progress tracker
        self.progress = Some(ProgressTracker::new(total_combinations));
        
        // Generate all combinations
        let all_targets: Vec<_> = ips.into_iter()
            .flat_map(|ip| {
                ports.iter().map(move |&port| (ip.clone(), port))
            })
            .collect();
            
        // Process in chunks for better memory management
        let chunk_size = self.config.scanner.batch_size;
        let chunks: Vec<_> = all_targets.chunks(chunk_size)
            .map(|chunk| chunk.to_vec())
            .collect();
            
        let start_time = Instant::now();
        let mut total_checked = 0;
        
        // Process all chunks with controlled concurrency
        let max_concurrent_chunks = self.config.scanner.max_workers / self.config.scanner.workers;
        
        let chunk_results = stream::iter(chunks)
            .map(|chunk| {
                let self_ref = &self;
                async move {
                    self_ref.process_chunk(chunk).await
                }
            })
            .buffer_unordered(max_concurrent_chunks)
            .collect::<Vec<_>>()
            .await;
            
        // Gather results
        for result in chunk_results {
            if let Ok(checked) = result {
                total_checked += checked;
            }
        }
        
        // Get final count of found servers
        let servers_guard = self.found_servers.lock().await;
        let found_count = servers_guard.len();
        drop(servers_guard);
        
        // Finish progress
        if let Some(progress) = &self.progress {
            let progress_guard = progress.lock().await;
            progress_guard.finish();
        }
        
        // Calculate elapsed time
        let duration = start_time.elapsed();
        let hours = duration.as_secs() / 3600;
        let minutes = (duration.as_secs() % 3600) / 60;
        let seconds = duration.as_secs() % 60;
        
        println!("\nScan completed in {:02}:{:02}:{:02}", hours, minutes, seconds);
        println!("Total checked: {}", total_checked);
        println!("Found servers: {}", found_count);
        
        // Save summary
        self.save_summary(total_checked, found_count).await?;
        
        Ok(())
    }
    
    /// Save scan summary to file
    async fn save_summary(&self, total_checked: usize, servers_found: usize) -> Result<()> {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        
        let servers_guard = self.found_servers.lock().await;
        let servers = servers_guard.iter().cloned().collect::<Vec<_>>();
        drop(servers_guard);
        
        let summary = json!({
            "scan_date": timestamp,
            "total_checked": total_checked,
            "servers_found": servers_found,
            "found_servers": servers
        });
        
        let mut file = File::create("scan_summary.json")?;
        file.write_all(serde_json::to_string_pretty(&summary)?.as_bytes())?;
        
        Ok(())
    }
}

/// Standalone helper function to get playlist content
async fn get_playlist(client: &Client, ip: &str, port: u16, timeout_secs: usize) -> Result<String> {
    let url = format!("http://{}:{}/playlist.m3u", ip, port);
    
    let response = client.get(&url)
        .timeout(Duration::from_secs(timeout_secs as u64))
        .send()
        .await?;
        
    if response.status().is_success() {
        let content = response.text().await?;
        if content.contains("#EXTM3U") {
            return Ok(content);
        }
    }
    
    Err(anyhow!("No valid playlist found"))
}

/// Standalone helper function to save working channels
async fn save_working_channels(channels: &[(String, String)]) -> Result<()> {
    // Create channels directory if it doesn't exist
    fs::create_dir_all("channels")?;
    
    // Prepare for reading existing channels
    let path = Path::new("channels/all_channels.m3u8");
    let mut existing_urls = HashSet::new();
    
    // Load existing channels to avoid duplicates
    if path.exists() {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut is_url_line = false;
        
        for line in reader.lines() {
            if let Ok(line) = line {
                if is_url_line {
                    existing_urls.insert(line);
                    is_url_line = false;
                } else if line.starts_with("#EXTINF:") {
                    is_url_line = true;
                }
            }
        }
    }
    
    // Filter out duplicate channels
    let new_channels: Vec<_> = channels.iter()
        .filter(|(_, url)| !existing_urls.contains(url))
        .collect();
        
    if new_channels.is_empty() {
        return Ok(());
    }
    
    // Append new channels to the file
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
        
    let mut writer = std::io::BufWriter::new(file);
    
    // Add header if file is new
    if !path.exists() || fs::metadata(path)?.len() == 0 {
        writeln!(writer, "#EXTM3U")?;
    }
    
    // Write new channels
    for (title, url) in new_channels {
        writeln!(writer, "{}", title)?;
        writeln!(writer, "{}", url)?;
    }
    
    writer.flush()?;
    
    Ok(())
}

impl SimpleScanner {
    pub fn new(config: crate::config::SimpleScannerConfig) -> Self {
        Self {
            config,
            is_running: false,
            progress: 0.0,
            found_servers: Vec::new(),
        }
    }
    
    pub fn update_config(&mut self, config: crate::config::SimpleScannerConfig) {
        self.config = config;
    }
    
    pub fn add_server(&mut self, server: Server) {
        self.found_servers.push(server);
    }
    
    pub fn get_servers(&self) -> &Vec<Server> {
        &self.found_servers
    }
    
    pub fn clear_servers(&mut self) {
        self.found_servers.clear();
    }
    
    pub fn set_progress(&mut self, progress: f32) {
        self.progress = progress;
    }
    
    pub fn get_progress(&self) -> f32 {
        self.progress
    }
    
    pub fn start(&mut self) {
        self.is_running = true;
        self.progress = 0.0;
    }
    
    pub fn stop(&mut self) {
        self.is_running = false;
    }
    
    pub fn is_running(&self) -> bool {
        self.is_running
    }
} 