use tokio::net::TcpListener;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde_json::json;
use chrono::Utc;
use std::fs::OpenOptions;
use std::io::Write;
use std::net::SocketAddr;

const LISTEN_ADDR: &str = "0.0.0.0:8081";
const ATTACK_THRESHOLD: usize = 100;

type SharedMap = Arc<Mutex<HashMap<String, usize>>>;

#[tokio::main]
async fn main() {
    println!("Analyzer started on {}", LISTEN_ADDR);
    log_attack("127.0.0.1", 999);

    let listener = TcpListener::bind(LISTEN_ADDR)
        .await
        .expect("Failed to bind");

    let connections: SharedMap =
        Arc::new(Mutex::new(HashMap::new()));

    loop {
        match listener.accept().await {
            Ok((_, addr)) => {
                handle_connection(addr, &connections);
            }

            Err(e) => {
                eprintln!("Accept error: {}", e);
            }
        }
    }
}

fn handle_connection(addr: SocketAddr, connections: &SharedMap) {
    let ip = addr.ip().to_string();

    let mut map = connections.lock().unwrap();

    let count = map.entry(ip.clone()).or_insert(0);

    *count += 1;

    println!("{} -> {}", ip, count);

    if *count == ATTACK_THRESHOLD {
        println!("ATTACK DETECTED: {}", ip);

        log_attack(&ip, *count);
    }
}

fn log_attack(ip: &str, connections: usize) {
    let log_entry = json!({
        "ip": ip,
        "connections": connections,
        "threat": "HIGH",
        "timestamp": Utc::now().to_rfc3339()
    });

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("attack-log.json")
        .expect("Failed to open log file");

    writeln!(file, "{}", log_entry.to_string())
        .expect("Failed to write log");

    println!("Logged attack from {}", ip);
}
