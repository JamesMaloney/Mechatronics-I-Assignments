use std::{thread, time::Duration};
use rpi_embedded::{pwm::Channel, pwm::Polarity, pwm::Pwm};

fn main() {
    let pwm = Pwm::with_frequency(Channel::Pwm0, 120.0, 0.0, Polarity::Normal, true).expect("Err");
    pwm.set_duty_cycle(0.7).expect("Err");
    thread::sleep(Duration::from_millis(1000));
    let mut duty: f64;
    pwm.set_duty_cycle(0.25).expect("Err");
    thread::sleep(Duration::from_millis(2000));
    for x in 25..100{
        duty = x as f64/100.0;
        println!("{}",duty);
        pwm.set_duty_cycle(duty).expect("Error ");
        thread::sleep(Duration::from_millis(100));
    }
    for x in 1..100{
        duty = 1.0 - x as f64/100.0;
        println!("{}",duty);
        pwm.set_duty_cycle(duty).expect("Error ");
        thread::sleep(Duration::from_millis(100));
    }
}
