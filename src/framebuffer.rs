use image::{GenericImageView, RgbaImage};

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    background_color: u32,
    current_color: u32,
    background_image: Option<RgbaImage>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![0; width * height],
            background_color: 0x000000,
            current_color: 0xFFFFFF,
            background_image: None,
        }
    }

    pub fn clear(&mut self) {
        if let Some(image) = &self.background_image {
            for y in 0..self.height {
                for x in 0..self.width {
                    let pixel = image.get_pixel(x as u32, y as u32);
                    let r = pixel[0] as u32;
                    let g = pixel[1] as u32;
                    let b = pixel[2] as u32;
                    self.buffer[y * self.width + x] = (r << 16) | (g << 8) | b;
                }
            }
        } else {
            for pixel in self.buffer.iter_mut() {
                *pixel = self.background_color;
            }
        }
    }

    pub fn point(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = self.current_color;
        }
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }

    pub fn set_background_image(&mut self, image_path: &str) {
        match image::open(image_path) {
            Ok(img) => {
                let img = img.to_rgba8();
                let resized_img = image::imageops::resize(&img, self.width as u32, self.height as u32, image::imageops::FilterType::Nearest);
                self.background_image = Some(resized_img);
            }
            Err(e) => {
                println!("Failed to load image: {}", e);
            }
        }
    }

    pub fn draw_text(&mut self, x: usize, y: usize, text: &str) {
        // Implementación básica para dibujar texto. 
        // Nota: La implementación real debería usar una biblioteca para fuentes.
        // Este código es solo un ejemplo y puede necesitar ajustes según la biblioteca que uses para el manejo de fuentes.

        // Define una simple fuente de texto en términos de píxeles (esto es solo un ejemplo)
        let font = vec![
            0b0110_0000, // 6
            0b0101_1000, // 5
            0b0100_0100, // 4
            0b0101_1000, // 5
            0b0110_0000, // 6
        ];

        for (i, ch) in text.chars().enumerate() {
            let char_x = x + i * 8; // Espaciado de caracteres
            for (row, bits) in font.iter().enumerate() {
                for col in 0..8 {
                    if bits & (1 << (7 - col)) != 0 {
                        self.point(char_x + col, y + row);
                    }
                }
            }
        }
    }
}
