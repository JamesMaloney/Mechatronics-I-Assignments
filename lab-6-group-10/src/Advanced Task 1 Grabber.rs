use std::{io::Write, io::stdin, io::stdout, thread};
use std::time::Duration;
use rpi_embedded::servo::Servo;

fn main(){
    let mut left_servo = Servo::new(0);
    let mut right_servo = Servo::new(1);
    open(&mut left_servo, &mut right_servo);
    let mut input;
    loop {
        input = String::from("");
    print!("Enter a string: ");
    let _= stdout().flush();
    stdin().read_line(&mut input).expect("Please enter a correct string");
    if input.trim() == "o" {
        open(&mut left_servo, &mut right_servo);
        thread::sleep(Duration::from_millis(500));
    }
    if input.trim() == "c" {
        close(&mut left_servo, &mut right_servo);
        thread::sleep(Duration::from_millis(500));
    }
    }
}

fn open(left_servo: &mut Servo, right_servo: &mut Servo) {
    left_servo.write(45 as u8).unwrap();
    right_servo.write(135 as u8).unwrap();
}

fn close(left_servo: &mut Servo, right_servo: &mut Servo) {
    left_servo.write(0 as u8).unwrap();
    right_servo.write(180 as u8).unwrap();
}