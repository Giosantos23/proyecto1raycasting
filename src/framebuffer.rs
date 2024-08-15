pub struct Color(pub u32);
use rusttype::{Font, Scale, point, PositionedGlyph};


pub struct Framebuffer{
//lo utilizamos para generar archivo
//buffer: matriz donde se generan colores
    pub width :usize,
    pub height :usize,
    pub buffer: Vec<u32>,
    //matriz
    background_color: u32,
    current_color: u32,
}
impl Framebuffer{
    //traits, como interfaces en POO
    pub fn new(width: usize, height: usize) -> Self {
        //unsize define tipo desconocido
        //-> tipo de retorno tipo Self o framebuffer
        Framebuffer{
            width,
            height,
            buffer: vec![0; width * height],
            //vector lleno de ceros con alto y ancho
            //operador ! sirve para realizar calculo
            background_color: 0x000000, 
            current_color: 0xFFFFFF 
        }

    }
    pub fn clear(&mut self) {
        //pinta la pantalla de un color para limpiar
        //mutabilidad de self 
        for pixel in self.buffer.iter_mut(){
            //iterador mutable sobre vector buffer
            *pixel = self.background_color;
        }


    }
    pub fn point(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = self.current_color;
        }
    }

    //pub fn point_with_color(&mut self, x: usize, y: usize, color: Color) {
    //    if x < self.width && y < self.height {
     //       self.buffer[y * self.width + x] = color.0;
     //   }
    //}
    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color.0;

    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color.0;
 
    }
    pub fn draw_text(&mut self, x: usize, y: usize, text: &str, color: Color) {
        let scale = Scale::uniform(20.0);
        let font_data = include_bytes!("../assets/avenir.ttc");
        let font = Font::try_from_bytes(font_data as &[u8]).unwrap();
        
        let v_metrics = font.v_metrics(scale);
        let start = point(x as f32, y as f32 + v_metrics.ascent);

        let glyphs: Vec<PositionedGlyph<'_>> = font.layout(text, scale, start).collect();

        for glyph in glyphs {
            if let Some(bb) = glyph.pixel_bounding_box() {
                glyph.draw(|gx, gy, gv| {
                    let gx = gx as i32 + bb.min.x;
                    let gy = gy as i32 + bb.min.y;
                    if gv > 0.3 {
                        if gx >= 0 && gx < self.width as i32 && gy >= 0 && gy < self.height as i32 {
                            self.buffer[(gy as usize) * self.width + gx as usize] = color.0;
                        }
                    }
                });
            }
        }
    }
}