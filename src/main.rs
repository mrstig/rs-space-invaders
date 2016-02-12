mod romloader;
mod disassembler;
mod cpu {
    pub mod cpu;
    pub mod state;
    pub mod condition_codes;
}

use cpu::cpu::CPU;

fn main() {
    let rom = romloader::load();

    let mut cpu: CPU = Default::default();
    cpu.load_rom(rom);
    cpu.emulate_op();

    // while pc < rom.len(){
    //     pc += disassembler::disassemble(&rom, pc);
    // }
    // println!("{:?}", &rom.len());
}
