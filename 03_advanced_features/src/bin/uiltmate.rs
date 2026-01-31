use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

const SERVER_ADDR: &str = "127.0.0.1:8080";

struct LootDatabase {
    stolen_creds: Vec<String>,
}

#[derive(Debug)]
enum LogLevel {
    Info,
    Alert,
    Error,
}

struct SystemLog {
    level: LogLevel,
    message: String,
}

fn main() {
    let loot_db = Arc::new(Mutex::new(LootDatabase {
        stolen_creds: Vec::new(),
    }));

    let (log_tx, log_rx) = mpsc::channel::<SystemLog>();

    let logger_handle = thread::spawn(move || {
        println!("--- [C2 SERVER ONLINE] ---");
        for log in log_rx {
            match log.level {
                LogLevel::Info => println!("[INFO]  {}", log.message),
                LogLevel::Alert => println!("[ALERT] {}", log.message),
                LogLevel::Error => println!("[ERROR] {}", log.message),
            }
        }
    });

    let db_clone = Arc::clone(&loot_db);
    let log_tx_clone = log_tx.clone();

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        simulate_bot_attack(log_tx_clone);
    });

    let listener = TcpListener::bind(SERVER_ADDR).expect("Could not bind port");
    let _ = log_tx.send(SystemLog {
        level: LogLevel::Info,
        message: format!("Listening on {}...", SERVER_ADDR),
    });

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let db_ref = Arc::clone(&loot_db);
                let tx_ref = log_tx.clone();

                thread::spawn(move || {
                    handle_bot_connection(stream, db_ref, tx_ref);
                });

            }
            Err(_) => {
                let _ = log_tx.send(SystemLog {
                    level: LogLevel::Error,
                    message: String::from("Connection failed"),
                });
            }
        }

        // Breaking loop for demonstration after catching the simulation
        if loot_db.lock().unwrap().stolen_creds.len() >= 3 {
            break;
        }
    }

    // Wait for logger to finish printing pending messages
    thread::sleep(Duration::from_secs(1));

    println!("\n--- [DUMPING LOOT DATABASE] ---");
    let final_db = loot_db.lock().unwrap();
    for (i, cred) in final_db.stolen_creds.iter().enumerate() {
        println!("Entry #{}: {}", i + 1, cred);
    }
}

fn handle_bot_connection(mut stream: TcpStream, db: Arc<Mutex<LootDatabase>>, tx: mpsc::Sender<SystemLog>) {
    let mut buffer = [0; 512];

    let _ = tx.send(SystemLog {
        level: LogLevel::Info,
        message: String::from("New Bot Connected"),
    });

    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            if bytes_read > 0 {
                let data = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
                let clean_data = data.trim().to_string();

                {
                    let mut safe_db = db.lock().unwrap();
                    safe_db.stolen_creds.push(clean_data.clone());
                }

                let _ = tx.send(SystemLog {
                    level: LogLevel::Alert,
                    message: format!("Data Exfiltrated: {}", clean_data),
                });

                let _ = stream.write(b"ACK_RECEIVED");
            }
        }
        Err(_) => {
            let _ = tx.send(SystemLog {
                level: LogLevel::Error,
                message: String::from("Failed to read from bot"),
            });
        }
    }
}

fn simulate_bot_attack(tx: mpsc::Sender<SystemLog>) {
    let payloads = vec![
        "user:admin|pass:123456",
        "cookie:session_id=xyz987",
        "ssh_key:PRIVATE_KEY_DATA...",
    ];

    for payload in payloads {
        thread::sleep(Duration::from_millis(500));
        match TcpStream::connect(SERVER_ADDR) {
            Ok(mut stream) => {
                let _ = stream.write(payload.as_bytes());
            }
            Err(_) => {
                let _ = tx.send(SystemLog {
                    level: LogLevel::Error,
                    message: String::from("Simulation failed to connect"),
                });
            }
        }
    }
}