use std::fs::File;
use std::io::Read;
use std::process::exit;

pub fn load(path: &String) -> Vec<u8> {
    let mut f:File = match File::open(path) {
        Ok(f) => { f }
        Err(e) => { println!("Error opening rom file '{}': {}", path, e); exit(-1);}
    };
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).unwrap();
    return buffer;
}
