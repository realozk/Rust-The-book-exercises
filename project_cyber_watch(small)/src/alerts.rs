use colored::*; 

pub enum AlertLevel {
    Info,
    Warning,
    Critical,
}

pub fn send_alert(msg: &str, level: AlertLevel) {
    match level {
        AlertLevel::Info => println!("[INFO] {}", msg.green()),
        AlertLevel::Warning => println!("[WARNING] {}", msg.yellow()),
        AlertLevel::Critical => println!("[CRITICAL]  {}", msg.red().bold()),
    }
}