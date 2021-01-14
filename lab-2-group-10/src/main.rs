use rust_gpiozero::*;
use std::{thread, time};

enum Pin<T> {
    Input(T, u8),
    Output(T, u8),
    Unused(u8),
}

fn main() {
    let button_type = Pin::Input(Button::new(17), 17);
    let led_single_type = Pin::Output(LED::new(19), 19);
    let led_button_type = Pin::Output(LED::new(20), 20);
    let mut status = false;
    if let Pin::Output(mut led_single, number) = led_single_type {
        led_single.blink(1.0, 1.0);
    }
    if let Pin::Input(button, number) = button_type {
        if let Pin::Output(led_button, number) = led_button_type {
            loop {
                if button.is_active() & status==false {
                    thread::sleep(time::Duration::from_micros(500));
                    if button.is_active() {
                        if !status {
                            println!("Led is on");
                        }
                        status = true;
                        led_button.on();
                    }
                }
                if !button.is_active() {
                    if status {
                        println!("Led is off");
                    }
                    status = false;
                    led_button.off();
                }
            }
        }
    }
}