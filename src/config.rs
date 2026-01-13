use windows::Win32::UI::WindowsAndMessaging::GetSystemMetrics;
use windows::Win32::UI::WindowsAndMessaging::{SM_CXSCREEN, SM_CYSCREEN};

// Python'daki hon_config.py REF_W ve REF_H değerleri
const REF_W: f32 = 3840.0;
const REF_H: f32 = 2160.0;

pub struct GameConfig {
    pub screen_w: i32,
    pub screen_h: i32,
    scale_x: f32,
    scale_y: f32,
}

impl GameConfig {
    pub fn new() -> Self {
        unsafe {
            // Windows API ile ekran çözünürlüğünü otomatik al
            let w = GetSystemMetrics(SM_CXSCREEN);
            let h = GetSystemMetrics(SM_CYSCREEN);
            
            println!("--- EKRAN ALGILANDI: {}x{} ---", w, h);

            Self {
                screen_w: w,
                screen_h: h,
                scale_x: w as f32 / REF_W,
                scale_y: h as f32 / REF_H,
            }
        }
    }

    // Koordinat ölçekleme (hon_utils.py 's' fonksiyonu)
    pub fn s(&self, val: i32, axis: char) -> i32 {
        match axis {
            'x' => (val as f32 * self.scale_x) as i32,
            'y' => (val as f32 * self.scale_y) as i32,
            _ => val,
        }
    }

    // Python'daki COMMON_BOXES["MY_HP"] kutusu
    pub fn get_my_hp_rect(&self) -> (i32, i32, i32, i32) {
        (
            self.s(1780, 'x'), // Left
            self.s(2065, 'y'), // Top
            self.s(280, 'x'),  // Width
            self.s(45, 'y'),   // Height
        )
    }
}