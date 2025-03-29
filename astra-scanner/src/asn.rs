use anyhow::{Result, Context, anyhow};
use std::fs::{self, File};
use std::io::{BufWriter, Write, BufReader, BufRead};
use std::path::Path;
use std::collections::HashSet;
use ipnetwork::IpNetwork;
use scraper::{Html, Selector};
use reqwest::Client;
use tokio::time::Duration;
use futures::{stream, StreamExt};

use crate::config::Config;
use crate::lang::LanguageManager;
use crate::ui::progress::ASNProgressTracker;

/// ASN Scanner for discovering IP ranges by country
pub struct ASNScanner<'a> {
    lang: &'a LanguageManager,
    config: &'a Config,
    client: Client,
}

impl<'a> ASNScanner<'a> {
    pub fn new(lang: &'a LanguageManager, config: &'a Config) -> Self {
        let client = Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .timeout(Duration::from_secs(config.asn.api_timeout as u64))
            .build()
            .unwrap_or_default();
            
        Self {
            lang,
            config,
            client,
        }
    }
    
    /// Get ASNs for a specific country
    async fn get_asns_for_country(&self, country_code: &str) -> Result<Vec<String>> {
        let url = format!("https://ipinfo.io/countries/{}", country_code.to_lowercase());
        
        let response = self.client.get(&url).send().await
            .with_context(|| format!("Failed to fetch ASNs for country {}", country_code))?;
            
        if !response.status().is_success() {
            return Err(anyhow!("Failed to fetch ASNs: HTTP {}", response.status()));
        }
        
        let html = response.text().await?;
        let document = Html::parse_document(&html);
        
        // Use scraper to extract ASNs from the HTML
        let asn_selector = Selector::parse("a[href*='/AS']").unwrap();
        
        let asns: HashSet<String> = document
            .select(&asn_selector)
            .filter_map(|element| {
                let href = element.value().attr("href")?;
                if href.contains("/AS") {
                    let parts: Vec<&str> = href.split("/AS").collect();
                    if parts.len() > 1 {
                        let asn_part = parts[1].split('/').next()?;
                        if asn_part.chars().all(|c| c.is_ascii_digit()) {
                            return Some(asn_part.to_string());
                        }
                    }
                }
                None
            })
            .collect();
            
        Ok(asns.into_iter().collect())
    }
    
    /// Get CIDRs for a specific ASN
    async fn get_cidrs_for_asn(&self, asn: &str) -> Result<Vec<String>> {
        let url = format!("https://ipinfo.io/AS{}", asn);
        
        let response = self.client.get(&url).send().await
            .with_context(|| format!("Failed to fetch CIDRs for ASN {}", asn))?;
            
        if !response.status().is_success() {
            return Err(anyhow!("Failed to fetch CIDRs: HTTP {}", response.status()));
        }
        
        let html = response.text().await?;
        let document = Html::parse_document(&html);
        
        // Use scraper to extract prefixes/CIDRs from the HTML
        let td_selector = Selector::parse("td").unwrap();
        
        let cidrs: HashSet<String> = document
            .select(&td_selector)
            .filter_map(|element| {
                let text = element.text().collect::<String>().trim().to_string();
                if text.contains('/') && !text.contains(':') { // Exclude IPv6
                    if let Ok(_) = text.parse::<IpNetwork>() {
                        return Some(text);
                    }
                }
                None
            })
            .collect();
            
        Ok(cidrs.into_iter().collect())
    }
    
    /// Save CIDR ranges to a file
    fn save_ranges(&self, ranges: &[String], filename: &str) -> Result<()> {
        if ranges.is_empty() {
            println!("No ranges to save");
            return Ok(());
        }
        
        fs::create_dir_all("pool")?;
        let path = Path::new("pool").join(filename);
        
        let file = File::create(&path)
            .with_context(|| format!("Failed to create file: {:?}", path))?;
            
        let mut writer = BufWriter::new(file);
        
        for range in ranges {
            writeln!(writer, "{}", range)?;
        }
        
        println!("Successfully saved {} ranges to {}", ranges.len(), filename);
        Ok(())
    }
    
    /// Save IPs to the ip.txt file
    fn save_ips(&self, ips: &[String]) -> Result<()> {
        if ips.is_empty() {
            println!("No IPs to save");
            return Ok(());
        }
        
        fs::create_dir_all("pool")?;
        let path = Path::new("pool/ip.txt");
        
        // Load existing IPs
        let mut existing_ips = HashSet::new();
        if path.exists() {
            let file = File::open(&path)?;
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(ip) = line {
                    existing_ips.insert(ip);
                }
            }
        }
        
