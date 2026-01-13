// Ayarları tutacak obje
let config = {
    faction: 'Legion',
    hero: 'Legionnaire',
    ui: 'Right',
    gpu: true
};

function selectOption(category, value, btnElement) {
    config[category] = value;
    let group = document.getElementById(category + '-group');
    let buttons = group.getElementsByClassName('btn-toggle');
    for (let btn of buttons) {
        btn.classList.remove('active');
    }
    btnElement.classList.add('active');
    checkReady();
}

function checkReady() {
    let btn = document.getElementById('btn-launch');
    btn.classList.add('ready');
    btn.disabled = false;
    document.getElementById('system-status').innerText = "SYSTEM: READY TO ENGAGE";
    document.getElementById('system-status').style.color = "#00f0ff";
}

function launchBot() {
    console.log("Launching with config:", config);
    if (window.__TAURI__) {
        window.__TAURI__.invoke('start_bot', { settings: config });
        // Butonu güncelle
        let btn = document.getElementById('btn-launch');
        btn.innerText = "SYSTEM ACTIVE - TRACKING TIME";
        btn.style.backgroundColor = "#00f0ff";
        btn.style.color = "#000";
    } else {
        alert("Bot Başlatılıyor (Test Modu)");
    }
}

// BOSS ÖLDÜ BİLDİRİMİ
function reportBoss(bossName) {
    if (window.__TAURI__) {
        window.__TAURI__.invoke('boss_dead', { boss: bossName });
        
        // Görsel efekt: Buton kırmızı yanıp söner
        let btns = document.getElementsByClassName('btn-boss');
        for (let btn of btns) {
            if (btn.innerText.includes(bossName.toUpperCase())) {
                let originalText = btn.innerText;
                btn.innerText = "TIMER SET!";
                btn.style.borderColor = "#ff1744";
                btn.style.color = "#ff1744";
                setTimeout(() => {
                    btn.innerText = originalText;
                    btn.style.borderColor = "";
                    btn.style.color = "";
                }, 2000);
            }
        }
    } else {
        console.log("Boss Dead:", bossName);
    }
}

document.addEventListener('DOMContentLoaded', () => {
    checkReady();
});