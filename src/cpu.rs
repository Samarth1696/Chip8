use crate::display::Display;
use crate::keyboard::Keyboard;
use crate::ram::Ram;
use crate::rand::ComplementaryMultiplyWithCarryGen;
use wasm_bindgen::prelude::*;

pub const PROGRAM_START: u16 = 0x200;

#[wasm_bindgen]
pub struct Cpu {
    vx: [u8; 16],
    pc: u16,
    i: u16,
    ret_stack: Vec<u16>,
    rand: ComplementaryMultiplyWithCarryGen,
    ram: Ram,
    display: Display,
    keyboard: Keyboard,
    delay_timer: u8,
    sound_timer: u8,
}

#[wasm_bindgen]
impl Cpu {
    pub fn new(data: &[u8]) -> Cpu {
        let mut memory: Vec<u8> = vec![0; 4096];
        for i in 0..data.len() {
            let p = i as usize;
            memory[0x200 + p] = data[p];
        }
        let mut cpu = Cpu {
            ram: Ram { mem: memory },
            vx: [0; 16],
            pc: PROGRAM_START,
            i: 0,
            ret_stack: Vec::<u16>::new(),
            rand: ComplementaryMultiplyWithCarryGen::new(1),
            display: Display::new(),
            keyboard: Keyboard::new(),
            delay_timer: 0,
            sound_timer: 0,
        };

        let sprites: [[u8; 5]; 16] = [
            [0xF0, 0x90, 0x90, 0x90, 0xF0], //0
            [0x20, 0x60, 0x20, 0x20, 0x70], //1
            [0xF0, 0x10, 0xF0, 0x10, 0xF0], //2
            [0xF0, 0x10, 0xF0, 0x10, 0xF0], //3
            [0x90, 0x90, 0xF0, 0x10, 0x10], //4
            [0xF0, 0x80, 0xF0, 0x10, 0xF0], //5
            [0xF0, 0x80, 0xF0, 0x90, 0xF0], //6
            [0xF0, 0x10, 0x20, 0x40, 0x40], //7
            [0xF0, 0x90, 0xF0, 0x90, 0xF0], //8
            [0xF0, 0x90, 0xF0, 0x10, 0xF0], //9
            [0xF0, 0x90, 0xF0, 0x90, 0x90], //A
            [0xE0, 0x90, 0xE0, 0x90, 0xE0], //B
            [0xF0, 0x80, 0x80, 0x80, 0xF0], //C
            [0xE0, 0x90, 0x90, 0x90, 0xE0], //D
            [0xF0, 0x80, 0xF0, 0x80, 0xF0], //E
            [0xF0, 0x80, 0xF0, 0x80, 0x80], //F
        ];
        let mut i = 0;
        for sprite in sprites.iter() {
            for ch in sprite {
                cpu.ram.write_byte(i, *ch);
                i += 1;
            }
        }

        cpu
    }

