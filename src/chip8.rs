use rand::Rng;
use std::fs;

pub struct Chip8 {
    pub opcode: u16,
    pub memory: [u8; 4096],
    pub v: [u8; 16],
    pub index: u16,
    pub pc: u16,
    pub display: [u8; 2048],
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub draw_flag: bool,
    pub stack: [u16; 16],
    pub sp: u16,
    pub keypad: [u8; 16],
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let fontset: [u8; 80] = [ 
        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
        0x20, 0x60, 0x20, 0x20, 0x70, // 1
        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
        0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
        0x90, 0x90, 0xF0, 0x10, 0x10, // 4
        0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
        0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
        0xF0, 0x10, 0x20, 0x40, 0x40, // 7
        0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
        0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
        0xF0, 0x90, 0xF0, 0x90, 0x90, // A
        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
        0xF0, 0x80, 0x80, 0x80, 0xF0, // C
        0xE0, 0x90, 0x90, 0x90, 0xE0, // D
        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
        0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];
        
        // Load fontset into memory
        let mut memory: [u8; 4096] = [0; 4096];
        for i in 0..80 {
            memory[0x50 + i] = fontset[i];
        }

        Chip8 {
            opcode: 0,
            memory,
            v: [0; 16],
            index: 0,
            pc: 0x200,
            display: [0; 2048],
            delay_timer: 0,
            sound_timer: 0,
            draw_flag: false,
            stack: [0; 16],
            sp: 0,
            keypad: [0; 16],
        }
    }

    pub fn emulate_cycle(&mut self) {
        // Fetch Opcode
        self.opcode = (self.memory[self.pc as usize] as u16) << 8 
                    | (self.memory[(self.pc as usize) + 1]) as u16; 

        // Decode and Execute
        match self.opcode & 0xF000 {
            0x0000 => match self.opcode & 0x0FFF {
                0x00E0 => Self::insn_00e0(self),
                0x00EE => Self::insn_00ee(self),
                _ => Self::insn_0nnn(self),
            },
            0x1000 => Self::insn_1nnn(self),
            0x2000 => Self::insn_2nnn(self),
            0x3000 => Self::insn_3xnn(self),
            0x4000 => Self::insn_4xnn(self),
            0x5000 => Self::insn_5xy0(self),
            0x6000 => Self::insn_6xnn(self),
            0x7000 => Self::insn_7xnn(self),
            0x8000 => match self.opcode & 0x000F {
                0x0000 => Self::insn_8xy0(self),
                0x0001 => Self::insn_8xy1(self),
                0x0002 => Self::insn_8xy2(self),
                0x0003 => Self::insn_8xy3(self),
                0x0004 => Self::insn_8xy4(self),
                0x0005 => Self::insn_8xy5(self),
                0x0006 => Self::insn_8xy6(self),
                0x0007 => Self::insn_8xy7(self),
                0x000E => Self::insn_8xye(self),
                _ => println!("Unknown opcode: {:X}", self.opcode),
            },
            0x9000 => Self::insn_9xy0(self),
            0xA000 => Self::insn_annn(self),
            0xB000 => Self::insn_bnnn(self),
            0xC000 => Self::insn_cxnn(self),
            0xD000 => Self::insn_dxyn(self),
            0xE000 => match self.opcode & 0x00FF {
                0x009E => Self::insn_ex9e(self),
                0x00A1 => Self::insn_exa1(self),
                _ => println!("Unknown opcode: {:X}", self.opcode),
            },
            0xF000 => match self.opcode & 0x00FF {
                0x0007 => Self::insn_fx07(self),
                0x000A => Self::insn_fx0a(self),
                0x0015 => Self::insn_fx15(self),
                0x0018 => Self::insn_fx18(self),
                0x001E => Self::insn_fx1e(self),
                0x0029 => Self::insn_fx29(self),
                0x0033 => Self::insn_fx33(self),
                0x0055 => Self::insn_fx55(self),
                0x0065 => Self::insn_fx65(self),
                _ => println!("Unknown opcode: {:X}", self.opcode),
            },
            _ => println!("Unknown opcode: {:X}", self.opcode),
        };

        // Update timers
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                println!("BEEP!");
            }
            self.sound_timer -= 1;
        }
    }

    pub fn load_rom(&mut self, path: &str) {
        let data = fs::read(path).expect("Error opening file");

        let mut i = 0;
        for x in data.iter() {
            self.memory[0x200 + i] = *x;
            i += 1;
        }
    }

    fn insn_0nnn(&mut self) {
        // Not necessary for most ROMs according to wikipedia
    }

    fn insn_00e0(&mut self) {
        // Clear display
        self.display = [0; 2048];
        self.draw_flag = true;
        self.pc += 2;
    }

    fn insn_00ee(&mut self) {
        // Return from subroutine
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
        self.pc += 2;
    }

    fn insn_1nnn(&mut self) {
        // Jump to NNN
        self.pc = self.opcode & 0x0FFF;
    }

    fn insn_2nnn(&mut self) {
        // Enter subroutine
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = self.opcode & 0x0FFF;
    }

    fn insn_3xnn(&mut self) {
        // If Vx == NN, skip next instruction
        if self.v[bs8_usize(self.opcode)] == (self.opcode & 0x00FF) as u8 {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    fn insn_4xnn(&mut self) {
        // If Vx != NN, skip next instruction
        if self.v[bs8_usize(self.opcode)] != (self.opcode & 0x00FF) as u8 {
            self.pc += 4;
        } else {
            self.pc += 2;
        } 
    }

    fn insn_5xy0(&mut self) {
        // If Vx == Vy, skip next instruction
        if self.v[bs8_usize(self.opcode)] == self.v[bs4_usize(self.opcode)] {
            self.pc += 4;
        } else {
            self.pc += 2;
        }    
    }

    fn insn_6xnn(&mut self) {
        // Set Vx to NN
        self.v[bs8_usize(self.opcode)] = (self.opcode & 0x00FF) as u8;
        self.pc += 2;
    }

    fn insn_7xnn(&mut self) {
        // Add NN to Vx
        // Need u16 to prevent overflow panic
        let result = self.v[bs8_usize(self.opcode)]as u16 + (self.opcode & 0x00FF);
        self.v[bs8_usize(self.opcode)] = (result & 0x00FF) as u8;
        self.pc += 2;
    }

    fn insn_8xy0(&mut self) {
        // Set Vx = Vy
        self.v[bs8_usize(self.opcode)] = self.v[bs4_usize(self.opcode)];
        self.pc += 2;
    }

    fn insn_8xy1(&mut self) {
        // Set Vx = Vx | Vy
        self.v[bs8_usize(self.opcode)] |= self.v[bs4_usize(self.opcode)];
        self.pc += 2;
    }

    fn insn_8xy2(&mut self) {
        // Set Vx = Vx & Vy
        self.v[bs8_usize(self.opcode)] &= self.v[bs4_usize(self.opcode)];
        self.pc += 2;
    }

    fn insn_8xy3(&mut self) {
        // Set Vx = Vx ^ Vy
        self.v[bs8_usize(self.opcode)] ^= self.v[bs4_usize(self.opcode)];
        self.pc += 2;
    }

    fn insn_8xy4(&mut self) {
        // Add Vy to Vx, set overflow on Vf
        // need u16 to prevent overflow panic
        let result = self.v[bs8_usize(self.opcode)] as u16 + self.v[bs4_usize(self.opcode)] as u16; 
        self.v[bs8_usize(self.opcode)] = (result & 0x00FF) as u8;
        self.v[0xF] = ((result & 0x0100) >> 8) as u8;
        self.pc += 2;
    }

    fn insn_8xy5(&mut self) {
        // Sub Vy from Vx, set carry on Vf
        // Need u16 to prevent overflow panic
        let result = (0x0100 | (self.v[bs8_usize(self.opcode)] as u16)) - self.v[bs4_usize(self.opcode)] as u16;
        self.v[bs8_usize(self.opcode)] = (result & 0x00FF) as u8;
        self.v[0xF] = ((result & 0x0100) >> 8) as u8;
        self.pc += 2;
    }

    fn insn_8xy6(&mut self) {
        // Shift Vx right and store LSB in Vf
        self.v[0xF] = self.v[bs8_usize(self.opcode)] & 0x01;
        self.v[bs8_usize(self.opcode)] >>= 1;
        self.pc += 2;
    }

    fn insn_8xy7(&mut self) {
        // Sub Vx from Vy, store in Vx, set carry on Vf
        // Need u16 to prevent overflow panic
        let result = (0x0100 | (self.v[bs4_usize(self.opcode)] as u16)) - self.v[bs8_usize(self.opcode)] as u16;
        self.v[bs8_usize(self.opcode)] = (result & 0x00FF) as u8;
        self.v[0xF] = ((result & 0x0100) >> 4) as u8;
        self.pc += 2;
    }

    fn insn_8xye(&mut self) {
        // Shift Vx left and store MSB in Vf
        self.v[0xF] = self.v[bs8_usize(self.opcode)] >> 7;
        self.v[bs8_usize(self.opcode)] <<= 1;
        self.pc += 2;
    }

    fn insn_9xy0(&mut self) {
        // If Vx != Vy, skip next instruction
        if self.v[bs8_usize(self.opcode)] != self.v[bs4_usize(self.opcode)] {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    fn insn_annn(&mut self) {
        // Set index to NNN
        self.index = self.opcode & 0x0FFF;
        self.pc += 2;
    }

    fn insn_bnnn(&mut self) {
        // Jump to NNN + V0
        self.pc = (self.opcode & 0x0FFF) + self.v[0] as u16;
    }

    fn insn_cxnn(&mut self) {
        // Vx = Rand & NN
        let mut rng = rand::thread_rng();
        self.v[bs8_usize(self.opcode)] = (rng.gen::<u16>() & (self.opcode & 0x00FF)) as u8;
        self.pc += 2;
    }

    fn insn_dxyn(&mut self) {
        // Update the video memory
        let x: u16 = self.v[bs8_usize(self.opcode)] as u16;
        let y: u16 = self.v[bs4_usize(self.opcode)] as u16;
        let height: u16 = self.opcode & 0x000F;
        let mut pixel: u16;
        
        self.v[0xF] = 0;
        for yline in 0..height {
            pixel = self.memory[(self.index + yline) as usize] as u16;

            for xline in 0..8u16 {
                if (pixel & (0x80 >> xline)) != 0 {
                    let mut pos = (x + xline + ((y + yline) * 64)) as usize;
                    
                    // prevent drawing outside of valid locations
                    if pos >= 2048 {
                        pos = 2047;
                    }

                    if self.display[pos] == 1 {
                        self.v[0xF] = 1;
                    }
                    
                    self.display[pos] ^= 1;
                }
            }
        }

        self.draw_flag = true;
        self.pc += 2;
    }

    fn insn_ex9e(&mut self) {
        // Skip insn if key is pressed
        if self.keypad[self.v[bs8_usize(self.opcode)] as usize] != 0 {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    fn insn_exa1(&mut self) {
        // Skip insn if key is not pressed
        if self.keypad[self.v[bs8_usize(self.opcode)] as usize] == 0 {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    fn insn_fx07(&mut self) {
        // Set Vx to value of delay timer
        self.v[bs8_usize(self.opcode)] = self.delay_timer;
        self.pc += 2;
    }

    fn insn_fx0a(&mut self) {
        // await keypress
        let mut key_pressed = false;
        
        for i in 0..0xF {
            if self.keypad[i] != 0 {
                self.v[bs8_usize(self.opcode)] = i as u8;
                key_pressed = true;
            }
        }

        if key_pressed {
            self.pc += 2;
        }
    }

    fn insn_fx15(&mut self) {
        // Set the delay timer to Vx
        self.delay_timer = self.v[bs8_usize(self.opcode)];
        self.pc += 2;
    }

    fn insn_fx18(&mut self) {
        // Set the sound timer to Vx
        self.sound_timer = self.v[bs8_usize(self.opcode)];
        self.pc += 2;
    }

    fn insn_fx1e(&mut self) {
        // Adds Vx to I
        let result = self.index + self.v[bs8_usize(self.opcode)] as u16;
        self.index = result & 0x0FFF;
        self.v[0xF] = ((result & 0x1000) >> 12) as u8;
        self.pc += 2;
    }

    fn insn_fx29(&mut self) {
        // Set index to character sprite address
        self.index = self.v[bs8_usize(self.opcode)] as u16 * 0x05; //+ 0x50;
        self.pc += 2;
    }

    fn insn_fx33(&mut self) {
        // Store BCD representation of Vx in memory at I
        let v_index = ((self.opcode & 0x0F00) >> 8) as usize;
        self.memory[self.index as usize] = self.v[v_index] / 100;
        self.memory[self.index as usize + 1] = (self.v[v_index] / 10) % 10;
        self.memory[self.index as usize + 2] = (self.v[v_index] % 100) % 10;
        self.pc += 2;
    }

    fn insn_fx55(&mut self) {
        // Store registers to memory at I
        for i in 0..bs8_usize(self.opcode) {
            self.memory[self.index as usize + i] = self.v[i];
        }
        //self.index += ((self.opcode & 0x0F00) >> 8) + 1;
        self.pc += 2;
    }

    fn insn_fx65(&mut self) {
        // Load registers from memory at I
        for i in 0..bs8_usize(self.opcode) {
            self.v[i] = self.memory[self.index as usize + i];
        }
        //self.index += ((self.opcode & 0x0F00) >> 8) + 1;
        self.pc += 2;
    }   
}

fn bs8_usize(val: u16) -> usize {
    ((val & 0x0F00) >> 8) as usize
}

fn bs4_usize(val: u16) -> usize {
    ((val & 0x00F0) >> 4) as usize
}