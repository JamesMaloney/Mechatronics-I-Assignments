use rpi_embedded::pwm::{Channel, Polarity, Pwm};
use std::thread;
use std::time::Duration;

const POSITION_0:u64 = 500;
const POSITION_90:u64 = 1500;
const POSITION_180:u64 = 2500;

fn set_pulse(pwm: &Pwm, pulse_micros: u64){
    let pulse= Duration::from_micros(pulse_micros);
    pwm.set_pulse_width(pulse).unwrap();
}

fn init(pwm: &Pwm) {
    set_pulse(&pwm, POSITION_0);
    thread::sleep(Duration::from_secs(2));
}


fn main(){
    let pwm = Pwm::with_frequency(Channel::Pwm0, 50.0, 0.00, Polarity::Normal, true).unwrap();
    init(&pwm);
    loop{
        set_pulse(&pwm, POSITION_0);
        thread::sleep(Duration::from_secs(1));
        set_pulse(&pwm, POSITION_90);
        thread::sleep(Duration::from_secs(1));
        set_pulse(&pwm, POSITION_180);
        thread::sleep(Duration::from_secs(1));
        set_pulse(&pwm, POSITION_90);
        thread::sleep(Duration::from_secs(1));
    }
}