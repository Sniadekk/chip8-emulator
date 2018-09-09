pub struct Display {
    pub screen: [[i8; 64]; 32],
    pub should_draw: bool,
}

impl Display {
    pub fn new() -> Self {
        Display {
            screen: [[0; 64]; 32],
            should_draw: true,
        }
    }

    pub fn draw(&mut self) {
        for row in self.screen.iter() {
            for &v in row.iter() {
                if v == 0 {
                    print!("□");
                } else {
                    print!("■")
                }
            }
            print!("\n");
        }
        print!("{}[2J", 27 as char);
    }

    pub fn clear(&mut self) {
        self.screen = [[0; 64]; 32];
    }
}
