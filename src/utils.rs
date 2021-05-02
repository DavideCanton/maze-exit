use std::io::{stdin, Read};

pub fn wait_for_enter() {
    let mut stdin = stdin();

    println!("Press enter to continue...");
    stdin.read(&mut [0u8]).unwrap();
}
