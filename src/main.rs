mod romloader;
mod disassembler;
mod cpu {
    pub mod cpu;
    pub mod state;
    pub mod condition_codes;
}

use cpu::cpu::CPU;
use std::process;


fn main() {
    let rom = romloader::load();
    let mut cpu: CPU = Default::default();

    cpu.load_rom(rom);

    let mut x = 0;
    loop {
        x +=1;
        if x>1545 {
            cpu.state.print();
        }
        if x>1555 {
            process::exit(1);
        }
        cpu.emulate_op();
    }
}
