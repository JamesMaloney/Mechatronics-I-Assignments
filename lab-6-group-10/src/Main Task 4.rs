use rpi_embedded::{uart::Parity, uart::Uart};
use std::time::Duration;
fn main() {
    let mut uart = Uart::new(115_200, Parity::None, 8, 1).expect("Error creating UART communication");
    uart.set_read_mode(1, Duration::default()).unwrap();
    let mut first_potentiometer;
    let mut second_potentiometer;
    loop {
        first_potentiometer = uart.read_until(char::from(',')).expect("Error reading from Arduino");
        second_potentiometer = uart.read_until(char::from(',')).expect("Error reading from Arduino");
        println!("First potentiometer: {}", first_potentiometer);
        println!("Second potentiometer: {}", second_potentiometer);
    }
}