    pub fn cpu_refresh() -> Cpu {
        Cpu {
            ram: Ram::new(),
            vx: [0; 16],
            pc: PROGRAM_START,
            i: 0,
            ret_stack: Vec::<u16>::new(),
            rand: ComplementaryMultiplyWithCarryGen::new(1),
            display: Display::new(),
            keyboard: Keyboard::new(),
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn key_up(&mut self, key: u8) {
        self.keyboard.key_up(key);
    }

    pub fn key_down(&mut self, key: u8) {
        self.keyboard.key_down(key);
    }

    pub fn get_display_memory(&self) -> Vec<u8> {
        self.display.get_display_memory()
    }

    pub fn get_memory(&self) -> Vec<u8> {
        self.ram.get_memory()
    }

    pub fn tick(&mut self) {
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }

        if self.delay_timer > 0 {
            self.delay_timer -= 1
        }
    }

    pub fn run_instruction(&mut self) {
        let hi = self.ram.read_byte(self.pc) as u16;
        let lo = self.ram.read_byte(self.pc + 1) as u16;
        let instruction: u16 = (hi << 8) | lo;

        let nnn = instruction & 0x0FFF;
        let nn = (instruction & 0x0FF) as u8;
        let n = (instruction & 0x00F) as u8;
        let x = ((instruction & 0x0F00) >> 8) as u8;
        let y = ((instruction & 0x00F0) >> 4) as u8;

        match (instruction & 0xF000) >> 12 {
            0x0 => {
                match nn {
                    0xE0 => {
                        self.display.clear();
                        self.pc += 2;
                    }
                    0xEE => {
                        //return from subroutine
                        let addr = self.ret_stack.pop().unwrap();
                        self.pc = addr;
                    }
                    _ => panic!(
                        "Unrecognized 0x00** instruction {:#X}:{:#X}",
                        self.pc, instruction
                    ),
                }
            }
            0x1 => {
                // goto nnn
                self.pc = nnn;
            }
            0x2 => {
                //Call subroutine at address NNN
                self.ret_stack.push(self.pc + 2);
                self.pc = nnn;
            }
            0x3 => {
                // if(Vx==nn)
                let vx = self.read_reg_vx(x);
                if vx == nn {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0x4 => {
                // if(Vx!=nn) Skip to next instruction
                let vx = self.read_reg_vx(x);
                if vx != nn {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0x5 => {
                //Skip next instruction if(Vx==Vy)
                let vx = self.read_reg_vx(x);
                let vy = self.read_reg_vx(y);
                if vx == vy {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0x6 => {
                // vx = nn
                self.write_reg_vx(x, nn);
                self.pc += 2;
            }
            0x7 => {
                let vx = self.read_reg_vx(x);
                self.write_reg_vx(x, vx.wrapping_add(nn));
                self.pc += 2;
            }
            0x8 => {
                match n {
                    0x0 => {
                        // Vx=Vy
                        let vy = self.read_reg_vx(y);
                        self.write_reg_vx(x, vy);
                    }
                    0x2 => {
                        // Vx &= Vy
                        let vy = self.read_reg_vx(y);
                        let vx = self.read_reg_vx(x);
                        self.write_reg_vx(x, vx & vy);
                    }
                    0x3 => {
                        // Vx ^ Vy
                        let vy = self.read_reg_vx(y);
                        let vx = self.read_reg_vx(x);
                        self.write_reg_vx(x, vx ^ vy);
                    }
                    0x4 => {
                        // Vx += Vy
                        let vy = self.read_reg_vx(y);
                        let vx = self.read_reg_vx(x);
                        let sum = (vx as u16).wrapping_add(vy as u16);
                        match sum > 0xFF {
                            true => self.write_reg_vx(0xF, 1),
                            false => self.write_reg_vx(0xF, 0),
                        }
                        self.write_reg_vx(x, sum as u8);
                    }
                    0x5 => {
                        let vy = self.read_reg_vx(y);
                        let vx = self.read_reg_vx(x);
                        let diff = vx.wrapping_sub(vy);
                        match vx > vy {
                            true => self.write_reg_vx(0xF, 1),
                            false => self.write_reg_vx(0xF, 0),
                        }
                        self.write_reg_vx(x, diff);
                    }
                    0x6 => {
                        // Vx=Vx>>1
                        let vx = self.read_reg_vx(x);
                        self.write_reg_vx(0xF, vx & 0x1);
                        self.write_reg_vx(x, vx >> 1);
                    }
                    0x7 => {
                        let vy = self.read_reg_vx(y);
                        let vx = self.read_reg_vx(x);
                        let diff = vy.wrapping_sub(vx);
                        match vy > vx {
                            true => self.write_reg_vx(0xF, 1),
                            false => self.write_reg_vx(0xF, 0),
                        }
                        self.write_reg_vx(x, diff);
                    }
                    0xE => {
                        // 0xF is the most significant bit value.
                        // SHR Vx
                        let vx = self.read_reg_vx(x);
                        self.write_reg_vx(0xF, (vx & 0x80) >> 7);
                        self.write_reg_vx(x, vx << 1);
                    }
                    _ => panic!(
                        "Unrecognized 0x8XY* instruction {:#X}:{:#X}",
                        self.pc, instruction
                    ),
                };

                self.pc += 2;
            }
            0x9 => {
                //skips the next instruction if(Vx!=Vy)
                let vx = self.read_reg_vx(x);
                let vy = self.read_reg_vx(y);
                if vx != vy {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0xA => {
                // I = NNN
                self.i = nnn;
                self.pc += 2;
            }
            0xB => {
                self.pc = self.read_reg_vx(0) as u16 + nnn;
            }
            0xC => {
                // Vx=rand()&NN
                let number = self.rand.random() as u8;
                self.write_reg_vx(x, number & nn);
                self.pc += 2;
            }
            0xD => {
                //draw(Vx,Vy,N)
                let vx = self.read_reg_vx(x);
                let vy = self.read_reg_vx(y);
                self.debug_draw_sprite(vx, vy, n);
                self.pc += 2;
            }
            0xE => {
                match nn {
                    0xA1 => {
                        //if(key()!=Vx) then skip the next instruction
                        let key = self.read_reg_vx(x);
                        if !self.keyboard.is_key_pressed(key) {
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    }
                    0x9E => {
                        //if(key()==Vx) then skip the next instruction
                        let key = self.read_reg_vx(x);
                        if self.keyboard.is_key_pressed(key) {
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    }
                    _ => panic!(
                        "Unrecognized 0xEX** instruction {:#X}:{:#X}",
                        self.pc, instruction
                    ),
                };
            }

            0xF => {
                match nn {
                    0x07 => {
                        self.write_reg_vx(x, self.delay_timer);
                        self.pc += 2;
                    }
                    0x0A => {
                        // Vx = get_key()
                        for (i, key) in self.keyboard.key_pressed.iter().enumerate() {
                            if *key == true {
                                self.vx[x as usize] = i as u8;
                                self.pc += 2;
                            }
                        }
                    }
                    0x15 => {
                        // delay_timer(Vx)
                        self.delay_timer = self.read_reg_vx(x);
                        self.pc += 2;
                    }
                    0x18 => {
                        // Sound timer (skipped for now)
                        self.sound_timer = self.read_reg_vx(x);
                        self.pc += 2;
                    }
                    0x1E => {
                        // I += Vx
                        let vx = self.read_reg_vx(x);
                        self.i += vx as u16;
                        self.pc += 2;
                    }
                    0x29 => {
                        //i == sprite address for character in Vx
                        //Multiply by 5 because each sprite has 5 lines, each line
                        //is 1 byte.
                        self.i = self.read_reg_vx(x) as u16 * 5;
                        self.pc += 2;
                    }
                    0x33 => {
                        let vx = self.read_reg_vx(x);
                        self.ram.write_byte(self.i, vx / 100);
                        self.ram.write_byte(self.i + 1, (vx % 100) / 10);
                        self.ram.write_byte(self.i + 2, vx % 10);
                        self.pc += 2;
                    }
                    0x55 => {
                        // reg_dump(Vx, &I)
                        for index in 0..x + 1 {
                            let value = self.read_reg_vx(index);
                            self.ram.write_byte(self.i + index as u16, value);
                        }
                        self.i += x as u16 + 1;
                        self.pc += 2;
                    }
                    0x65 => {
                        // reg_load(Vx, &I)
                        for index in 0..x + 1 {
                            let value = self.ram.read_byte(self.i + index as u16);
                            self.write_reg_vx(index, value);
                        }
                        self.i += x as u16 + 1;
                        self.pc += 2;
                    }
                    _ => panic!(
                        "Unrecognized 0xF0** instruction {:#X}:{:#X}",
                        self.pc, instruction
                    ),
                }
            }
            _ => panic!("Unrecognized instruction {:#X}:{:#X}", self.pc, instruction),
        }
    }

    fn debug_draw_sprite(&mut self, x: u8, y: u8, height: u8) {
        let mut should_set_vf = false;
        for sprite_y in 0..height {
            let b = self.ram.read_byte(self.i + sprite_y as u16);
            if self.display.debug_draw_byte(b, x, y + sprite_y) {
                should_set_vf = true;
            }
        }
        if should_set_vf {
            self.write_reg_vx(0xF, 1);
        } else {
            self.write_reg_vx(0xF, 0);
        }
        // bus.present_screen();
    }

    pub fn write_reg_vx(&mut self, index: u8, value: u8) {
        self.vx[index as usize] = value
    }

    pub fn read_reg_vx(&mut self, index: u8) -> u8 {
        self.vx[index as usize]
    }
}
