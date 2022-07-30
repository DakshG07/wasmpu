enum ValueType {
    i32,
    i64,
    f32,
    f64,
}


pub fn decode(i: &mut Vec<u8>) {
    //let mut cf: Vec<u8> = Vec::new(); // Control flow stack
    //let mut p: Vec<u8> = Vec::new(); // Program stack
    //let mut call: Vec<u8> = Vec::new(); // Call stack
    // First off, the begin WASM section
    let wasmv = &i[..4];
    println!("{:?}", wasmv);
    // verify the magic cookie
    if !(wasmv == [0, 0x61, 0x73, 0x6d]) {
       println!("SUS DETECTED")
    }
    i.drain(..4); // Your service is no longer needed.
    let mut typesec: Vec<Vec<ValueType>> = Vec::new(); // The type section
    let mut pos: i64 = 0; // Program position
    loop {
        // logic goes here...
    }
}    

