use rpi_embedded::{pwm::{Pwm,Channel,Polarity}, uart::Parity, uart::Uart};
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::time::Duration;
use std::thread;
use rpi_embedded::gpio::Gpio;


fn main() {
    let mut uart = Uart::new(115_200, Parity::None, 8, 1).expect("Error creating UART communication");
    let mut pwm = Pwm::with_period(Channel::Pwm0, Duration::from_millis(100), Duration::from_millis(50), Polarity::Normal, true).unwrap();
    let mut dir = Gpio::output(22).unwrap();
    pwm.set_polarity(Polarity::Inverse).expect("msg");
    let mut pot1;
    let mut prev_state:u64=0;
    let mut position: i64;
    uart.set_read_mode(1, Duration::default()).unwrap();
    dir.set_high();
    pwm.set_polarity(Polarity::Normal).expect("msg");
    let mut file = OpenOptions::new()
        .read(true)
        .open("position")
        .unwrap();
    let mut position_string = String::new();
    file.read_to_string(&mut position_string).expect("Unable to read the file");
    position = position_string.parse::<i64>().unwrap();
    if position<0{
        dir.set_high();
    } else {
        dir.set_low()
    }
    for _ in 0..position.abs() {
        steps(&mut pwm);
        thread::sleep(Duration::from_millis(10));
    }
    position=0;
    loop {
        pot1 = uart.read_until(char::from(',')).expect("Error reading from Arduino");
        if let Ok(x) = pot1.parse::<u64>(){
            if x > prev_state{
                dir.set_high();
                steps(&mut pwm);
                position=position+1;
            } else if x < prev_state{
                dir.set_low();
                steps(&mut pwm);
                position=position-1;
            }
            prev_state=x;
            write_file(position);
        }
    }
}

fn write_file(position: i64) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("position")
        .unwrap();
    file.write(position.to_string().as_bytes()).expect("Error writing to file");
}

fn steps(pwm:& mut Pwm){
    pwm.enable().unwrap();
    thread::sleep(Duration::from_nanos(50));
    pwm.disable().unwrap();
    thread::sleep(Duration::from_nanos(50));
}