        // Filter new IPs
        let new_ips: Vec<&String> = ips.iter()
            .filter(|ip| !existing_ips.contains(*ip))
            .collect();
            
        if new_ips.is_empty() {
            println!("No new IPs to add");
            return Ok(());
        }
        
        // Append new IPs
        let file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)?;
            
        let mut writer = BufWriter::new(file);
        
        for ip in &new_ips {
            writeln!(writer, "{}", ip)?;
        }
        
        println!("Added {} new IPs to pool/ip.txt", new_ips.len());
        Ok(())
    }
    
    /// Expand CIDRs into individual IP addresses
    fn expand_cidrs(&self, cidrs: &[String]) -> Result<Vec<String>> {
        let mut all_ips = Vec::new();
        
        for cidr_str in cidrs {
            match cidr_str.parse::<IpNetwork>() {
                Ok(network) => {
                    // Only process IPv4 networks
                    if network.is_ipv4() {
                        // For small networks, add all IPs
                        if network.prefix() >= 16 {
                            for ip in network.iter() {
                                all_ips.push(ip.to_string());
                            }
                        } else {
                            println!("Skipping large network {} (/{}) - too many IPs", 
                                     network.network(), network.prefix());
                        }
                    }
                },
                Err(e) => {
                    println!("Error parsing CIDR {}: {}", cidr_str, e);
                }
            }
        }
        
        Ok(all_ips)
    }
    
    /// Process a country to find ASNs, CIDRs, and IPs
    pub async fn process_country(&self, country_code: &str) -> Result<()> {
        if country_code.len() != 2 {
            return Err(anyhow!("Invalid country code. Please use a 2-letter country code (e.g., US, ES, BR)"));
        }
        
        let country_code = country_code.to_uppercase();
        println!("\nProcessing ASNs for country: {}", country_code);
        println!("{}", self.lang.get("asn.start_msg"));
        
        // Get ASNs for the country
        let asns = self.get_asns_for_country(&country_code).await?;
        
        if asns.is_empty() {
            println!("{}", self.lang.get("asn.no_asn_found"));
            return Ok(());
        }
        
        println!("Found {} ASNs for {}", asns.len(), country_code);
        
        // Create progress tracker
        let progress = ASNProgressTracker::new(asns.len());
        
        // Get CIDRs for each ASN concurrently
        let max_concurrent = self.config.asn.max_workers;
        let mut all_cidrs = Vec::new();
        
        let results = stream::iter(asns.iter())
            .map(|asn| {
                let self_ref = &self;
                async move {
                    let result = self_ref.get_cidrs_for_asn(asn).await;
                    (asn, result)
                }
            })
            .buffer_unordered(max_concurrent)
            .collect::<Vec<_>>()
            .await;
            
        for (asn, result) in results {
            match result {
                Ok(cidrs) => {
                    if !cidrs.is_empty() {
                        println!("ASN {}: Found {} CIDRs", asn, cidrs.len());
                        all_cidrs.extend(cidrs);
                    }
                },
                Err(e) => {
                    println!("Error processing ASN {}: {}", asn, e);
                }
            }
            
            let mut progress_guard = progress.lock().await;
            progress_guard.update_asn(1);
        }
        
        if all_cidrs.is_empty() {
            println!("{}", self.lang.get("asn.no_ranges_found"));
            return Ok(());
        }
        
        println!("\nTotal CIDRs found: {}", all_cidrs.len());
        
        let mut progress_guard = progress.lock().await;
        progress_guard.set_cidr_count(all_cidrs.len());
        drop(progress_guard);
        
        // Save CIDRs
        let filename = format!("asn_{}.txt", country_code.to_lowercase());
        self.save_ranges(&all_cidrs, &filename)?;
        
        // Expand CIDRs to individual IPs
        println!("\nConverting CIDRs to individual IPs...");
        let all_ips = self.expand_cidrs(&all_cidrs)?;
        
        if all_ips.is_empty() {
            println!("No IPs were generated from the CIDRs");
            return Ok(());
        }
        
        println!("\nTotal IPs generated: {}", all_ips.len());
        
        // Save IPs
        self.save_ips(&all_ips)?;
        
        let progress_guard = progress.lock().await;
        progress_guard.finish();
        
        Ok(())
    }
} 