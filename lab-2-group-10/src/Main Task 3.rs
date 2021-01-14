use rust_gpiozero::*;
use std::{thread, time};

fn main() {
    let button = Button::new(17);
    let led = LED::new(20);
    let mut status = false;
    loop {
        if button.is_active() & status==false {
            thread::sleep(time::Duration::from_micros(500));
            if button.is_active() {
                if !status {
                    println!("Led is on");
                }
                status = true;
                led.on();
            }
        }
        if button.is_active()==false {
            if status {
                println!("Led is off");
            }
            status = false;
            led.off();
        }
    }
}