//! Security hardening measures
//!
//! Provides security utilities and protections for the VEGA core.

use std::time::{Duration, Instant};

/// Rate limiter for preventing abuse.
pub struct RateLimiter {
    max_requests: usize,
    window: Duration,
    requests: Vec<Instant>,
}

impl RateLimiter {
    /// Creates a new rate limiter.
    pub fn new(max_requests: usize, window: Duration) -> Self {
        Self {
            max_requests,
            window,
            requests: Vec::new(),
        }
    }

    /// Checks if a request is allowed.
    pub fn check(&mut self) -> bool {
        let now = Instant::now();
        self.requests.retain(|&t| now.duration_since(t) < self.window);

        if self.requests.len() < self.max_requests {
            self.requests.push(now);
            true
        } else {
            false
        }
    }

    /// Resets the rate limiter.
    pub fn reset(&mut self) {
        self.requests.clear();
    }
}

/// Input sanitizer for strings.
pub fn sanitize_string(input: &str, max_length: usize) -> String {
    let mut sanitized = String::new();
    let mut char_count = 0;

    for c in input.chars() {
        if char_count >= max_length {
            break;
        }

        // Filter control characters except newline and tab
        if c.is_control() && c != '\n' && c != '\t' {
            continue;
        }

        sanitized.push(c);
        char_count += 1;
    }

    sanitized
}

/// Validates that a file path is safe (no directory traversal).
pub fn is_safe_path(path: &str) -> bool {
    // Check for directory traversal patterns
    if path.contains("..") || path.contains("\\..") || path.contains("/..") {
        return false;
    }

    // Check for absolute paths (optional, depending on requirements)
    if path.starts_with('/') || path.starts_with("\\") {
        return false;
    }

    true
}

/// Bounds checker for integer operations.
pub trait BoundsChecker {
    /// Checks if adding would overflow.
    fn checked_add_assign(&mut self, rhs: Self) -> bool;
    /// Checks if subtracting would underflow.
    fn checked_sub_assign(&mut self, rhs: Self) -> bool;
}

impl BoundsChecker for usize {
    fn checked_add_assign(&mut self, rhs: Self) -> bool {
        match self.checked_add(rhs) {
            Some(result) => {
                *self = result;
                true
            }
            None => false,
        }
    }

    fn checked_sub_assign(&mut self, rhs: Self) -> bool {
        match self.checked_sub(rhs) {
            Some(result) => {
                *self = result;
                true
            }
            None => false,
        }
    }
}

impl BoundsChecker for u32 {
    fn checked_add_assign(&mut self, rhs: Self) -> bool {
        match self.checked_add(rhs) {
            Some(result) => {
                *self = result;
                true
            }
            None => false,
        }
    }

    fn checked_sub_assign(&mut self, rhs: Self) -> bool {
        match self.checked_sub(rhs) {
            Some(result) => {
                *self = result;
                true
            }
            None => false,
        }
    }
}

/// Timeout handler for long-running operations.
pub struct TimeoutHandler {
    timeout: Duration,
    start: Instant,
}

impl TimeoutHandler {
    /// Creates a new timeout handler.
    pub fn new(timeout: Duration) -> Self {
        Self {
            timeout,
            start: Instant::now(),
        }
    }

    /// Checks if the timeout has been exceeded.
    pub fn has_timed_out(&self) -> bool {
        self.start.elapsed() > self.timeout
    }

    /// Returns the remaining time.
    pub fn remaining(&self) -> Duration {
        self.timeout.saturating_sub(self.start.elapsed())
    }

    /// Resets the timer.
    pub fn reset(&mut self) {
        self.start = Instant::now();
    }
}

/// Validates that a byte slice is within size limits.
pub fn validate_size(data: &[u8], max_size: usize) -> bool {
    data.len() <= max_size
}

/// Safe string truncation.
pub fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        s.chars().take(max_len).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limiter() {
        let mut limiter = RateLimiter::new(3, Duration::from_secs(1));

        assert!(limiter.check());
        assert!(limiter.check());
        assert!(limiter.check());
        assert!(!limiter.check()); // Exceeds limit
    }

    #[test]
    fn test_sanitize_string() {
        let input = "Hello\x00World";
        let sanitized = sanitize_string(input, 100);
        assert!(!sanitized.contains('\x00'));
    }

    #[test]
    fn test_is_safe_path() {
        assert!(is_safe_path("file.txt"));
        assert!(!is_safe_path("../etc/passwd"));
        assert!(!is_safe_path("C:\\Windows\\System32"));
    }

    #[test]
    fn test_bounds_checker() {
        let mut value: usize = 100;
        assert!(value.checked_add_assign(50));
        assert_eq!(value, 150);

        assert!(!value.checked_add_assign(usize::MAX));
    }

    #[test]
    fn test_timeout_handler() {
        let handler = TimeoutHandler::new(Duration::from_millis(100));
        assert!(!handler.has_timed_out());
        std::thread::sleep(Duration::from_millis(150));
        assert!(handler.has_timed_out());
    }

    #[test]
    fn test_validate_size() {
        assert!(validate_size(&[0u8; 100], 1000));
        assert!(!validate_size(&[0u8; 1000], 100));
    }

    #[test]
    fn test_truncate_string() {
        let s = "Hello World";
        assert_eq!(truncate_string(s, 5), "Hello");
        assert_eq!(truncate_string(s, 20), "Hello World");
    }
}
