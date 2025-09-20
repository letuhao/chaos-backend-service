//! Security service for API Gateway

use crate::config::{Config, SecurityConfig};
use crate::errors::{ApiGatewayError, Result};
use crate::types::{SecurityEvent, SecurityEventType, SecuritySeverity, RequestContext};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;
use tracing::{info, debug, warn, error};

/// Security service
#[derive(Debug, Clone)]
pub struct SecurityService {
    config: SecurityConfig,
    security_events: Arc<tokio::sync::RwLock<Vec<SecurityEvent>>>,
    ip_blacklist: Arc<tokio::sync::RwLock<HashMap<String, SystemTime>>>,
    ip_whitelist: Arc<tokio::sync::RwLock<HashMap<String, ()>>>,
    suspicious_ips: Arc<tokio::sync::RwLock<HashMap<String, u32>>>,
}

impl SecurityService {
    /// Create a new security service
    pub fn new(config: Config) -> Result<Self> {
        let security_config = config.security.clone();
        
        Ok(Self {
            config: security_config,
            security_events: Arc::new(tokio::sync::RwLock::new(Vec::new())),
            ip_blacklist: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            ip_whitelist: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            suspicious_ips: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        })
    }

    /// Check if IP is blacklisted
    pub async fn is_ip_blacklisted(&self, ip: &str) -> Result<bool> {
        if !self.config.ip_blacklist.enabled {
            return Ok(false);
        }

        let blacklist = self.ip_blacklist.read().await;
        
        // Check if IP is in blacklist
        if blacklist.contains_key(ip) {
            // Check if ban has expired
            if let Some(ban_time) = blacklist.get(ip) {
                let ban_duration = SystemTime::now().duration_since(*ban_time).unwrap_or_default();
                if ban_duration.as_secs() > self.config.ip_blacklist.ban_duration {
                    // Ban has expired, remove from blacklist
                    drop(blacklist);
                    let mut blacklist = self.ip_blacklist.write().await;
                    blacklist.remove(ip);
                    return Ok(false);
                }
            }
            return Ok(true);
        }

        Ok(false)
    }

    /// Check if IP is whitelisted
    pub async fn is_ip_whitelisted(&self, ip: &str) -> Result<bool> {
        if !self.config.ip_whitelist.enabled {
            return Ok(false);
        }

        let whitelist = self.ip_whitelist.read().await;
        Ok(whitelist.contains_key(ip))
    }

    /// Add IP to blacklist
    pub async fn blacklist_ip(&self, ip: &str, duration_seconds: Option<u64>) -> Result<()> {
        debug!("Blacklisting IP: {} for {} seconds", ip, duration_seconds.unwrap_or(self.config.ip_blacklist.ban_duration));

        let mut blacklist = self.ip_blacklist.write().await;
        blacklist.insert(ip.to_string(), SystemTime::now());

        // Record security event
        let event = SecurityEvent {
            event_type: SecurityEventType::UnauthorizedAccess,
            severity: SecuritySeverity::Medium,
            description: format!("IP {} added to blacklist", ip),
            ip_address: ip.to_string(),
            user_id: None,
            timestamp: SystemTime::now(),
            metadata: HashMap::new(),
        };

        self.record_security_event(event).await?;

        info!("IP blacklisted: {}", ip);
        Ok(())
    }

    /// Remove IP from blacklist
    pub async fn unblacklist_ip(&self, ip: &str) -> Result<()> {
        debug!("Removing IP from blacklist: {}", ip);

        let mut blacklist = self.ip_blacklist.write().await;
        blacklist.remove(ip);

        info!("IP removed from blacklist: {}", ip);
        Ok(())
    }

    /// Add IP to whitelist
    pub async fn whitelist_ip(&self, ip: &str) -> Result<()> {
        debug!("Adding IP to whitelist: {}", ip);

        let mut whitelist = self.ip_whitelist.write().await;
        whitelist.insert(ip.to_string(), ());

        info!("IP added to whitelist: {}", ip);
        Ok(())
    }

    /// Remove IP from whitelist
    pub async fn unwhitelist_ip(&self, ip: &str) -> Result<()> {
        debug!("Removing IP from whitelist: {}", ip);

        let mut whitelist = self.ip_whitelist.write().await;
        whitelist.remove(ip);

        info!("IP removed from whitelist: {}", ip);
        Ok(())
    }

    /// Check for suspicious activity
    pub async fn check_suspicious_activity(&self, context: &RequestContext) -> Result<bool> {
        debug!("Checking suspicious activity for IP: {}", context.ip_address);

        // Check for rapid requests
        if self.is_rapid_requests(context).await? {
            let event = SecurityEvent {
                event_type: SecurityEventType::DDoS,
                severity: SecuritySeverity::High,
                description: "Rapid requests detected".to_string(),
                ip_address: context.ip_address.clone(),
                user_id: context.user_id.clone(),
                timestamp: SystemTime::now(),
                metadata: HashMap::new(),
            };
            self.record_security_event(event).await?;
            return Ok(true);
        }

        // Check for malicious patterns
        if self.is_malicious_request(context).await? {
            let event = SecurityEvent {
                event_type: SecurityEventType::MaliciousRequest,
                severity: SecuritySeverity::High,
                description: "Malicious request pattern detected".to_string(),
                ip_address: context.ip_address.clone(),
                user_id: context.user_id.clone(),
                timestamp: SystemTime::now(),
                metadata: HashMap::new(),
            };
            self.record_security_event(event).await?;
            return Ok(true);
        }

        Ok(false)
    }

