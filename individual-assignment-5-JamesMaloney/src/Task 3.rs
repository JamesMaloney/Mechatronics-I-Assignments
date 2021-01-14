use rpi_embedded::i2c::I2c;
use std::thread;
use std::time::Duration;

fn main() {
    let mut i2c = I2c::with_bus(1).unwrap();
    i2c.set_slave_address(0x1d).expect("Error setting the slave address");
    loop {
        i2c.write(&[0 as u8]).expect("Error writing to slave");
        thread::park_timeout(Duration::from_millis(10));
        i2c.write(&[1 as u8]).expect("Error writing to slave");
        thread::park_timeout(Duration::from_millis(10));
    }
}