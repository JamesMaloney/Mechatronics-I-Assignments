use std::thread;
use std::time::Duration;
use rpi_embedded::{i2c::I2c, servo::Servo};

fn get_angle(roll: f64) -> u8{
    let mut angle: f64;
    match roll {
        r if r == 0.0 => angle = 90.0,
        _ => angle = 90.0+roll/2.0
    }
    if angle > 180.0 {
        angle = 180.0;
    }
    angle as u8
}

fn get_speed(pitch: f64) -> Duration {
    let mut speed_millis: f64;
    let index:f64 = 100.0/90.0;
    match pitch {
        p if p == 0.0 => speed_millis = 100.0,
        p if p < 0.0 => speed_millis = 100.0 + pitch,
        _ => speed_millis = 100.0+pitch*index
    }
    if speed_millis > 200.0 {
        speed_millis = 200.0;
    }
    Duration::from_millis(speed_millis as u64)
}


fn main(){
    let mut servo = Servo::new(0);
    let mut i2c = I2c::new().expect("i2c failed in initilazation");
    i2c.set_slave_address(0x53).expect("slave adress failed");

    let mut buffer = [0u8;1];
    buffer[0]=8;
    i2c.block_write(0x2D ,&mut buffer).expect("Failure in write CMD");

    let mut raw_data = [0u8;6];
    let mut data = [0i16;3];
    let mut roll: f64;
    let mut pitch:f64;

    loop{
        i2c.block_read(50,&mut raw_data).expect("READING RAW DATA FAILED");
        data[0] = (raw_data[0] as u16  +(raw_data[1] as u16).rotate_right(8)) as i16;
        data[1] = (raw_data[2] as u16  +(raw_data[3] as u16).rotate_right(8)) as i16;
        data[2] = (raw_data[4] as u16  +(raw_data[5] as u16).rotate_right(8)) as i16;

        let x = data[0] as f64;
        let y = data[1] as f64;
        let z = data[2] as f64;

        roll = y.atan2(z)*57.3;
        pitch = -x.atan2((y*y + z*z).sqrt())*57.3;

        let angle = get_angle(roll);
        let speed = get_speed(pitch);
        println!("Angle: {}; speed: {}", angle, speed.as_millis());
        servo.write(angle).unwrap();
        thread::sleep(speed);
    }
}