use std::time::SystemTime;

#[link_section = ".key"]
pub static mut XOR_KEY: [u8; 132] = [0;132];

#[link_section = ".toxic"]
pub fn payload() {
    let start = SystemTime::now();

    println!("PAYLOAD EXECUTED AT {:?}", start);
}
