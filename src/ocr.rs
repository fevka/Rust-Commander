use windows::{
    Graphics::Imaging::{BitmapPixelFormat, SoftwareBitmap},
    Media::Ocr::OcrEngine,
    Storage::Streams::DataWriter,
};

pub struct HonOcr {
    engine: OcrEngine,
}

impl HonOcr {
    pub fn new() -> Self {
        let engine = OcrEngine::TryCreateFromUserProfileLanguages()
            .expect("OCR Motoru başlatılamadı. Windows dil paketi yüklü mü?");
        Self { engine }
    }

    pub async fn read_text(&self, width: i32, height: i32, pixel_data: &[u8]) -> String {
        // 1. Piksel verisini (Vec<u8>) Windows Buffer'ına çevir
        let writer = DataWriter::new().unwrap();
        // Veriyi yaz
        writer.WriteBytes(pixel_data).unwrap();
        // Buffer nesnesini al
        let buffer = writer.DetachBuffer().unwrap();

        // 2. Buffer'dan SoftwareBitmap oluştur
        // CreateCopyFromBuffer fonksiyonu veriyi kopyalar, böylece güvenlidir.
        let bitmap = SoftwareBitmap::CreateCopyFromBuffer(
            &buffer,
            BitmapPixelFormat::Bgra8,
            width,
            height
        ).unwrap();

        // 3. OCR İşlemi (Artık hata vermeyecek)
        match self.engine.RecognizeAsync(&bitmap) {
            Ok(op) => match op.await {
                Ok(result) => result.Text().unwrap_or_default().to_string(),
                Err(_) => String::new(),
            },
            Err(_) => String::new(),
        }
    }
}