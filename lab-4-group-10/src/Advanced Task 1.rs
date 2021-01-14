use rpi_embedded::{adxl::Adxl, servo::Servo};
use std::{thread, time::Duration};

fn start(accel: &mut Adxl){
    while accel.get_power_status() != 8 {
        accel.set_power_status(8);
        thread::sleep(Duration::from_millis(100));
    }
}

fn get_measurement(accel: &mut Adxl) {
    accel.get_data_raw();
    accel.get_data();
    accel.rotations();
}

fn get_angle(roll: f64) -> u8 {
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
        p if p < 0.0 => speed_millis = 100.0+pitch,
        _ => speed_millis = 100.0+pitch*index
    }
    if speed_millis > 200.0 {
        speed_millis = 200.0;
    }
    Duration::from_millis(speed_millis as u64)
}


fn slow_move(angle: u8, speed_time: Duration, servo: &mut Servo, previous_angle: &mut u8) {
    let millis = speed_time.as_millis() as u64;
    let step = millis/angle as u64;
    let y = previous_angle.clone();

    if angle > y {
        for x in y..angle {
            servo.write(x).unwrap();
            thread::sleep(Duration::from_millis(step));
        }
    }
    else {
        for x in angle..y {
            servo.write(x).unwrap();
            thread::sleep(Duration::from_millis(step));
        }
    }
}


fn main(){
    let mut first_accelerometer = Adxl::new_alt_adress(0x53);
    let mut second_accelerometer = Adxl::new_alt_adress(0x1D);
    let mut first_servo = Servo::new(1);
    let mut second_servo = Servo::new(0);
    let mut first_previous_angle = 0u8;
    let mut second_previous_angle = 0u8;

    start(&mut first_accelerometer);
    start(&mut second_accelerometer);

    loop {
        let first_angle=get_angle(first_accelerometer.roll);
        get_measurement(&mut first_accelerometer);
        slow_move(first_angle, get_speed(first_accelerometer.pitch),&mut first_servo, &mut first_previous_angle);
        get_measurement(&mut second_accelerometer);
        let second_angle=get_angle(second_accelerometer.roll);
        slow_move(second_angle, get_speed(second_accelerometer.pitch),&mut second_servo, &mut second_previous_angle);
        first_previous_angle=first_angle;
        second_previous_angle=second_angle;
    }
}