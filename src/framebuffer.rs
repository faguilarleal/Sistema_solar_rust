// framebuffer.rs
use rand::Rng; 


pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    pub zbuffer: Vec<f32>,
    background_color: u32,
    current_color: u32,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![0; width * height],
            zbuffer: vec![f32::INFINITY; width * height],
            background_color: 0x000000,
            current_color: 0xFFFFFF,
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = self.background_color;
        }
        for depth in self.zbuffer.iter_mut() {
            *depth = f32::INFINITY;
        }
    }

    pub fn point(&mut self, x: usize, y: usize, depth: f32) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            if self.zbuffer[index] > depth {
                self.buffer[index] = self.current_color;
                self.zbuffer[index] = depth;
            }
        }
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
        self.set_background_with_stars(0xFFFFFF, 100);
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }

    pub fn set_background_with_stars(&mut self, star_color: u32, num_stars: usize) {
        // Limpia el framebuffer con el color de fondo
        for pixel in self.buffer.iter_mut() {
            *pixel = self.background_color;
        }

        // Genera estrellas aleatorias
        let mut rng = rand::thread_rng();
        for _ in 0..num_stars {
            let x = rng.gen_range(0..self.width); // Genera coordenada x aleatoria dentro del ancho
            let y = rng.gen_range(0..self.height); // Genera coordenada y aleatoria dentro del alto
            self.buffer[y * self.width + x] = star_color; // Dibuja la estrella en el buffer
        }
    }

}