    /// Check for rapid requests
    async fn is_rapid_requests(&self, context: &RequestContext) -> Result<bool> {
        let mut suspicious_ips = self.suspicious_ips.write().await;
        let count = suspicious_ips.entry(context.ip_address.clone()).or_insert(0);
        *count += 1;

        // If more than 100 requests in the last minute, consider it suspicious
        if *count > 100 {
            return Ok(true);
        }

        Ok(false)
    }

    /// Check for malicious request patterns
    async fn is_malicious_request(&self, context: &RequestContext) -> Result<bool> {
        // Check for SQL injection patterns
        if self.contains_sql_injection(&context.query_params) {
            return Ok(true);
        }

        // Check for XSS patterns
        if self.contains_xss(&context.query_params) {
            return Ok(true);
        }

        // Check for path traversal patterns
        if self.contains_path_traversal(&context.path_params) {
            return Ok(true);
        }

        Ok(false)
    }

    /// Check for SQL injection patterns
    fn contains_sql_injection(&self, params: &HashMap<String, String>) -> bool {
        let sql_patterns = [
            "' OR '1'='1",
            "'; DROP TABLE",
            "UNION SELECT",
            "INSERT INTO",
            "DELETE FROM",
            "UPDATE SET",
            "ALTER TABLE",
            "CREATE TABLE",
        ];

        for (_, value) in params {
            for pattern in &sql_patterns {
                if value.to_uppercase().contains(pattern) {
                    return true;
                }
            }
        }

        false
    }

    /// Check for XSS patterns
    fn contains_xss(&self, params: &HashMap<String, String>) -> bool {
        let xss_patterns = [
            "<script>",
            "javascript:",
            "onload=",
            "onerror=",
            "onclick=",
            "onmouseover=",
            "onfocus=",
            "onblur=",
        ];

        for (_, value) in params {
            for pattern in &xss_patterns {
                if value.to_lowercase().contains(pattern) {
                    return true;
                }
            }
        }

        false
    }

    /// Check for path traversal patterns
    fn contains_path_traversal(&self, params: &HashMap<String, String>) -> bool {
        let path_traversal_patterns = [
            "../",
            "..\\",
            "....//",
            "....\\\\",
            "%2e%2e%2f",
            "%2e%2e%5c",
        ];

        for (_, value) in params {
            for pattern in &path_traversal_patterns {
                if value.contains(pattern) {
                    return true;
                }
            }
        }

        false
    }

    /// Record security event
    pub async fn record_security_event(&self, event: SecurityEvent) -> Result<()> {
        debug!("Recording security event: {:?}", event.event_type);

        let mut security_events = self.security_events.write().await;
        security_events.push(event);

        // Keep only the last 1000 events
        if security_events.len() > 1000 {
            let len = security_events.len();
            if len > 1000 {
                security_events.drain(0..len - 1000);
            }
        }

        Ok(())
    }

    /// Get security events
    pub async fn get_security_events(&self) -> Result<Vec<SecurityEvent>> {
        let security_events = self.security_events.read().await;
        Ok(security_events.clone())
    }

    /// Get security events by type
    pub async fn get_security_events_by_type(&self, event_type: SecurityEventType) -> Result<Vec<SecurityEvent>> {
        let security_events = self.security_events.read().await;
        Ok(security_events.iter()
            .filter(|e| e.event_type == event_type)
            .cloned()
            .collect())
    }

    /// Get security events by severity
    pub async fn get_security_events_by_severity(&self, severity: SecuritySeverity) -> Result<Vec<SecurityEvent>> {
        let security_events = self.security_events.read().await;
        Ok(security_events.iter()
            .filter(|e| e.severity == severity)
            .cloned()
            .collect())
    }

    /// Get security events by IP
    pub async fn get_security_events_by_ip(&self, ip: &str) -> Result<Vec<SecurityEvent>> {
        let security_events = self.security_events.read().await;
        Ok(security_events.iter()
            .filter(|e| e.ip_address == ip)
            .cloned()
            .collect())
    }

    /// Get blacklisted IPs
    pub async fn get_blacklisted_ips(&self) -> Result<Vec<String>> {
        let blacklist = self.ip_blacklist.read().await;
        Ok(blacklist.keys().cloned().collect())
    }

    /// Get whitelisted IPs
    pub async fn get_whitelisted_ips(&self) -> Result<Vec<String>> {
        let whitelist = self.ip_whitelist.read().await;
        Ok(whitelist.keys().cloned().collect())
    }

    /// Get suspicious IPs
    pub async fn get_suspicious_ips(&self) -> Result<Vec<(String, u32)>> {
        let suspicious_ips = self.suspicious_ips.read().await;
        Ok(suspicious_ips.iter().map(|(k, v)| (k.clone(), *v)).collect())
    }

    /// Clear security events
    pub async fn clear_security_events(&self) -> Result<()> {
        let mut security_events = self.security_events.write().await;
        security_events.clear();
        Ok(())
    }

    /// Clear suspicious IPs
    pub async fn clear_suspicious_ips(&self) -> Result<()> {
        let mut suspicious_ips = self.suspicious_ips.write().await;
        suspicious_ips.clear();
        Ok(())
    }

    /// Get security configuration
    pub fn get_config(&self) -> &SecurityConfig {
        &self.config
    }

    /// Check if security is enabled
    pub fn is_enabled(&self) -> bool {
        self.config.ip_blacklist.enabled || self.config.ip_whitelist.enabled
    }
}
