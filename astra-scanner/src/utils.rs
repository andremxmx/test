use std::net::IpAddr;
use ipnetwork::IpNetwork;
use std::str::FromStr;
use std::time::{Duration, Instant};
use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Validates if a string is a valid IPv4 address
#[allow(dead_code)]
pub fn is_valid_ipv4(ip: &str) -> bool {
    IpAddr::from_str(ip).map_or(false, |addr| addr.is_ipv4())
}

/// Validates if a string is a valid port number
#[allow(dead_code)]
pub fn is_valid_port(port: &str) -> bool {
    if let Ok(num) = port.parse::<u16>() {
        num > 0
    } else {
        false
    }
}

/// Creates a customized progress bar
#[allow(dead_code)]
pub fn create_progress_bar(len: u64, msg: &str) -> ProgressBar {
    let pb = ProgressBar::new(len);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {wide_msg}")
            .unwrap()
            .progress_chars("=>-")
    );
    pb.set_message(msg.to_string());
    pb
}

/// Formats a duration in seconds to a human readable string
#[allow(dead_code)]
pub fn format_duration(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;
    
    if hours > 0 {
        format!("{:02}h {:02}m {:02}s", hours, minutes, secs)
    } else if minutes > 0 {
        format!("{:02}m {:02}s", minutes, secs)
    } else {
        format!("{:02}s", secs)
    }
}

/// Validates if a string is a valid CIDR notation
#[allow(dead_code)]
pub fn is_valid_cidr(cidr: &str) -> bool {
    IpNetwork::from_str(cidr).is_ok()
}

/// Calculates expected time to complete based on progress
#[allow(dead_code)]
pub fn estimate_completion_time(progress: usize, total: usize, start_time: Instant) -> String {
    if progress == 0 {
        return "N/A".to_string();
    }
    
    let elapsed = start_time.elapsed().as_secs();
    let items_per_second = progress as f64 / elapsed as f64;
    
    if items_per_second <= 0.0 {
        return "N/A".to_string();
    }
    
    let remaining_items = total - progress;
    let remaining_seconds = (remaining_items as f64 / items_per_second) as u64;
    
    format_duration(remaining_seconds)
}

/// Creates a throttled task spawner to control concurrency
#[allow(dead_code)]
pub struct TaskThrottler {
    max_concurrent: usize,
    active: Arc<Mutex<usize>>,
}

impl TaskThrottler {
    #[allow(dead_code)]
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            max_concurrent,
            active: Arc::new(Mutex::new(0)),
        }
    }
    
    #[allow(dead_code)]
    pub async fn spawn<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce() -> Result<R> + Send + 'static,
        R: Send + 'static,
    {
        // Wait until there's capacity
        loop {
            let active = {
                let mut active = self.active.lock().await;
                if *active < self.max_concurrent {
                    *active += 1;
                    *active
                } else {
                    0
                }
            };
            
            if active > 0 {
                break;
            }
            
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        
        // Execute the function
        let result = f()?;
        
        // Decrement active count
        let mut active = self.active.lock().await;
        *active -= 1;
        
        Ok(result)
    }
}

/// Limits the rate of requests to avoid overloading servers
#[allow(dead_code)]
pub struct RateLimiter {
    requests_per_second: f64,
    last_request: Arc<Mutex<Instant>>,
}

impl RateLimiter {
    #[allow(dead_code)]
    pub fn new(requests_per_second: f64) -> Self {
        Self {
            requests_per_second,
            last_request: Arc::new(Mutex::new(Instant::now())),
        }
    }
    
    #[allow(dead_code)]
    pub async fn wait(&self) {
        if self.requests_per_second <= 0.0 {
            return;
        }
        
        let interval = Duration::from_secs_f64(1.0 / self.requests_per_second);
        
        let mut last = self.last_request.lock().await;
        let elapsed = last.elapsed();
        
        if elapsed < interval {
            let wait_time = interval - elapsed;
            drop(last); // Release lock before sleeping
            tokio::time::sleep(wait_time).await;
            *self.last_request.lock().await = Instant::now();
        } else {
            *last = Instant::now();
        }
    }
} 