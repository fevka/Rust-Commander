use windows::Win32::Graphics::Gdi::{
    BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetDC, GetDIBits,
    SelectObject, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, HBITMAP, HDC, SRCCOPY,
};
use std::ptr::null_mut;
use std::ffi::c_void;

pub struct ScreenCapture {
    // Gerekirse durum tutmak için
}

impl ScreenCapture {
    pub fn capture(x: i32, y: i32, w: i32, h: i32) -> Vec<u8> {
        unsafe {
            let screen_dc = GetDC(None);
            let mem_dc = CreateCompatibleDC(screen_dc);
            let bitmap = CreateCompatibleBitmap(screen_dc, w, h);
            SelectObject(mem_dc, bitmap);

            // Ekranın o bölgesini kopyala (Screenshot)
            BitBlt(mem_dc, 0, 0, w, h, screen_dc, x, y, SRCCOPY).unwrap();

            // Pikselleri okumak için hazırlık
            let mut bi = BITMAPINFO {
                bmiHeader: BITMAPINFOHEADER {
                    biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                    biWidth: w,
                    biHeight: -h, // Negatif değer resmi yukarıdan aşağı okur
                    biPlanes: 1,
                    biBitCount: 32, // BGRA formatı (4 byte)
                    biCompression: BI_RGB,
                    ..Default::default()
                },
                ..Default::default()
            };

            let mut pixels: Vec<u8> = vec![0; (w * h * 4) as usize];
            
            // Bitmap verisini vektöre çek
            GetDIBits(
                mem_dc,
                bitmap,
                0,
                h as u32,
                Some(pixels.as_mut_ptr() as *mut c_void),
                &mut bi,
                DIB_RGB_COLORS,
            );

            // Temizlik
            DeleteObject(bitmap);
            DeleteDC(mem_dc);
            
            pixels
        }
    }
}