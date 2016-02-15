use cpu::state::State;
use std::process;
use std::convert::From;

pub struct CPU {
    pub state: State
}

impl CPU {
    pub fn emulate_op(&mut self) {
        let pc = self.state.pc as usize;
        let op_code: u8 = self.state.memory[pc];

        let xx = op_code >> 6 & 0b11;
        let yyy = op_code >> 3 & 0b111;
        let zzz = op_code & 0b111;

        match xx {
            0b00 => {
                self.emulate_op_00(yyy, zzz);
            },
            0b01 => {
                self.mov(zzz, yyy);
            },
            _ => self.unimplemented_instruction()
        }
    }

    pub fn emulate_op_00(&mut self, yyy: u8, zzz: u8) {
        match zzz {
            0b000 => {
                if yyy == 0b000 { //NOP
                    self.increase_pc(1);
                } else {
                    panic!("Undefined op!");
                }
            },
            0b001 => {
                if yyy & 0b001 == 1 {
                    self.lxi(yyy >> 1 & 0b011);
                } else {
                    // self.dad(yyy);
                }
            },
            _ => self.unimplemented_instruction()
        }
    }

    fn lxi(&mut self, register_pair_reference: u8) {
        let pc = self.state.pc as usize;
        let byte_2 = self.state.memory[pc + 1];
        let byte_3 = self.state.memory[pc + 2];

        match register_pair_reference {
            0b00 => { //BC
                self.state.b = byte_3;
                self.state.c = byte_2;
            },
            0b01 => { //DE
                self.state.d = byte_3;
                self.state.e = byte_2;
            },
            0b10 => { //HL
                self.state.h = byte_3;
                self.state.l = byte_2;
            },
            0b11 => { //SP
                panic!("NYS - LXI SP")
                // self.state.b = byte_3;
                // self.state.c = byte_2;
            },
            _ => panic!("Illegal Pair Reference {:02X}", register_pair_reference)
        }
        self.increase_pc(3);
    }

    pub fn mov(&mut self, source: u8, destination: u8) {
        let val = self.get_val(source);
        self.set_val(destination, val);
        self.increase_pc(1);
    }

    fn get_val(&mut self, source: u8) -> u8{
        match source {
            0b000 => return self.state.b,
            0b001 => return self.state.c,
            0b010 => return self.state.d,
            0b011 => return self.state.e,
            0b100 => return self.state.h,
            0b101 => return self.state.l,
            0b110 => {
                let addr = self.hl();
                return self.get_mem_at_addr(addr)
            },
            0b111 => return self.state.a,
            _ => panic!("Invalid reference: {}", source)
        }
    }

    fn set_val(&mut self, destination: u8, val: u8) {
        match destination {
            0b000 => self.state.b = val,
            0b001 => self.state.c = val,
            0b010 => self.state.d = val,
            0b011 => self.state.e = val,
            0b100 => self.state.h = val,
            0b101 => self.state.l = val,
            0b110 => {
                let addr = self.hl(); //
                self.set_mem_at_addr(addr, val)
            },
            0b111 => self.state.a = val,
            _ => panic!("Invalid reference: {}", destination)
        }
    }

