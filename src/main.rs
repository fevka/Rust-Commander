mod config;
mod ocr;
mod screen;

use config::GameConfig;
use screen::ScreenCapture;
use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() {
    println!("--- HoN AI COMMANDER (RUST EDITION) BAŞLATILIYOR ---");

    // 1. Ayarları yükle
    let config = GameConfig::new();
    
    // 2. OCR Motorunu başlat
    // let ocr_engine = ocr::HonOcr::new(); // (OCR yapısı tamamlandığında aktif edilecek)

    // HP Bar koordinatlarını al (Python'daki config'den)
    let (hp_x, hp_y, hp_w, hp_h) = config.get_my_hp_rect();
    println!("HP Bölgesi İzleniyor: x={} y={} w={} h={}", hp_x, hp_y, hp_w, hp_h);

    // Ana Döngü (100ms'de bir çalışır)
    let mut interval = time::interval(Duration::from_millis(100));

    loop {
        interval.tick().await;

        // 1. Ekran Görüntüsü Al
        let pixels = ScreenCapture::capture(hp_x, hp_y, hp_w, hp_h);

        // Şimdilik sadece piksel okuduğumuzu kanıtlayalım (Performans testi)
        // Ortadaki pikselin rengine bakalım (Basit bir sağlık kontrolü simülasyonu)
        let center_idx = (pixels.len() / 2) - ((pixels.len() / 2) % 4);
        let b = pixels[center_idx];
        let g = pixels[center_idx + 1];
        let r = pixels[center_idx + 2];

        // Bu çıktı çok hızlı akmalı (Python'dan farkı göreceksiniz)
        // print!("\rOkunan Renk (BGRA): [{}, {}, {}, 255]", b, g, r); 
        
        // Buraya OCR ve mantık eklenecek...
    }
}