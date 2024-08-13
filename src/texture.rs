
use image::ImageReader;
use image::{GenericImageView, Pixel};

pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>, // El buffer que contendrá los colores de la textura
}

impl Texture {
    /// Crea una nueva instancia de Texture a partir de un archivo de imagen.
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Cargar la imagen utilizando la crate `image`
        let img = ImageReader::open(path)?.decode()?;
        let (width, height) = img.dimensions();

        // Convertir la imagen a RGBA8 y cargar los datos de píxeles en el buffer
        let buffer: Vec<u32> = img
            .to_rgba8()
            .pixels()
            .map(|p| {
                let rgba = p.0;
                (rgba[0] as u32) << 16 | (rgba[1] as u32) << 8 | (rgba[2] as u32)
            })
            .collect();

        Ok(Texture {
            width: width as usize,
            height: height as usize,
            buffer,
        })
    }

    /// Retorna el color en una posición (x, y) específica en la textura.
    /// `x` y `y` deben ser valores normalizados entre 0.0 y 1.0
    pub fn get_color(&self, x: f32, y: f32) -> u32 {
        // Asegurarse de que x y y están dentro del rango [0, 1]
        let x = x.clamp(0.0, 1.0);
        let y = y.clamp(0.0, 1.0);

        // Convertir coordenadas normalizadas a índices de píxel
        let tex_x = (x * self.width as f32) as usize;
        let tex_y = (y * self.height as f32) as usize;

        // Retornar el color del buffer
        self.buffer[tex_y * self.width + tex_x]
    }
    
    /// Obtiene un color interpolado basado en coordenadas flotantes (sub-píxeles).
    pub fn get_color_interpolated(&self, x: f32, y: f32) -> u32 {
        let x = x.clamp(0.0, 1.0) * (self.width - 1) as f32;
        let y = y.clamp(0.0, 1.0) * (self.height - 1) as f32;

        // Coordenadas de los cuatro píxeles más cercanos
        let x1 = x.floor() as usize;
        let y1 = y.floor() as usize;
        let x2 = (x1 + 1).min(self.width - 1);
        let y2 = (y1 + 1).min(self.height - 1);

        // Valores de mezcla
        let fx = x - x.floor();
        let fy = y - y.floor();

        // Colores de los cuatro píxeles
        let c11 = self.buffer[y1 * self.width + x1];
        let c12 = self.buffer[y2 * self.width + x1];
        let c21 = self.buffer[y1 * self.width + x2];
        let c22 = self.buffer[y2 * self.width + x2];

        // Interpolación bilineal
        let r = (
            (((c11 >> 16) & 0xFF) as f32 * (1.0 - fx) + ((c21 >> 16) & 0xFF) as f32 * fx) * (1.0 - fy)
            + (((c12 >> 16) & 0xFF) as f32 * (1.0 - fx) + ((c22 >> 16) & 0xFF) as f32 * fx) * fy
        ) as u32;
        let g = (
            (((c11 >> 8) & 0xFF) as f32 * (1.0 - fx) + ((c21 >> 8) & 0xFF) as f32 * fx) * (1.0 - fy)
            + (((c12 >> 8) & 0xFF) as f32 * (1.0 - fx) + ((c22 >> 8) & 0xFF) as f32 * fx) * fy
        ) as u32;
        let b = (
            (((c11) & 0xFF) as f32 * (1.0 - fx) + ((c21) & 0xFF) as f32 * fx) * (1.0 - fy)
            + (((c12) & 0xFF) as f32 * (1.0 - fx) + ((c22) & 0xFF) as f32 * fx) * fy
        ) as u32;

        (r << 16) | (g << 8) | b
    }
}
