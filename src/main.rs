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

    loop {
        chip8.run_instruction();
    }
}
