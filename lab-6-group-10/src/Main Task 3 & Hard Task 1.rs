use rpi_embedded::pwm::{Pwm,Channel,Polarity};
use std::time::Duration;
use std::thread;
use rpi_embedded::gpio::Gpio;


fn main() {
    let pwm = Pwm::with_period(Channel::Pwm0, Duration::from_millis(100), Duration::from_millis(100), Polarity::Normal, true).unwrap();
    let mut dir = Gpio::output(22).unwrap();
    dir.set_high();
    let mut angle = 0;
    loop {
        pwm.enable().unwrap();
        thread::sleep(Duration::from_millis(100));
        pwm.disable().unwrap();
        thread::sleep(Duration::from_millis(100));
        if dir.is_set_high() {
            angle += 18;
        }
        else {
            angle -= 18;
        }
        if angle == 900 || angle == 0 {
            if dir.is_set_high() {
                dir.set_low();
            }
            else {
                dir.set_high();
            }
        }
        println!("Angle: {}", (angle as f64 /10 as f64));
    }
}