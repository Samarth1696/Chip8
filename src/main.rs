extern crate minifb;

use minifb::{Key, Window, WindowOptions};
use std::fs::File;
use std::io::Read;
use chip8::Chip8;

mod ram;
mod bus;
mod cpu;
mod chip8;
mod keyboard;
mod display;

fn main() {
    let mut file = File::open("data/INVADERS").unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data).unwrap();

    let mut chip8 = Chip8::new();
    chip8.load_rom(data);

    const WIDTH: usize = 640;
    const HEIGHT: usize = 320;

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Chip8 Emulator in Rust",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        chip8.run_instruction();
        let chip8_buffer = chip8.get_display_buffer();

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }

    // loop {
    //     chip8.run_instruction();
    // }
}
