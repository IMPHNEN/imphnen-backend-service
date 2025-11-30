//! Input sanitization utilities for security
//!
//! This module provides utilities to sanitize user input and prevent
//! common security vulnerabilities like XSS, HTML injection, SQL injection, etc.
//! Specifically optimized for PostgreSQL backend (SurrealDB migration complete).

use regex::Regex;
use std::sync::LazyLock;

// Note: HTML escaping is done via char-by-char mapping for better performance
// No regex needed for basic HTML entity escaping

/// PostgreSQL-specific SQL injection patterns
///
/// Comprehensive pattern set targeting PostgreSQL vulnerabilities while maintaining
/// compatibility with standard SQL injection prevention
static SQL_INJECTION_PATTERNS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(union|select|insert|update|delete|drop|create|alter|truncate|vacuum|analyze|reindex|cluster|copy|exec|script|javascript|onerror|onload|with|from|where|join|group by|order by|limit|offset|having|distinct|into|values|union all|union distinct|::|%|:=|current_user|session_user|user|version|current_date|current_time|now|pg_sleep|pg_user|pg_database|pg_tables|pg_columns|chr|ascii|substring|position|strpos|concat|concat_ws|string_agg|array_agg|array_to_string|string_to_array)").unwrap()
});

/// Path traversal patterns
static PATH_TRAVERSAL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\.\.(/|\\)").unwrap()
});

/// Sanitize HTML by escaping special characters
///
/// # Example
/// ```rust
/// use imphnen_utils::sanitize_html;
///
/// let dirty = "<script>alert('xss')</script>";
/// let clean = sanitize_html(dirty);
/// assert_eq!(clean, "&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;");
/// ```
pub fn sanitize_html(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            '<' => "&lt;".to_string(),
            '>' => "&gt;".to_string(),
            '"' => "&quot;".to_string(),
            '\'' => "&#39;".to_string(),
            '&' => "&amp;".to_string(),
            _ => c.to_string(),
        })
        .collect()
}

/// Sanitize string to prevent SQL injection and other dangerous patterns
///
/// PostgreSQL-optimized sanitization that removes potentially dangerous patterns
/// while preserving legitimate user input where possible
pub fn sanitize_dangerous_patterns(input: &str) -> String {
    // First pass: Remove SQL injection patterns
    let without_sql_injection = SQL_INJECTION_PATTERNS.replace_all(input, "[FILTERED]");
    
    // Second pass: Additional PostgreSQL-specific protection
    let without_postgres_specific = without_sql_injection.replace(";--", ";[FILTERED]");
    
    without_postgres_specific.to_owned()
}

/// Check if string contains path traversal attempts
pub fn contains_path_traversal(input: &str) -> bool {
    PATH_TRAVERSAL_REGEX.is_match(input)
}

/// Sanitize a string for safe usage in file names
///
/// Removes or replaces characters that could cause issues in file systems
pub fn sanitize_filename(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            c if c.is_control() => '_',
            c => c,
        })
        .collect()
}

/// Sanitize user input text (removes HTML and dangerous patterns)
///
/// Use this for fields like names, descriptions, bios, etc.
pub fn sanitize_user_text(input: &str) -> String {
    let without_html = sanitize_html(input);
    sanitize_dangerous_patterns(&without_html)
}

/// Trim and normalize whitespace in a string
pub fn normalize_whitespace(input: &str) -> String {
    input
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string()
}

/// Validate and sanitize email format
pub fn sanitize_email(email: &str) -> Option<String> {
    let trimmed = email.trim().to_lowercase();
    
    // Basic email validation
    if trimmed.contains('@') && trimmed.contains('.') {
        Some(trimmed)
    } else {
        None
    }
}

/// Sanitize URL to prevent javascript: and data: schemes
pub fn sanitize_url(url: &str) -> Option<String> {
    let trimmed = url.trim();
    
    // Block dangerous URL schemes
    let lower = trimmed.to_lowercase();
    if lower.starts_with("javascript:") || lower.starts_with("data:") || lower.starts_with("vbscript:") {
        return None;
    }
    
    // Allow http, https, and relative URLs
    if lower.starts_with("http://") || lower.starts_with("https://") || lower.starts_with("/") {
        Some(trimmed.to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_html() {
        assert_eq!(
            sanitize_html("<script>alert('xss')</script>"),
            "&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;"
        );
        assert_eq!(
            sanitize_html("Normal text"),
            "Normal text"
        );
    }

    #[test]
    fn test_sanitize_dangerous_patterns() {
        // Test basic SQL injection
        assert!(sanitize_dangerous_patterns("SELECT * FROM users").contains("[FILTERED]"));
        
        // Test PostgreSQL-specific patterns
        assert!(sanitize_dangerous_patterns("SELECT current_user;").contains("[FILTERED]"));
        assert!(sanitize_dangerous_patterns("SELECT version();").contains("[FILTERED]"));
        assert!(sanitize_dangerous_patterns("SELECT 'a'::text;").contains("[FILTERED]"));
        assert!(sanitize_dangerous_patterns("SELECT 'a'%'b';").contains("[FILTERED]"));
        
        // Test comment injection
        assert!(sanitize_dangerous_patterns("'; DROP TABLE users; --").contains("[FILTERED]"));
        
        // Test legitimate input remains unchanged
        assert_eq!(
            sanitize_dangerous_patterns("Normal search query using 'quotes' and ; semicolons"),
            "Normal search query using 'quotes' and ; semicolons"
        );
        
        // Test PostgreSQL function filtering
        assert!(sanitize_dangerous_patterns("SELECT pg_sleep(10);").contains("[FILTERED]"));
        assert!(sanitize_dangerous_patterns("SELECT concat('a', 'b');").contains("[FILTERED]"));
    }

    #[test]
    fn test_path_traversal() {
        assert!(contains_path_traversal("../../../etc/passwd"));
        assert!(contains_path_traversal("..\\windows\\system32"));
        assert!(!contains_path_traversal("normal/path/to/file"));
    }

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(
            sanitize_filename("file<name>.txt"),
            "file_name_.txt"
        );
        assert_eq!(
            sanitize_filename("normal_file.pdf"),
            "normal_file.pdf"
        );
    }

    #[test]
    fn test_sanitize_url() {
        assert_eq!(
            sanitize_url("https://example.com"),
            Some("https://example.com".to_string())
        );
        assert_eq!(sanitize_url("javascript:alert('xss')"), None);
        assert_eq!(sanitize_url("data:text/html,<script>alert('xss')</script>"), None);
    }

    #[test]
    fn test_normalize_whitespace() {
        assert_eq!(
            normalize_whitespace("  multiple   spaces   "),
            "multiple spaces"
        );
    }
}
