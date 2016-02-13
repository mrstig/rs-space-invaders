use cpu::state::State;
use std::process;
use std::convert::From;

pub struct CPU {
    state: State
}

impl CPU {
    pub fn emulate_op(&mut self) {
        println!("PC:{} HEX:{:02X}", self.state.pc, self.state.memory[self.state.pc as usize]);

        let pc = self.state.pc as usize;

        let op_code = self.state.memory[pc];
        match op_code {
            0x00 => {}, //NOP
            0x06 => {
                    self.state.b = self.state.memory[pc + 1];
                    self.state.pc += 1;
            },
            0x07 => { //RLC
                self.state.a.rotate_left(1);
            },
            0x31 => { //LXI SP
                let low = self.state.memory[pc + 1];
                let high = self.state.memory[pc + 2];
                let combined = u16::from(high).rotate_left(8) | u16::from(low);
                self.state.sp = combined;
                self.state.pc += 2;
            },
            0xc3 => { //JMP
                let low = self.state.memory[pc + 1];
                let high = self.state.memory[pc + 2];
                let combined = u16::from(high).rotate_left(8) | u16::from(low);
                self.state.pc = combined;
                return; //Jump to address. Do not increment pc after.
            },
            0xcd => { //CALL
                let low = self.state.memory[pc + 1];
                let high = self.state.memory[pc + 2];
                let combined = u16::from(high).rotate_left(8) | u16::from(low);
                self.state.memory[self.state.sp as usize - 1] = high;
                self.state.memory[self.state.sp as usize - 2] = low;
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
