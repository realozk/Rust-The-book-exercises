use std::fmt::Display;
use crate::types::Severity;
use crate::traits::{SecurityObject, Reportable};

// --- Network Model ---
pub struct FirewallLog {
    pub source_ip: String,
    pub dest_port: u16,
    pub payload_size: usize,
    pub is_blocked: bool,
}

impl FirewallLog {
    pub fn new(ip: &str, port: u16, size: usize, blocked: bool) -> Self {
        Self {
            source_ip: ip.to_string(),
            dest_port: port,
            payload_size: size,
            is_blocked: blocked,
        }
    }
}

impl SecurityObject for FirewallLog {
    fn calculate_severity(&self) -> Severity {
        if self.payload_size > 5000 && !self.is_blocked {
            Severity::Critical
        } else if self.dest_port == 22 || self.dest_port == 3389 {
            Severity::High
        } else {
            Severity::Info
        }
    }

    fn source_id(&self) -> String {
        format!("FW::{}", self.source_ip)
    }
}

impl Display for FirewallLog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Firewall] Src: {}, Port: {}", self.source_ip, self.dest_port)
    }
}

// --- Auth Model ---
pub struct AuthLog {
    pub user: String,
    pub failed_attempts: u8,
    pub location: String,
}

impl SecurityObject for AuthLog {
    fn calculate_severity(&self) -> Severity {
        if self.failed_attempts > 10 {
            Severity::Critical
        } else if self.failed_attempts > 3 {
            Severity::Medium
        } else {
            Severity::Low
        }
    }

    fn source_id(&self) -> String {
        format!("AUTH::{}", self.user)
    }
}

impl Display for AuthLog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Auth] User: {}, Failed: {}", self.user, self.failed_attempts)
    }
}