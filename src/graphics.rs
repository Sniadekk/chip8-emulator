use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;

pub struct Display {
    pub screen: [[i8; 64]; 32],
    pub should_draw: bool,
    pub canvas: Canvas<Window>,
}

impl Display {
    pub fn new(canvas: Canvas<Window>) -> Self {
        Display {
            screen: [[0; 64]; 32],
            should_draw: true,
            canvas,
        }
    }

    pub fn draw_sprite(&mut self, x: usize, y: usize, sprite: &[u8]) -> u8 {
        let mut collision = 0u8;
        let n = sprite.len() as usize;
        let mut yj: usize;
        let mut xi: usize;

        for j in range(0, n) {
            for i in range(0, 8) {
                yj = (y + j) % 32;
                xi = (x + i) % 64;

                if (sprite[j] & (0x80 >> i)) != 0 {
                    if self.gfx[yj][xi] == 1 { collision = 1 }
                    self.gfx[yj][xi] ^= 1;
                }
            }
        }

        self.should_draw = true;
        collision
    }

    pub fn draw(&mut self) {
        for (row_index, row) in self.screen.iter().enumerate() {
            for &v in row.iter() {
                if v != 0 {
                    self.canvas.fill_rect(Rect::new(row_index as i32 * 10, v as i32 * 10, 10, 10));
                }
                eprintln!("v = {}", v);
            }
        }
        self.canvas.clear();
        self.canvas.present();
    }

    pub fn clear(&mut self) {
        self.screen = [[0; 64]; 32];
    }
}
