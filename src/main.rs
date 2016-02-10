mod romloader;
mod disassemler;

fn main() {
    let rom = romloader::load();
    let mut pc:usize = 0;
    while pc < rom.len(){
        pc += disassemler::disassemble(&rom, pc);
    }
    println!("{:?}", &rom.len());
}
