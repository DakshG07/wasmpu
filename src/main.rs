use std::io::Read;
use std::io::BufReader;
use std::fs::File;
mod decoder;
use decoder::decode;

fn main() {
    // Open file
    let f = File::open("test.wasm").expect("Unable to open file.");
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();
    // Read into vector
    reader.read_to_end(&mut buffer);
    decode(&mut buffer)
}


