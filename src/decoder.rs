enum ValueType {
    I32,
    I64,
    F32,
    F64,
}
// Sections
struct Section {
    name: String, // Name of section
    opcode: u8,   // Section opcode
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
    let sections: Vec<Section> = Vec::from([
        Section {name: "Type".to_string(), opcode: 0x01}
    ]);
    // Check for sections
    let opcodes = sections.iter().map(|x| x.opcode).collect::<Vec<u8>>();
    while opcodes.contains(&i[0]) {
        // Handle sections
        if i[0] == 1 {
            // Type section
        }
    }
    loop {
        // logic goes here...
        
    }
} 

