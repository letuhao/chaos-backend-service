use axum::{
    extract::ConnectInfo,
    http::HeaderMap,
};
use std::net::SocketAddr;

/// Extract IP address from request headers or connection info
pub fn extract_ip_address(headers: &HeaderMap, connect_info: Option<ConnectInfo<SocketAddr>>) -> Option<String> {
    // Try to get IP from X-Forwarded-For header first (for load balancers/proxies)
    if let Some(forwarded_for) = headers.get("x-forwarded-for") {
        if let Ok(forwarded_str) = forwarded_for.to_str() {
            // X-Forwarded-For can contain multiple IPs, take the first one
            if let Some(first_ip) = forwarded_str.split(',').next() {
                return Some(first_ip.trim().to_string());
            }
        }
    }

    // Try X-Real-IP header
    if let Some(real_ip) = headers.get("x-real-ip") {
        if let Ok(real_ip_str) = real_ip.to_str() {
            return Some(real_ip_str.to_string());
        }
    }

    // Fall back to connection info
    if let Some(ConnectInfo(addr)) = connect_info {
        return Some(addr.ip().to_string());
    }

    None
}

/// Extract User-Agent from request headers
pub fn extract_user_agent(headers: &HeaderMap) -> Option<String> {
    if let Some(user_agent) = headers.get("user-agent") {
        if let Ok(ua_str) = user_agent.to_str() {
            return Some(ua_str.to_string());
        }
    }
    None
}

/// Extract client information from request
pub struct ClientInfo {
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

impl ClientInfo {
    pub fn from_request(headers: &HeaderMap, connect_info: Option<ConnectInfo<SocketAddr>>) -> Self {
        Self {
            ip_address: extract_ip_address(headers, connect_info),
            user_agent: extract_user_agent(headers),
        }
    }
}
