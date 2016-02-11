mod romloader;
mod disassembler;

fn main() {
    let rom = romloader::load();
    let mut pc:usize = 0;
    while pc < rom.len(){
        pc += disassembler::disassemble(&rom, pc);
    }
    println!("{:?}", &rom.len());
}
