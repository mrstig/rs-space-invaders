pub fn disassemble(code_buffer: &Vec<u8>, pc: usize) -> usize {
    let code = code_buffer[pc];
    let mut opbytes:usize = 1;
    print!("{:?}: ", pc);

    match code {
        0x00 => {print!("NOP");},
        0x01 => {print!("LXI B, {:x}{:x}", code_buffer[pc +2], code_buffer[pc +1]); opbytes = 3},
        0x02 => {print!("STAX B");},
        0x03 => {print!("INX B");},
        0x04 => {print!("INR B");},
        0x05 => {print!("DCR B");},
        0x06 => {print!("MVI B, {:x}", code_buffer[pc + 1]); opbytes = 2},
        0x07 => {print!("RLC");},
        0x08 => {print!("NOP");},
        0x09 => {print!("DAD B");},
        0x0a => {print!("LDAX B");},
        0x0b => {print!("DCX B");},
        0x0c => {print!("INR C");},
        0x0d => {print!("DCR C");},
        0x0e => {print!("MVI C"); opbytes = 2},
        0x0f => {print!("RRC");},
        0x10 => {print!("NOP");},
        0x31 => {print!("LXI SP, {:02X}{:02X}", code_buffer[pc + 2], code_buffer[pc + 1]);  opbytes = 3;},
        0xc3 => {print!("JMP adr, {:02X}{:02X}", code_buffer[pc + 2], code_buffer[pc + 1]); opbytes = 3},
        _ => print!("Something else")
    }

    println!("");
    return opbytes;
}
