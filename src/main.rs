#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod config;
mod ocr;
mod screen;

use config::GameConfig;
use screen::ScreenCapture;
use ocr::HonOcr;
use std::time::{Duration, Instant};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use tauri::State;

// --- GLOBAL AYARLAR ---
struct AppState {
    config: Mutex<BotConfig>,
}

#[derive(Clone, Debug)]
struct BotConfig {
    running: bool,
    start_time: Option<Instant>,
    
    // Boss Hedef Zamanları (Oyun saniyesi cinsinden)
    next_kongor_time: u64,
    next_phoenix_time: u64,
}

impl Default for BotConfig {
    fn default() -> Self {
        Self {
            running: false,
            start_time: None,
            // İlk çıkış zamanları (Saniye)
            next_kongor_time: 600,  // 10 dakika (600 sn)
            next_phoenix_time: 900, // 15 dakika (900 sn)
        }
    }
}

fn play_sound(file_name: &str) {
    let path = file_name.to_string();
    std::thread::spawn(move || {
        if let Ok(file) = File::open(&path) {
            let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
            let sink = rodio::Sink::try_new(&stream_handle).unwrap();
            let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
            sink.append(source);
            sink.sleep_until_end();
        }
    });
}

// --- KOMUTLAR ---

#[tauri::command]
fn start_bot(state: State<AppState>) {
    println!(">>> OYUN BAŞLADI (SAYAÇ AKTİF - 00:00)");
    let mut config = state.config.lock().unwrap();
    config.running = true;
    
    // Oyunun 00:00 anında bu tuşa basıldığı varsayılır
    config.start_time = Some(Instant::now()); 
    
    // Sayaçları varsayılana döndür
    config.next_kongor_time = 600; 
    config.next_phoenix_time = 900; 
    
    play_sound("execute.wav");
}

#[tauri::command]
fn boss_dead(boss: String, state: State<AppState>) {
    let mut config = state.config.lock().unwrap();
    
    if let Some(start) = config.start_time {
        let current_game_time = start.elapsed().as_secs();
        
        if boss == "Kongor" {
            // Kongor öldü: Şu anki süre + 6 dakika (360 sn)
            config.next_kongor_time = current_game_time + 360;
            println!("KONGOR ÖLDÜ! Yeni doğuş: {} saniye sonra (Oyun saati: {})", 360, current_game_time + 360);
        } else if boss == "Phoenix" {
            // Phoenix öldü: Şu anki süre + 8 dakika (480 sn)
            config.next_phoenix_time = current_game_time + 480;
            println!("PHOENIX ÖLDÜ! Yeni doğuş: {} saniye sonra (Oyun saati: {})", 480, current_game_time + 480);
        }
    }
}

fn main() {
    let app_state = Arc::new(AppState {
        config: Mutex::new(BotConfig::default()),
    });

    let thread_state = app_state.clone();

    // --- BOT THREAD ---
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().expect("Runtime Hatasi");
        rt.block_on(async {
            println!("--- BOT HAZIR ---");
            
            let game_config = GameConfig::new();
            let ocr_engine = HonOcr::new();
            let (hp_x, hp_y, hp_w, hp_h) = game_config.get_my_hp_rect();
            
            let mut interval = tokio::time::interval(Duration::from_millis(200));
            
            // Tekrarı önlemek için hafıza değişkenleri
            let mut last_minute_check = 999; 
            let mut last_rune_check = 999;
            let mut last_kongor_warn = 0;
            let mut last_phoenix_warn = 0;

            loop {
                interval.tick().await;

                // Config verilerini kopyala
                let (is_running, start_time, next_kongor, next_phoenix) = {
                    let c = thread_state.config.lock().unwrap();
                    (c.running, c.start_time, c.next_kongor_time, c.next_phoenix_time)
                };

                if !is_running || start_time.is_none() {
                    continue;
                }

                // Geçen Oyun Süresi (Saniye)
                let elapsed = start_time.unwrap().elapsed().as_secs();
                let current_minute = elapsed / 60;
                
                // ---------------------------------------------------------
                // 1. STACK UYARISI (Her dakika x:50'de uyar)
                // Amaç: x:55'te çekmek için 5 saniye hazırlık.
                // ---------------------------------------------------------
                if elapsed > 10 && (elapsed % 60) == 50 {
                    if current_minute != last_minute_check {
                        println!(">> [STACK] HAZIRLAN! (x:50)");
                        play_sound("Stack.wav");
                        last_minute_check = current_minute;
                    }
                }

                // ---------------------------------------------------------
                // 2. RUNE UYARISI (Her 2 dakikada bir, 15sn kala)
                // Örn: 1:45, 3:45, 5:45...
                // ---------------------------------------------------------
                if elapsed > 10 && (elapsed + 15) % 120 == 0 {
                    // Sadece o 1 saniye içinde bir kez çalması için basit kontrol:
                    let check_val = elapsed / 10; // Kabaca o zaman dilimi
                    if check_val != last_rune_check {
                        println!(">> [RUNE] KONTROL ZAMANI!");
                        play_sound("Rune.wav");
                        // Rune sesi uzunsa Stack sesiyle karışmasın diye kısa bekleme
                        tokio::time::sleep(Duration::from_secs(2)).await;
                        last_rune_check = check_val;
                    }
                }

                // ---------------------------------------------------------
                // 3. KONGOR KONTROL (Doğmasına 30 saniye kala)
                // ---------------------------------------------------------
                if next_kongor > elapsed && (next_kongor - elapsed) <= 30 && (next_kongor - elapsed) > 20 {
                    if next_kongor != last_kongor_warn {
                        println!("!!! KONGOR DOĞUYOR (30sn) !!!");
                        play_sound("execute.wav"); 
                        last_kongor_warn = next_kongor;
                    }
                }

                // ---------------------------------------------------------
                // 4. PHOENIX KONTROL (Doğmasına 30 saniye kala)
                // ---------------------------------------------------------
                if next_phoenix > elapsed && (next_phoenix - elapsed) <= 30 && (next_phoenix - elapsed) > 20 {
                    if next_phoenix != last_phoenix_warn {
                        println!("!!! PHOENIX DOĞUYOR (30sn) !!!");
                        play_sound("execute.wav");
                        last_phoenix_warn = next_phoenix;
                    }
                }

                // ---------------------------------------------------------
                // 5. HP & EXECUTE KONTROLÜ (Sürekli)
                // ---------------------------------------------------------
                let pixels = ScreenCapture::capture(hp_x, hp_y, hp_w, hp_h);
                let text = ocr_engine.read_text(hp_w, hp_h, &pixels).await;
                let clean_text = text.trim();

                if !clean_text.is_empty() {
                    if clean_text.contains('/') {
                        let parts: Vec<&str> = clean_text.split('/').collect();
                        if parts.len() == 2 {
                            if let (Ok(curr), Ok(_)) = (parts[0].trim().parse::<i32>(), parts[1].trim().parse::<i32>()) {
                                // 350 Can altı = EXECUTE
                                if curr < 350 && curr > 0 { 
                                    println!("--- EXECUTE FIRSATI! ({}) ---", curr);
                                    play_sound("execute.wav");
                                    tokio::time::sleep(Duration::from_millis(1500)).await;
                                }
                            }
                        }
                    }
                }
            }
        });
    });

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![start_bot, boss_dead])
        .run(tauri::generate_context!())
        .expect("Uygulama hatası");
}