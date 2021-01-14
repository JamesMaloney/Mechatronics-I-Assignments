use std::thread;
use std::time::Duration;
use rpi_embedded::{i2c::I2c};

fn main(){
    let mut i2c = I2c::new().expect("Couldn't initialize i2c device");
    i2c.set_slave_address(0x53).expect("Couldn't set slave address");
    //Writes "wakeup" to power_ctl
    i2c.write(&[0x01 as u8]).expect("Couldn't write");
    i2c.cmd_write(0x2D as u8,0x08 as u8).expect("Couldn't write cmd");
    //Reads to check if value is now "8"
    let mut buffer = [0u8;1];
    i2c.block_read(0x2D, &mut buffer).expect("Couldn't read block");
    println!("POWER_CTL value is now {} (should be 8 if awake)", buffer[0]);
    //Tests the accelerometer functionality by getting some values
    let mut buffer = [0u8;6];
    let mut data = [0i16;3];

    loop {
        i2c.block_read(50, &mut buffer).expect("Couldn't read");
        data[0] = (buffer[0] as u16 + (buffer[1] as u16).rotate_right(8)) as i16;
        data[1] = (buffer[2] as u16 + (buffer[3] as u16).rotate_right(8)) as i16;
        data[2] = (buffer[4] as u16 + (buffer[5] as u16).rotate_right(8)) as i16;

        println!("Accelerometer readings: {:?}", data);
        thread::sleep(Duration::from_millis(500));
    }
}