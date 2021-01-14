use std::thread;
use std::time::Duration;
use rpi_embedded::{i2c::I2c, servo::Servo};

fn main(){
    let mut servo = Servo::new(0);
    let mut i2c = I2c::new().expect("Couldn't initialize i2c device");
    i2c.set_slave_address(0x53).expect("Couldn't set slave address");
    //Writes "wakeup" to power_ctl
    i2c.write(&[0x01 as u8]).expect("Couldn't write");
    i2c.cmd_write(0x2D as u8,0x08 as u8).expect("Couldn't write cmd");

    let mut raw_data = [0u8;2];
    let mut data;

    loop{
        i2c.block_read(52, &mut raw_data).expect("Couldn't read");
        data = (raw_data[0] + raw_data[1]).rotate_right(8);
        println!("Y: {:?}", data);
        servo.write(data as u8).unwrap();
        thread::sleep(Duration::from_millis(1000));
    }
}