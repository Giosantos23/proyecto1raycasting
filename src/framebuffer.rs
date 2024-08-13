pub struct Color(pub u32);


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
            background_color: 0x000000, //blanco
            current_color: 0xFFFFFF //negro
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

    pub fn point_with_color(&mut self, x: usize, y: usize, color: Color) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = color.0;
        }
    }
    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color.0;

    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color.0;
 
    }
}