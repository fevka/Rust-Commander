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
use tauri::Manager;

// DÜZELTME: Thread hatası için String dönüşümü yapıldı
fn play_sound(file_name: &str) {
    // Referansı "Sahipli" (Owned) bir String'e çeviriyoruz
    let path = file_name.to_string();
    
    std::thread::spawn(move || {
        // Artık 'path' bu thread'in malı oldu, güvenle kullanabilir
        if let Ok(file) = File::open(&path) {
            // Ses sistemi başlat
            let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
            let sink = rodio::Sink::try_new(&stream_handle).unwrap();
            
            let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
            sink.append(source);
            sink.sleep_until_end();
        } else {
            println!("SES DOSYASI BULUNAMADI: {}", path);
        }
    });
}

#[tauri::command]
fn start_bot(settings: serde_json::Value) {
    println!("AYARLAR ALINDI: {:?}", settings);
}

fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            // ARKA PLAN GÖREVİ
            std::thread::spawn(|| {
                // Tokio Runtime'ı manuel başlatıyoruz
                let rt = tokio::runtime::Runtime::new().expect("Tokio Runtime başlatılamadı");
                
                rt.block_on(async {
                    println!("--- BOT SİSTEMİ DEVREDE ---");
                    
                    let config = GameConfig::new();
                    let ocr_engine = HonOcr::new();
                    let (hp_x, hp_y, hp_w, hp_h) = config.get_my_hp_rect();
                    
                    let start_time = Instant::now();
                    let mut last_rune_check = 0;
                    let mut last_stack_check = 0;

                    let mut interval = tokio::time::interval(Duration::from_millis(150));

                    loop {
                        interval.tick().await;
                        
                        let elapsed_secs = start_time.elapsed().as_secs();
                        
                        // RUNE KONTROL (Her 2 dakikada bir, 15sn kala)
                        if elapsed_secs > 0 && (elapsed_secs + 15) % 120 == 0 && elapsed_secs != last_rune_check {
                            println!("UYARI: RUNE ZAMANI!");
                            play_sound("Rune.wav");
                            last_rune_check = elapsed_secs;
                        }

                        // STACK KONTROL (Her 1 dakikada bir, 13sn kala)
                        if elapsed_secs > 0 && (elapsed_secs + 13) % 60 == 0 && elapsed_secs != last_stack_check {
                            println!("UYARI: STACK ZAMANI!");
                            play_sound("Stack.wav");
                            last_stack_check = elapsed_secs;
                        }

                        // OCR Taraması
                        let pixels = ScreenCapture::capture(hp_x, hp_y, hp_w, hp_h);
                        let text = ocr_engine.read_text(hp_w, hp_h, &pixels).await;
                        let clean_text = text.trim().to_uppercase();

                        if !clean_text.is_empty() {
                            // Basit bir "/" kontrolü (Örn: 500/1000)
                            if clean_text.contains('/') {
                                let parts: Vec<&str> = clean_text.split('/').collect();
                                if parts.len() == 2 {
                                    if let (Ok(current_hp), Ok(_)) = (parts[0].trim().parse::<i32>(), parts[1].trim().parse::<i32>()) {
                                        
                                        // 300 Can altı = EXECUTE
                                        if current_hp < 300 && current_hp > 0 { 
                                            println!("--- EXECUTE FIRSATI! ({}) ---", current_hp);
                                            play_sound("execute.wav");
                                            // Spam engelleme
                                            tokio::time::sleep(Duration::from_millis(1000)).await;
                                        }
                                    }
                                }
                            }
                        }
                    }
                });
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![start_bot])
        .run(tauri::generate_context!())
        .expect("Uygulama hatası");
}