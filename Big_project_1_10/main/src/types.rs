#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Severity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug)]
pub enum ScanError {
    EmptySource,
    DataSourceCorrupted(String),
}