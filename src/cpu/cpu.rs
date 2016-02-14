use cpu::state::State;
use std::process;
use std::convert::From;

pub struct CPU {
    pub state: State
}

impl CPU {
    pub fn emulate_op(&mut self) {
        // println!("PC:{} HEX:{:02X}", self.state.pc, self.state.memory[self.state.pc as usize]);

        let pc = self.state.pc as usize;

        let op_code = self.state.memory[pc];
        match op_code {
            0x00 => {}, //NOP
            0x01 => { //LXI B
                let byte_2 = self.state.memory[pc + 1];
                let byte_3 = self.state.memory[pc + 2];
                self.state.b = byte_3;
                self.state.c = byte_2;
                self.state.pc += 2;
            },
            0x05 => { //DCR B
                let x = self.state.b;
                self.state.b = self.decrement_byte(x);
            },
            0x06 => { //MVI B
                self.state.b = self.state.memory[pc + 1];
                self.state.pc += 1;
            },
            0x0d => { //DCR C
                let x = self.state.c;
                self.state.c = self.decrement_byte(x);
            },
            0x0e => { //MVI C
                self.state.c = self.state.memory[pc + 1];
                self.state.pc += 1;
            },
            0x0f => { //RRC
                let x = self.state.a;
                self.state.cc.cy = 1 == (x&1);
                self.state.a = x.rotate_right(1);
            },
            0x11 => { //LXI D
                let byte_2 = self.state.memory[pc + 1];
                let byte_3 = self.state.memory[pc + 2];
                self.state.d = byte_3;
                self.state.e = byte_2;
                self.state.pc += 2;
            },
            0x13 => { //INX D
                self.state.e = self.state.e.wrapping_add(1);
                if self.state.e == 0 { //If e wraps around to 0, increment d
                    self.state.d = self.state.d.wrapping_add(1);
                }
            },
            0x1a => { //LDAX D
                self.state.a = self.mem_at_addr(self.de());
            },
            0x21 => { //LXI H
                let byte_2 = self.state.memory[pc + 1];
                let byte_3 = self.state.memory[pc + 2];
                self.state.h = byte_3;
                self.state.l = byte_2;
                self.state.pc += 2;
            },
            0x23 => { //INX H
                self.state.l = self.state.l.wrapping_add(1);
                if self.state.l == 0 { //If l wraps around to 0, increment h
                    self.state.h = self.state.h.wrapping_add(1);
                }
            },
            0x31 => { //LXI SP
                let addr = self.pc_addr();
                self.state.sp = addr;
                self.state.pc += 2;
            },
            0x32 => { //STA addr
                let addr = self.pc_addr();
                self.state.memory[addr as usize] = self.state.a;
                self.state.pc += 2;
            },
            0x40 => { //MOV B, B
                self.state.b = self.state.b;
            },
            0x41 => { //MOV B, C
                self.state.b = self.state.c;
            },
            0x42 => { //MOV B, D
                self.state.b = self.state.d;
            },
            0x43 => { //MOV B, E
                self.state.b = self.state.e;
            },
            0x44 => { //MOV B, H
                self.state.b = self.state.h;
            },
            0x45 => { //MOV B, L
                self.state.b = self.state.l;
            },
            0x46 => { //MOV B, M
                self.state.b = self.mem_at_addr(self.hl());
            },
            0x47 => { //MOV B, A
                self.state.b = self.state.a;
            },
            0x48 => { //MOV C, B
                self.state.c = self.state.b;
            },
            0x49 => { //MOV C, C
                self.state.c = self.state.c;
            },
            0x4a => { //MOV C, D
                self.state.c = self.state.d;
            },
            0x4b => { //MOV C, E
                self.state.c = self.state.e;
            },
            0x4c => { //MOV C, H
                self.state.c = self.state.h;
            },
            0x4d => { //MOV C, L
                self.state.c = self.state.l;
            },
            0x4e => { //MOV C, M
                self.state.c = self.mem_at_addr(self.hl());
            },
            0x4f => { //MOV C, A
                self.state.c = self.state.a;
            },
            0x50 => { //MOV D, B
                self.state.d = self.state.b;
            },
            0x51 => { //MOV D, C
                self.state.d = self.state.c;
            },
            0x52 => { //MOV D, D
                self.state.d = self.state.d;
            },
            0x53 => { //MOV D, E
                self.state.d = self.state.e;
            },
            0x54 => { //MOV D, H
                self.state.d = self.state.h;
            },
            0x55 => { //MOV D, L
                self.state.d = self.state.l;
            },
            0x56 => { //MOV D, M
                self.state.d = self.mem_at_addr(self.hl());
            },
            0x57 => { //MOV D, A
                self.state.d = self.state.a;
            },
            0x58 => { //MOV E, B
                self.state.e = self.state.b;
            },
            0x59 => { //MOV E, C
                self.state.e = self.state.c;
            },
            0x5a => { //MOV E, D
                self.state.e = self.state.d;
            },
            0x5b => { //MOV E, E
                self.state.e = self.state.e;
            },
            0x5c => { //MOV E, H
                self.state.e = self.state.h;
            },
            0x5d => { //MOV E, L
                self.state.e = self.state.l;
            },
            0x5e => { //MOV E, M
                self.state.e = self.mem_at_addr(self.hl());
            },
            0x5f => { //MOV E, A
                self.state.e = self.state.a;
            },
            0x60 => { //MOV H, B
                self.state.h = self.state.b;
            },
            0x61 => { //MOV H, C
                self.state.h = self.state.c;
            },
            0x62 => { //MOV H, D
                self.state.h = self.state.d;
            },
            0x63 => { //MOV H, E
                self.state.h = self.state.e;
            },
            0x64 => { //MOV H, H
                self.state.h = self.state.h;
            },
            0x65 => { //MOV H, L
                self.state.h = self.state.l;
            },
            0x66 => { //MOV H, M
                self.state.h = self.mem_at_addr(self.hl());
            },
            0x67 => { //MOV H, A
                self.state.h = self.state.a;
            },
            0x68 => { //MOV L, B
                self.state.l = self.state.b;
            },
            0x69 => { //MOV L, C
                self.state.l = self.state.c;
            },
            0x6a => { //MOV L, D
                self.state.l = self.state.d;
            },
            0x6b => { //MOV L, E
                self.state.l = self.state.e;
            },
            0x6c => { //MOV L, H
                self.state.l = self.state.h;
            },
            0x6d => { //MOV L, L
                self.state.l = self.state.l;
            },
            0x6e => { //MOV L, M
                self.state.l = self.mem_at_addr(self.hl());
            },
            0x6f => { //MOV L, A
                self.state.l = self.state.a;
            },
            0x77 => { //MOV M, A
                let addr = self.hl();
                self.state.memory[addr as usize] = self.state.a;
            },
            0x7a => { //MOV A, D
                self.state.a = self.state.d;
            },
            0x7b => { //MOV A, E
                self.state.a = self.state.e;
            },
            0x7c => { //MOV A, H
                self.state.a = self.state.h;
            },
            0x7d => { //MOV A, L
                self.state.a = self.state.l;
            },
            0x7e => { //MOV A, M
                self.state.a = self.mem_at_addr(self.hl());
            },
            0xc2 => { //JNZ
                if !self.state.cc.z {
                    self.jump();
                    return;
                }
                self.state.pc += 2;
            },
            0xc3 => { //JMP
                self.jump();
                return; //Jump to address. Do not increment pc after.
            },
            0xc9 => { //RET
                let low = self.state.memory[self.state.sp as usize];
                let high = self.state.memory[self.state.sp as usize + 1];
                let combined = u16::from(high).rotate_left(8) | u16::from(low);
                println!("{:02X}, {:02X}, {:02X}, {:04X}", self.state.sp, low, high, combined);
                self.state.pc = combined;
                self.state.sp += 2;
                return;
            },
            0xcd => { //CALL Uhm. Don't actually understand what happens here
                let ret = pc+3;
                let byte_2 = self.state.memory[pc + 1];
                let byte_3 = self.state.memory[pc + 2];
                let combined = u16::from(byte_3).rotate_left(8) | u16::from(byte_2);
                self.state.memory[self.state.sp as usize - 1] = ((ret >> 8) & 0xFF) as u8;
                self.state.memory[self.state.sp as usize - 2] = (ret & 0xFF) as u8;
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

    fn decrement_byte(&mut self, mut x: u8) -> u8 {
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

    fn pc_addr(&self) -> u16 {
        let byte_2 = self.state.memory[self.state.pc as usize + 1];
        let byte_3 = self.state.memory[self.state.pc as usize + 2];
        return u16::from(byte_3).rotate_left(8) | u16::from(byte_2);
     }

    fn bc(&self) -> u16 {
        return u16::from(self.state.b).rotate_left(8) | u16::from(self.state.c);
    }

    fn de(&self) -> u16 {
        return u16::from(self.state.d).rotate_left(8) | u16::from(self.state.e);
    }

    fn hl(&self) -> u16 {
        return u16::from(self.state.h).rotate_left(8) | u16::from(self.state.l);
    }

    fn mem_at_addr(&self, addr: u16) -> u8 {
        return self.state.memory[addr as usize];
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
