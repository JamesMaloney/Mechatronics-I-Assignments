use std::{thread};
use std::time::Duration;

use rpi_embedded::i2c::I2c;

fn main(){
    let mut i2c = I2c::with_bus(1).expect("I2C initialization failed");
    i2c.set_slave_address(0x1d).expect("Slave address setup failed");
    let mut read_buffer = [0u8; 1];
    read_buffer[0] = 0;
    loop {
        i2c.write(&[read_buffer[0]]).expect("I2C write failed");
        i2c.read(&mut read_buffer).expect("I2C read failed");
        println!("{}", read_buffer[0]);
        thread::sleep(Duration::from_secs(1));
        read_buffer[0] += 10;
    }
}