use rpi_embedded::{pwm::{Pwm, Channel, Polarity}, uart::{Parity, Uart}};
use std::{time::Duration, thread};
use rpi_embedded::gpio::Gpio;


fn main() {
    let mut uart = Uart::new(115_200, Parity::None, 8, 1).expect("Error creating UART communication");
    let mut dir = Gpio::output(22).unwrap();
    let button = rust_gpiozero::Button::new(13);
    let dc = Pwm::with_frequency(Channel::Pwm1, 30.0, 0.0, Polarity::Normal, true).expect("Err");
    let mut pot;
    uart.set_read_mode(1, Duration::default()).unwrap();
    dir.set_high();
    let mut condition = true;
    let mut duty = 0.0;
    loop {
        pot = uart.read_until(char::from(',')).expect("Error reading from Arduino");
        if let Ok(x) = pot.parse::<f64>(){
            duty = x/1000.0;
            if duty > 1.0{
                duty = 1.0;
            }
        }
        if button.is_active() && condition {
            condition = false;
            dc.set_duty_cycle(duty).expect("dury err");
            thread::sleep(Duration::from_millis(100));
            dc.set_duty_cycle(0.0).expect("dury err");
        }
        if !button.is_active() {
            condition = true;
        }
    }
}
