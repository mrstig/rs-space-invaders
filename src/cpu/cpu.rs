use cpu::state::State;
use std::process;
use std::convert::From;

pub struct CPU {
    state: State
}

impl CPU {
    pub fn emulate_op(&mut self) {
        // println!("PC:{} HEX:{:02X}", self.state.pc, self.state.memory[self.state.pc as usize]);

        let pc = self.state.pc as usize;

        let op_code = self.state.memory[pc];
        match op_code {
            0x00 => {}, //NOP
            0x05 => { //DCR B
                let x = self.state.b;
                self.state.b = self.deinrement_byte(x);
            },
            0x06 => { //MVI B
                self.state.b = self.state.memory[pc + 1];
                self.state.pc += 1;
            },
            0x11 => { //LXI D
                let byte_2 = self.state.memory[pc + 1];
                let byte_3 = self.state.memory[pc + 2];
                self.state.d = byte_3;
                self.state.e = byte_2;
                self.state.pc += 2;
            },
            0x13 => { //INX D
                self.state.e.wrapping_add(1);
                if self.state.e == 0 { //If e wraps around to 0, increment d
                    self.state.d.wrapping_add(1);
                }
            },
            0x1a => { //LDAX D
                let addr = u16::from(self.state.d).rotate_left(8) | u16::from(self.state.e);
                self.state.a = self.state.memory[addr as usize];
            },
            0x21 => { //LXI H
                let byte_2 = self.state.memory[pc + 1];
                let byte_3 = self.state.memory[pc + 2];
                self.state.h = byte_3;
                self.state.l = byte_2;
                self.state.pc += 2;
            },
            0x23 => { //INX H
                self.state.l.wrapping_add(1);
                if self.state.l == 0 { //If l wraps around to 0, increment h
                    self.state.h.wrapping_add(1);
                }
            },
            0x31 => { //LXI SP
                let byte_2 = self.state.memory[pc + 1];
                let byte_3 = self.state.memory[pc + 2];
                let combined = u16::from(byte_3).rotate_left(8) | u16::from(byte_2);
                self.state.sp = combined;
                self.state.pc += 2;
            },
            0x32 => { //STA addr
                let byte_2 = self.state.memory[pc + 1];
                let byte_3 = self.state.memory[pc + 2];
                let addr = u16::from(byte_3).rotate_left(8) | u16::from(byte_2);
                self.state.memory[addr as usize] = self.state.a;
            },
            0x77 => { //MOV M, A
                let addr = u16::from(self.state.h).rotate_left(8) | u16::from(self.state.l);
                self.state.memory[addr as usize] = self.state.a;
            },
            0xc2 => { //JNZ
                if !self.state.cc.z {
                    self.jump();
                    return;
                }
            },
            0xc3 => { //JMP
                self.jump();
                return; //Jump to address. Do not increment pc after.
            },
            0xc9 => { //RET
                let low = self.state.memory[self.state.sp as usize];
                let high = self.state.memory[self.state.sp as usize + 1];
                let combined = u16::from(high).rotate_left(8) | u16::from(low);
                self.state.sp += 2;
                self.state.pc = combined;
                return;
            },
            0xcd => { //CALL
                let byte_2 = self.state.memory[pc + 1];
                let byte_3 = self.state.memory[pc + 2];
                let combined = u16::from(byte_3).rotate_left(8) | u16::from(byte_2);
                self.state.memory[self.state.sp as usize - 1] = byte_3;
                self.state.memory[self.state.sp as usize - 2] = byte_2;
                self.state.sp -= 2;
                self.state.pc = combined;
                return;
            },
            _ => self.unimplemented_instruction()
        }
        self.state.pc += 1;
    }

    fn unimplemented_instruction(&self) {
        println!("Unimplemented instruction found. PC:{} HEX:{:02X}", self.state.pc, self.state.memory[self.state.pc as usize]);
        process::exit(1);
    }

    fn deinrement_byte(&mut self, mut x: u8) -> u8 {
        x = x.wrapping_sub(1);
        self.state.cc.z = x == 0;
        self.state.cc.s = 0x80 == (x & 0x80);
        self.state.cc.p = x%2 == 0;
        // self.state.cc.ac -- Not used by space invaders
        return x;
    }

    fn jump(&mut self) {
        let byte_2 = self.state.memory[self.state.pc as usize + 1];
        let byte_3 = self.state.memory[self.state.pc as usize + 2];
        let combined = u16::from(byte_3).rotate_left(8) | u16::from(byte_2);
        self.state.pc = combined;
        return; //Jump to address. Do not increment pc after.
    }


    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.state.memory = rom;
        self.state.memory.resize(65536, 0);
    }
}

impl Default for CPU {
    fn default() -> CPU {
        return CPU {
            state: Default::default()
        };
    }
}
