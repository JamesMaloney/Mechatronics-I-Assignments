use std::io::Write;
use std::io::stdout;
use std::io::stdin;
use std::{thread, time::Duration};

use rpi_embedded::i2c::I2c;

fn main() {
    let mut i2c = I2c::with_bus(1).unwrap();
    i2c.set_slave_address(0x1D).expect("slave adress failed");
    let mut input;
    let mut output: String;
    loop {
        input = String::from("");
        output = String::from("");
        print!("Enter a string: ");
        let _= stdout().flush();
        stdin().read_line(&mut input).expect("Please enter a correct string");
        for character in input.chars() {
            i2c.write(&[character as u8]).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
        let mut read_vector = vec![0; input.len()];
        i2c.read(&mut read_vector).expect("Err during reading");
        for character in read_vector.to_vec() {
            output.push(character as char);
        }
        println!("Received string: {}", output);
    }
}