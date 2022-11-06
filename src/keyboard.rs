pub struct Keyboard{
    pub key_pressed: [bool; 16]  
}

impl Keyboard{
    pub fn new() -> Keyboard{
        Keyboard {
            key_pressed: [false; 16]
        }
    }
    //TODO implement proper key handling
    
    pub fn is_key_pressed(&self, index: u8) -> bool {
        self.key_pressed[index as usize]
    }

    pub fn key_down(&mut self, index: u8) {
        self.key_pressed[index as usize] = true;
    }
    
      pub fn key_up(&mut self, index: u8) {
        self.key_pressed[index as usize] = false;
    }
}