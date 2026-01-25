use std::fmt::Display;
use crate::types::{Severity, ScanError};
use crate::traits::SecurityObject;

pub struct IncidentReport<'a, T> {
    pub timestamp: u64,
    pub original_data: &'a T,
    pub severity: Severity,
    pub recommendation: String,
}

impl<'a, T: Display> Display for IncidentReport<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ALERT [{:?}] | Source: {} | Action: {}",
            self.severity, self.original_data, self.recommendation
        )
    }
}

pub struct ThreatEngine<T> {
    data_source: Vec<T>,
}

impl<T> ThreatEngine<T> {
    pub fn load(data: Vec<T>) -> Self {
        Self { data_source: data }
    }
}

impl<T> ThreatEngine<T>
where
    T: SecurityObject + Display,
{
    pub fn analyze<'a>(&'a self) -> Result<Vec<IncidentReport<'a, T>>, ScanError> {
        if self.data_source.is_empty() {
            return Err(ScanError::EmptySource);
        }

        let mut incidents = Vec::new();

        for item in &self.data_source {
            let severity = item.calculate_severity();

            // Filter only High and Critical issues
            if severity >= Severity::High {
                let action = match severity {
                    Severity::Critical => "IMMEDIATE ISOLATION REQUIRED",
                    Severity::High => "Investigate IP and User",
                    _ => "Monitor",
                };

                incidents.push(IncidentReport {
                    timestamp: 1700000000,
                    original_data: item,
                    severity,
                    recommendation: action.to_string(),
                });
            }
        }

        Ok(incidents)
    }
}