use crate::types::Severity;

pub trait SecurityObject {
    fn calculate_severity(&self) -> Severity;
    fn source_id(&self) -> String;
}

pub trait Reportable {
    fn generate_summary(&self) -> String;
}