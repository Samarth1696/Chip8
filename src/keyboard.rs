use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Keyboard{
    pub key_pressed: Option<u8>     
}

#[wasm_bindgen]
impl Keyboard{
    pub fn new() -> Keyboard{
        Keyboard {
            key_pressed: None
        }
    }
    //TODO implement proper key handling
    pub fn is_key_pressed(&self, key_code: u8) -> bool {
        if let Some(key) = self.key_pressed {
            key == key_code
       } else {
           false
       }
    }

    pub fn key_down(&mut self, key: Option<u8>) {
        self.key_pressed = None;
    }
    
    pub fn key_up(&mut self, key: u8) {
        self.key_pressed = Some(key);
    }

    pub fn get_key_pressed(&self) -> Option<u8> {
        self.key_pressed
    }
}