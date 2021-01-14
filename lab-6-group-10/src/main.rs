use std::thread;
use std::time::Duration;
use rpi_embedded::{i2c::I2c, servo::Servo};

fn close(left_servo:&mut Servo, right_servo:&mut Servo){
    right_servo.write(180).expect("Error writing");
    left_servo.write(0).expect("Error writing");
}

fn open(left_servo:&mut Servo, right_servo:&mut Servo){
    right_servo.write(135).expect("Error writing");
    left_servo.write(45).expect("Error writing");
}

fn main(){
    let mut left_servo = Servo::new(0);
    let mut right_servo = Servo::new(1);
    let mut i2c = I2c::new().expect("i2c failed in initilazation");
    i2c.set_slave_address(0x53).expect("slave adress failed");

    let mut buffer = [0u8;1];
    buffer[0]=8;
    i2c.block_write(0x2D ,&mut buffer).expect("Failure in write CMD");

    let mut raw_data = [0u8;6];
    let mut data = [0i16;3];
    let mut pitch:f64;

    let mut previous_status = false;
    close(&mut left_servo, &mut right_servo);

    loop{
        i2c.block_read(50,&mut raw_data).expect("READING RAW DATA FAILED");
        data[0] = (raw_data[0] as u16  +(raw_data[1] as u16).rotate_right(8)) as i16;
        data[1] = (raw_data[2] as u16  +(raw_data[3] as u16).rotate_right(8)) as i16;
        data[2] = (raw_data[4] as u16  +(raw_data[5] as u16).rotate_right(8)) as i16;

        let x = data[0] as f64;
        let y = data[1] as f64;
        let z = data[2] as f64;

        pitch = -x.atan2((y*y + z*z).sqrt())*57.3;

        if pitch >= 25.0 && previous_status==true{
            close(&mut left_servo, &mut right_servo);
            previous_status=false;
        }else if pitch <= -25.0 && previous_status==false{
            open(&mut left_servo, &mut right_servo);
            previous_status=true;
        }
        println!("{}",pitch);
        thread::sleep(Duration::from_millis(200));
    }
}