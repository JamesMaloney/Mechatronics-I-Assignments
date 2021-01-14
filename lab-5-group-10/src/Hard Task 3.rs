use rpi_embedded::uart::{Uart, Parity};
use std::time::Duration;

fn main() {
    get_temperature(20.0, 30.0, 0.25);
}

fn get_temperature(first_threshold: f64, second_threshold: f64, tolerance: f64) {
    let mut uart = Uart::new(115_200, Parity::None, 8, 1).expect("Error creating UART communication");
    uart.set_read_mode(1, Duration::default()).unwrap();
    uart.write(first_threshold.to_string()).expect("Error writing to Arduino");
    uart.write(String::from(",")).expect("Error writing to Arduino");
    uart.write(second_threshold.to_string()).expect("Error writing to Arduino");
    uart.write(String::from(",")).expect("Error writing to Arduino");
    uart.write(tolerance.to_string()).expect("Error writing to Arduino");
    uart.write(String::from(",")).expect("Error writing to Arduino");
    let mut read;
    loop {
        read = uart.read_line().expect("Error reading from Arduino");
        println!("{}", read);
    }
}