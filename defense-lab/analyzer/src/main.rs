use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};

// Her bir IP için profil oluşturuyoruz
struct IpStats {
    count: u32,
    unique_agents: HashSet<String>,
}

#[tokio::main]
async fn main() {
    let traffic_map = Arc::new(Mutex::new(HashMap::<String, IpStats>::new()));
    let traffic_clone = Arc::clone(&traffic_map);
    println!("🕵️ ZEKİ ANALİZÖR AKTİF: Davranış Analizi Başladı...");
    // Analiz ve Raporlama Döngüsü
    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(2)).await;
            let mut data = traffic_clone.lock().unwrap();
            
            println!("\n--- [ TRAFİK DENETİMİ ] ---");
            for (ip, stats) in data.iter() {
                let agent_count = stats.unique_agents.len();
                
                // MANTIK: Eğer çok fazla istek gelmişse VE çok fazla farklı User-Agent varsa = BOT!
                if stats.count > 50 && agent_count > 2 {
                    println!("🚨 ALARM: {} IP adresinden BOT SALDIRISI tespit edildi!", ip);
                    println!("   - İstek Hızı: {} / 2sn", stats.count);
                    println!("   - Kimlik Sayısı: {} (Kılık değiştirme tespit edildi!)", agent_count);
                } else {
                    println!("🟢 IP: {} | Durum: Normal", ip);
                }
            }
            data.clear(); // Her 2 saniyede bir sıfırla ki anlık hızı ölçelim
        }
    });

    // Simülasyon: Gelen trafiği işleyen ana döngü
    loop {
        {
            let mut data = traffic_map.lock().unwrap();
            let entry = data.entry("127.0.0.1".to_string()).or_insert(IpStats {
                count: 0,
                unique_agents: HashSet::new(),
            });

            entry.count += 1;
            // Burası gerçekte ağ kartından gelen User-Agent'ı alacak
            entry.unique_agents.insert("Simüle Edilen UA".to_string());
        }
        sleep(Duration::from_millis(10)).await;
    }
}

// ... (Önceki analyzer importları)

async fn monitor_ui(count: u32, ip: &str) {
    let limit = 100;
    let (color, icon) = if count > limit {
        ("\x1b[31m", "🦈 SHARK ATTACK!") // Kırmızı ve Köpekbalığı
    } else {
        ("\x1b[32m", "🐬 SAFE WATER")   // Yeşil ve Yunus
    };

    let bar_length = (count / 10).min(30) as usize;
    let bar = "█".repeat(bar_length);
    let space = " ".repeat(30 - bar_length);

    println!("{}[{}{}] | RPS: {:<4} | IP: {:<12} | {}{}\x1b[0m", 
             color, bar, space, count, ip, icon, "\x1b[0m");
}