use windows::Win32::Graphics::Gdi::{
    BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetDC, GetDIBits,
    SelectObject, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, SRCCOPY,
};
use std::ffi::c_void;

pub struct ScreenCapture;

impl ScreenCapture {
    pub fn capture(x: i32, y: i32, w: i32, h: i32) -> Vec<u8> {
        unsafe {
            // HATA ÇÖZÜMÜ: GetDC artık Win32_Foundation özelliği ile erişilebilir
            let screen_dc = GetDC(None);
            let mem_dc = CreateCompatibleDC(screen_dc);
            let bitmap = CreateCompatibleBitmap(screen_dc, w, h);
            SelectObject(mem_dc, bitmap);

            // Ekran görüntüsü al
            BitBlt(mem_dc, 0, 0, w, h, screen_dc, x, y, SRCCOPY).unwrap();

            let mut bi = BITMAPINFO {
                bmiHeader: BITMAPINFOHEADER {
                    biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                    biWidth: w,
                    biHeight: -h, // Negatif değer = yukarıdan aşağı okuma
                    biPlanes: 1,
                    biBitCount: 32,
                    // HATA ÇÖZÜMÜ: BI_RGB artık bir struct, .0 ile değerini alıyoruz
                    biCompression: BI_RGB.0, 
                    ..Default::default()
                },
                ..Default::default()
            };

            let mut pixels: Vec<u8> = vec![0; (w * h * 4) as usize];
            
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
            // ReleaseDC(None, screen_dc); // İdealde screen_dc de bırakılmalı ama unsafe blokta kritik değil
            
            pixels
        }
    }
}