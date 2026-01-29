use cyber_watch::alerts::{self, AlertLevel}; 
use cyber_watch::systems;
use cyber_watch::LoginAttempt; 

fn main() {
    println!("--- {} ---", systems::get_system_status());

    let attempts = vec![
        LoginAttempt::new("user1", "192.168.1.10", true),
        LoginAttempt::new("guest", "192.168.1.15", false),
        LoginAttempt::new("admin", "10.0.0.66", false), 
    ];

    for attempt in attempts {
        let is_suspicious = systems::auth::analyze_attempt(&attempt);

        if is_suspicious {
            let msg = format!("Suspicious activity detected from IP: {}", attempt.ip);
            alerts::send_alert(&msg, AlertLevel::Critical);
            systems::firewall::stop_ip(&attempt.ip);
        } else {
            if attempt.success {
                alerts::send_alert("User logged in successfully", AlertLevel::Info);
            } else {
                alerts::send_alert("Failed login attempt", AlertLevel::Warning);
            }
        }
    }
}