use rust_gpiozero::*;
use std::{thread, time, panic};

enum Pin {
    Input(u8),
    Output(u8),
    Unused(u8)
}

struct PinType<T> {
    io: T,
    pin: Pin
}

impl<T> PinType<T>{
    fn create(io:T, pin: Pin) -> Self {
        PinType {
            io,
            pin
        }
    }

    fn read(&self) {
        match self.pin {
            Pin::Input(_) => { println!("Reading...")}
            Pin::Output(_) => { panic::catch_unwind(|| { panic!("ERROR: Cannot read from an output pin!")});}
            Pin::Unused(_) => { panic::catch_unwind(|| { panic!("ERROR: Cannot read from an unused pin!")});}
        }
    }

    fn write(&self) {
        match self.pin {
            Pin::Input(_) => { panic::catch_unwind(|| { panic!("ERROR: Cannot write to an input pin!")});}
            Pin::Output(_) => { println!("Writing...")}
            Pin::Unused(_) => { panic::catch_unwind(|| { panic!("ERROR: Cannot write to an unused pin!")});}
        }
    }
}

fn main() {
    let button = PinType::create(Button::new(17), Pin::Input(17));
    let stop_button = PinType::create(Button::new(21), Pin::Input(21));
    let mut led_single = PinType::create(LED::new(19), Pin::Output(19));
    let led_button = PinType::create(LED::new(20), Pin::Output(20));
    let unused = PinType::create(1, Pin::Unused(1));

    //simulate the panic from an incorrect read
    unused.read();

    let mut status = false;
    led_single.io.blink(2.0, 1.0);
    loop {
        if button.io.is_active() & !status {
            thread::sleep(time::Duration::from_micros(500));
            if button.io.is_active() {
                if !status {
                    println!("Trying to read Button:");
                    button.read();
                    println!("Trying to write Led:");
                    led_button.write();
                }
                status = true;
                led_button.io.on();
            }
        }
        if button.io.is_active()==false {
            if status {
                println!("Trying to write Led:");
                led_button.write();
                }
            status = false;
            led_button.io.off();
        }
        if stop_button.io.is_active() {
            panic!("Stop button pressed");
        }
    }
}