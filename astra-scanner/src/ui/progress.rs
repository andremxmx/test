use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::Instant;

/// Progress tracker for the Astra scanner
#[allow(dead_code)]
pub struct ProgressTracker {
    multi: MultiProgress,
    total_bar: ProgressBar,
    servers_bar: ProgressBar,
    channels_bar: ProgressBar,
    total_checks: usize,
    found_servers: usize,
    working_channels: usize,
}

impl ProgressTracker {
    pub fn new(total_checks: usize) -> Arc<Mutex<Self>> {
        let multi = MultiProgress::new();
        
        // Create progress bar for total progress
        let total_bar = multi.add(ProgressBar::new(total_checks as u64));
        total_bar.set_style(
            ProgressStyle::default_bar()
                .template("{prefix:.cyan.bold} [{wide_bar:.cyan}] {pos}/{len} checks ({eta})")
                .unwrap()
        );
        total_bar.set_prefix("Total progress");
        
        // Create progress bar for found servers
        let servers_bar = multi.add(ProgressBar::new(0));
        servers_bar.set_style(
            ProgressStyle::default_bar()
                .template("{prefix:.green.bold} {pos} found")
                .unwrap()
        );
        servers_bar.set_prefix("Astra Servers");
        
        // Create progress bar for working channels
        let channels_bar = multi.add(ProgressBar::new(0));
        channels_bar.set_style(
            ProgressStyle::default_bar()
                .template("{prefix:.yellow.bold} {pos} working")
                .unwrap()
        );
        channels_bar.set_prefix("Channels");
        
        Arc::new(Mutex::new(Self {
            multi,
            total_bar,
            servers_bar,
            channels_bar,
            total_checks,
            found_servers: 0,
            working_channels: 0,
        }))
    }
    
    pub fn update_total(&mut self, increment: usize) {
        self.total_bar.inc(increment as u64);
    }
    
    pub fn update_servers(&mut self, increment: usize) {
        self.found_servers += increment;
        self.servers_bar.set_position(self.found_servers as u64);
    }
    
    pub fn update_channels(&mut self, increment: usize) {
        self.working_channels += increment;
        self.channels_bar.set_position(self.working_channels as u64);
    }
    
    pub fn finish(&self) {
        self.total_bar.finish_with_message("Scan completed");
    }
}

/// Progress tracker for the ASN scanner
#[allow(dead_code)]
pub struct ASNProgressTracker {
    multi: MultiProgress,
    asn_bar: ProgressBar,
    cidr_bar: ProgressBar,
    ip_bar: ProgressBar,
    start_time: Instant,
}

impl ASNProgressTracker {
    pub fn new(total_asns: usize) -> Arc<Mutex<Self>> {
        let multi = MultiProgress::new();
        
        // Create progress bar for ASNs
        let asn_bar = multi.add(ProgressBar::new(total_asns as u64));
        asn_bar.set_style(
            ProgressStyle::default_bar()
                .template("{prefix:.blue.bold} [{wide_bar:.blue}] {pos}/{len} ({eta})")
                .unwrap()
        );
        asn_bar.set_prefix("Processing ASNs");
        
        // Create progress bar for CIDRs
        let cidr_bar = multi.add(ProgressBar::new(0));
        cidr_bar.set_style(
            ProgressStyle::default_bar()
                .template("{prefix:.green.bold} {pos} found")
                .unwrap()
        );
        cidr_bar.set_prefix("CIDRs");
        
        // Create progress bar for IPs
        let ip_bar = multi.add(ProgressBar::new(0));
        ip_bar.set_style(
            ProgressStyle::default_bar()
                .template("{prefix:.yellow.bold} {msg}")
                .unwrap()
        );
        ip_bar.set_prefix("IPs");
        ip_bar.set_message("Waiting for CIDR expansion...");
        
        Arc::new(Mutex::new(Self {
            multi,
            asn_bar,
            cidr_bar,
            ip_bar,
            start_time: Instant::now(),
        }))
    }
    
    pub fn update_asn(&mut self, increment: usize) {
        self.asn_bar.inc(increment as u64);
    }
    
    pub fn set_cidr_count(&mut self, count: usize) {
        self.cidr_bar.set_position(count as u64);
    }
    
    #[allow(dead_code)]
    pub fn update_ip_progress(&mut self, current: usize, total: usize) {
        self.ip_bar.set_message(format!("Generated {} of {} IPs ({:.1}%)", 
            current, total, (current as f64 / total as f64) * 100.0));
    }
    
    pub fn finish(&self) {
        self.asn_bar.finish_with_message("ASN processing completed");
        self.ip_bar.finish_with_message("IP generation completed");
    }
} 