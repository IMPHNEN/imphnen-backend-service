use axum::http::HeaderMap;

/// Extract real client IP address from various headers commonly used in proxies
/// 
/// Priority order:
/// 1. X-Forwarded-For (first IP in the list)
/// 2. X-Real-IP
/// 3. CF-Connecting-IP (Cloudflare)
/// 4. True-Client-IP (Akamai and others)
/// 5. X-Cluster-Client-IP
/// 6. Forwarded (standard header)
/// 7. Direct connection IP (if available)
pub fn extract_real_ip(headers: &HeaderMap) -> Option<String> {
    // Try different headers in priority order
    if let Some(ip) = extract_from_x_forwarded_for(headers) {
        return Some(ip);
    }
    
    if let Some(ip) = extract_header_value(headers, "x-real-ip") {
        return Some(ip);
    }
    
    if let Some(ip) = extract_header_value(headers, "cf-connecting-ip") {
        return Some(ip);
    }
    
    if let Some(ip) = extract_header_value(headers, "true-client-ip") {
        return Some(ip);
    }
    
    if let Some(ip) = extract_header_value(headers, "x-cluster-client-ip") {
        return Some(ip);
    }
    
    if let Some(ip) = extract_from_forwarded_header(headers) {
        return Some(ip);
    }
    
    None
}

/// Extract the first IP from X-Forwarded-For header
fn extract_from_x_forwarded_for(headers: &HeaderMap) -> Option<String> {
    let header_value = headers.get("x-forwarded-for")?;
    let header_str = header_value.to_str().ok()?;
    
    // X-Forwarded-For can contain multiple IPs separated by commas
    // We take the first one (the original client IP)
    header_str.split(',').next()
        .map(|ip| ip.trim().to_string())
        .filter(|ip| is_valid_ip(ip))
}

/// Extract IP from Forwarded header (RFC 7239)
fn extract_from_forwarded_header(headers: &HeaderMap) -> Option<String> {
    let header_value = headers.get("forwarded")?;
    let header_str = header_value.to_str().ok()?;
    
    // Parse Forwarded header: for=192.0.2.60;proto=http;by=203.0.113.43
    for part in header_str.split(';') {
        if part.trim().starts_with("for=") {
            let ip = part.trim().trim_start_matches("for=");
            // Remove quotes and brackets if present
            let ip = ip.trim_matches('"').trim_matches('[').trim_matches(']');
            if is_valid_ip(ip) {
                return Some(ip.to_string());
            }
        }
    }
    
    None
}

/// Extract value from a specific header
fn extract_header_value(headers: &HeaderMap, header_name: &str) -> Option<String> {
    let header_value = headers.get(header_name)?;
    let value_str = header_value.to_str().ok()?;
    
    if is_valid_ip(value_str) {
        Some(value_str.to_string())
    } else {
        None
    }
}

/// Basic IP validation
fn is_valid_ip(ip: &str) -> bool {
    // Simple validation - check if it looks like an IP address
    if ip.is_empty() || ip == "unknown" || ip == "undefined" {
        return false;
    }
    
    // Check for IPv4 pattern
    if ip.split('.').count() == 4 && ip.chars().all(|c| c.is_ascii_digit() || c == '.') {
        return true;
    }
    
    // Check for IPv6 pattern (simplified)
    if ip.contains(':') {
        return true;
    }
    
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderValue;

    #[test]
    fn test_extract_from_x_forwarded_for() {
        let mut headers = HeaderMap::new();
        headers.insert("x-forwarded-for", HeaderValue::from_static("192.168.1.1, 10.0.0.1"));
        
        assert_eq!(extract_from_x_forwarded_for(&headers), Some("192.168.1.1".to_string()));
    }

    #[test]
    fn test_extract_from_forwarded_header() {
        let mut headers = HeaderMap::new();
        headers.insert("forwarded", HeaderValue::from_static("for=192.168.1.1;proto=https"));
        
        assert_eq!(extract_from_forwarded_header(&headers), Some("192.168.1.1".to_string()));
    }

    #[test]
    fn test_extract_real_ip_priority() {
        let mut headers = HeaderMap::new();
        headers.insert("x-forwarded-for", HeaderValue::from_static("192.168.1.1"));
        headers.insert("x-real-ip", HeaderValue::from_static("10.0.0.1"));
        
        // Should prefer x-forwarded-for
        assert_eq!(extract_real_ip(&headers), Some("192.168.1.1".to_string()));
    }

    #[test]
    fn test_invalid_ip_rejection() {
        let mut headers = HeaderMap::new();
        headers.insert("x-forwarded-for", HeaderValue::from_static("unknown"));
        
        assert_eq!(extract_real_ip(&headers), None);
    }
}