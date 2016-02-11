pub fn disassemble(code_buffer: &Vec<u8>, pc: usize) -> usize {
    let code = code_buffer[pc];
    let mut opbytes:usize = 1;
    print!("{:?}: ", pc);

    match code {
        0x00 => {print!("NOP");},
        0x01 => {print!("LXI"); opbytes = 3},
        0x02 => {print!("STAX   ");},
        0x03 => {print!("INX    ");},
        0x04 => {print!("INR    ");},
        0x05 => {print!("DCR    ");},
        0x06 => {print!("MVI    "); opbytes = 2},
        0x07 => {print!("RLC");},
        0x08 => {print!("NOP");},
        _ => print!("Something else")
    }

    println!("");
    return opbytes;
}
