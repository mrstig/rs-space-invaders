extern crate rustc_serialize;
extern crate docopt;


mod romloader;
mod disassembler;
mod cpu {
    pub mod cpu;
    pub mod state;
    pub mod condition_codes;
}

use cpu::cpu::CPU;
use std::process;
use docopt::Docopt;
const USAGE: &'static str = "
Ate zero

Usage:
  atezero [--rom <path_to_rom>]

Options:
  -h --help             Show this screen.
  --version             Show version.
  --rom=<path_to_rom>   Path to ROM to execute [default: mem.rom]
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_rom: String,
}

fn main() {

    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    
    let rom = romloader::load(&args.flag_rom);
    let mut cpu: CPU = Default::default();

    cpu.load_rom(rom);

    let mut x = 0;
    loop {
        x +=1;
        if x>1545 {
            // cpu.state.print();
        }
        if x>1555 {
            process::exit(1);
        }
        cpu.emulate_op();
    }
}
