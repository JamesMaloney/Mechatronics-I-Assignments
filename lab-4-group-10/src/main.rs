//Advanced Task 2
extern crate ads1x1x;
extern crate embedded_hal;
extern crate linux_embedded_hal;
#[macro_use(block)]
extern crate nb;
use ads1x1x::{Ads1x1x, SlaveAddr, channel};
use embedded_hal::adc::OneShot;
use linux_embedded_hal::I2cdev;
use rpi_embedded::{self, servo::Servo};
use rust_gpiozero::Button;

fn get_angle(value:i16)-> u8 {
    let mut v = 180.0/32752.0;
    v *= value as f64;
    v as u8
}


fn main() {
    let mut first_servo = Servo::new(0);
    let mut second_servo = Servo::new(1);
    let i2c = I2cdev::new("/dev/i2c-1").unwrap();
    let address = SlaveAddr::default();
    let mut adc = Ads1x1x::new_ads1115(i2c, address);
    let mut value: i16;
    let mut debounce = false;
    let button = Button::new(13);
    let mut rotated = false;

    loop {
        value = block!(adc.read(&mut channel::SingleA0)).unwrap();
        first_servo.write(get_angle(value)).expect("Servo error!");
        if button.is_active() && !debounce {
            if !rotated {
                rotated = true;
                second_servo.write(0).expect("Write error!");
            }
            else {
                rotated = false;
                second_servo.write(180).expect("Write error!");
            }
            debounce = true;
        }
        if !button.is_active() {
            debounce = false;
        }
    }
}