use std::fs::File;
use std::io::Read;

pub fn load() -> Vec<u8> {
    let mut f = File::open("D:/Development/rs-space-invaders/res/invaders.rom").unwrap();
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).unwrap();
    return buffer;
}
