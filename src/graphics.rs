pub struct Display {
    screen: [[i8; 64]; 32]
}

impl Display {
    pub fn new() -> Self {
        Display {
            screen: [[0; 64]; 32]
        }
    }
}
