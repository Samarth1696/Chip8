use wasm_bindgen::prelude::*;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

#[wasm_bindgen]
pub struct Display {
    screen: [u8; WIDTH * HEIGHT],
}

#[wasm_bindgen]
impl Display {
    pub fn new() -> Display {
        Display {
            screen: [0; WIDTH * HEIGHT],
        }
    }

    pub fn get_display_memory(&self) -> Vec<u8> {
        self.screen.to_vec()
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, on: bool) {
        self.screen[y * WIDTH + x] = on as u8;
    }

    pub fn get_index_from_coords(x: usize, y: usize) -> usize {
        y * WIDTH + x
    }

    pub fn debug_draw_byte(&mut self, byte: u8, x: u8, y: u8) -> bool {
        let mut erased = false;
        let mut b = byte;
        let mut coord_x = x as usize;
        let mut coord_y = y as usize;
        // console::log_1(self.screen);
        for _ in 0..8 {
            coord_x %= WIDTH;
            coord_y %= HEIGHT;
            let index = Display::get_index_from_coords(coord_x, coord_y);
            let bit = (b & 0b1000_0000) >> 7;
            let prev_value = self.screen[index];
            let new_value = self.screen[index] ^ bit;
            
            if prev_value == 1 && new_value == 0 {
                erased = true;
            }
            self.set_pixel(coord_x, coord_y, if new_value == 0 { false } else { true });
            coord_x += 1;
            b = b << 1;
        }

        erased
    }

    pub fn clear(&mut self) {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
              self.set_pixel(x, y, false);
            }
          }
    }
}
