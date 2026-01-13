// Ayarları tutacak obje
let config = {
    faction: 'Legion',     // Varsayılan
    hero: 'Legionnaire',   // Varsayılan
    ui: 'Right',           // Varsayılan
    gpu: true
};

// Seçenek butonlarına tıklandığında çalışır
function selectOption(category, value, btnElement) {
    // 1. Config'i güncelle
    config[category] = value;

    // 2. Görsel güncelleme (Active sınıfını değiştir)
    // Önce o gruptaki tüm butonlardan 'active' sınıfını sil
    let group = document.getElementById(category + '-group');
    let buttons = group.getElementsByClassName('btn-toggle');
    for (let btn of buttons) {
        btn.classList.remove('active');
    }
    // Tıklanan butona ekle
    btnElement.classList.add('active');

    checkReady();
}

// Başlat butonu kontrolü
function checkReady() {
    let btn = document.getElementById('btn-launch');
    // Basit kontrol: Her şey seçili mi? (Varsayılanlar olduğu için hep hazır)
    btn.classList.add('ready');
    btn.disabled = false;
    document.getElementById('system-status').innerText = "SYSTEM: READY TO ENGAGE";
    document.getElementById('system-status').style.color = "#00f0ff";
}

// Botu Başlatma Fonksiyonu
function launchBot() {
    // Burada Rust tarafına sinyal göndereceğiz
    console.log("Launching with config:", config);
    
    // Tauri invoke fonksiyonu (İleride Rust ile bağlayacağız)
    if (window.__TAURI__) {
        window.__TAURI__.invoke('start_bot', { settings: config });
    } else {
        alert("Bot Başlatılıyor!\n" + JSON.stringify(config, null, 2));
    }
}

// Sayfa yüklendiğinde çalışacaklar
document.addEventListener('DOMContentLoaded', () => {
    checkReady();
    
    // GPU Toggle Dinleyici
    document.getElementById('gpu-toggle').addEventListener('change', (e) => {
        config.gpu = e.target.checked;
        console.log("GPU:", config.gpu);
    });
});