    fn increase_pc (&mut self, inc: u16) {
        self.state.pc += inc;
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

    fn hl(&mut self) -> u16 {
        return u16::from(self.state.h).rotate_left(8) | u16::from(self.state.l);
    }

    fn get_mem_at_addr(&self, addr: u16) -> u8 {
        return self.state.memory[addr as usize];
    }

    fn set_mem_at_addr(&mut self, addr: u16, val: u8) {
        self.state.memory[addr as usize] = val;
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


// just keep this while refactoring
// pub fn emulate_op(&mut self) {
//         // println!("PC:{} HEX:{:02X}", self.state.pc, self.state.memory[self.state.pc as usize]);
//
//         let pc = self.state.pc as usize;
//
//         let op_code = self.state.memory[pc];
//         match op_code {
//             0x00 => {}, //NOP
//             0x01 => { //LXI B
//                 let byte_2 = self.state.memory[pc + 1];
//                 let byte_3 = self.state.memory[pc + 2];
//                 self.state.b = byte_3;
//                 self.state.c = byte_2;
//                 self.state.pc += 2;
//             },
//             0x05 => { //DCR B
//                 let x = self.state.b;
//                 self.state.b = self.decrement_byte(x);
//             },
//             0x06 => { //MVI B
//                 self.state.b = self.state.memory[pc + 1];
//                 self.state.pc += 1;
//             },
//             0x0d => { //DCR C
//                 let x = self.state.c;
//                 self.state.c = self.decrement_byte(x);
//             },
//             0x0e => { //MVI C
//                 self.state.c = self.state.memory[pc + 1];
//                 self.state.pc += 1;
//             },
//             0x0f => { //RRC
//                 let x = self.state.a;
//                 self.state.cc.cy = 1 == (x&1);
//                 self.state.a = x.rotate_right(1);
//             },
//             0x11 => { //LXI D
//                 let byte_2 = self.state.memory[pc + 1];
//                 let byte_3 = self.state.memory[pc + 2];
//                 self.state.d = byte_3;
//                 self.state.e = byte_2;
//                 self.state.pc += 2;
//             },
//             0x13 => { //INX D
//                 self.state.e = self.state.e.wrapping_add(1);
//                 if self.state.e == 0 { //If e wraps around to 0, increment d
//                     self.state.d = self.state.d.wrapping_add(1);
//                 }
//             },
//             0x1a => { //LDAX D
//                 self.state.a = self.mem_at_addr(self.de());
//             },
//             0x21 => { //LXI H
//                 let byte_2 = self.state.memory[pc + 1];
//                 let byte_3 = self.state.memory[pc + 2];
//                 self.state.h = byte_3;
//                 self.state.l = byte_2;
//                 self.state.pc += 2;
//             },
//             0x23 => { //INX H
//                 self.state.l = self.state.l.wrapping_add(1);
//                 if self.state.l == 0 { //If l wraps around to 0, increment h
//                     self.state.h = self.state.h.wrapping_add(1);
//                 }
//             },
//             0x31 => { //LXI SP
//                 let addr = self.pc_addr();
//                 self.state.sp = addr;
//                 self.state.pc += 2;
//             },
//             0x32 => { //STA addr
//                 let addr = self.pc_addr();
//                 self.state.memory[addr as usize] = self.state.a;
//                 self.state.pc += 2;
//             },
//             0x7e => { //MOV A, M
//                 self.state.a = self.mem_at_addr(self.hl());
//             },
//             0xc2 => { //JNZ
//                 if !self.state.cc.z {
//                     self.jump();
//                     return;
//                 }
//                 self.state.pc += 2;
//             },
//             0xc3 => { //JMP
//                 self.jump();
//                 return; //Jump to address. Do not increment pc after.
//             },
//             0xc9 => { //RET
//                 let low = self.state.memory[self.state.sp as usize];
//                 let high = self.state.memory[self.state.sp as usize + 1];
//                 let combined = u16::from(high).rotate_left(8) | u16::from(low);
//                 println!("{:02X}, {:02X}, {:02X}, {:04X}", self.state.sp, low, high, combined);
//                 self.state.pc = combined;
//                 self.state.sp += 2;
//                 return;
//             },
//             0xcd => { //CALL Uhm. Don't actually understand what happens here
//             let ret = pc+3;
//             let byte_2 = self.state.memory[pc + 1];
//             let byte_3 = self.state.memory[pc + 2];
//             let combined = u16::from(byte_3).rotate_left(8) | u16::from(byte_2);
//             self.state.memory[self.state.sp as usize - 1] = ((ret >> 8) & 0xFF) as u8;
//             self.state.memory[self.state.sp as usize - 2] = (ret & 0xFF) as u8;
//             self.state.sp -= 2;
//             self.state.pc = combined;
//             return;
//         },
//         _ => self.unimplemented_instruction()
//     }
//     self.state.pc += 1;
// }
