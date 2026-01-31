use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

struct SecurityAlert {
    sensor_id: u32,
    target_ip: String,
    severity: Severity,
    message: String,
}

fn main() {
    let global_threat_counter = Arc::new(Mutex::new(0));
    let (tx, rx) = mpsc::channel();
    let mut workers = Vec::new();

    for id in 1..=5 {
        let counter_ref = Arc::clone(&global_threat_counter);
        let sender_ref = tx.clone();

        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(id as u64 * 200));

            {
                let mut num = counter_ref.lock().unwrap();
                *num += 1;
            }

            let alert = SecurityAlert {
                sensor_id: id,
                target_ip: format!("10.0.50.{}", 100 + id),
                severity: if id % 2 == 0 { Severity::Critical } else { Severity::High },
                message: String::from("Suspicious packet signature detected"),
            };

            sender_ref.send(alert).unwrap();
        });

        workers.push(handle);
    }

    drop(tx);

    println!("--- IDS DASHBOARD INITIALIZED ---");

    for alert in rx {
        println!(
            "[ALERT] Sensor #{} | IP: {} | Level: {:?} | Msg: {}",
            alert.sensor_id, alert.target_ip, alert.severity, alert.message
        );
    }

    for handle in workers {
        handle.join().unwrap();
    }

    let final_count = global_threat_counter.lock().unwrap();
    println!("--- SYSTEM REPORT ---");
    println!("Total Threats Neutralized: {}", *final_count);
}