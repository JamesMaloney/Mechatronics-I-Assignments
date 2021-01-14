//Task 10
use rpi_embedded::uart::{Uart, Parity};
use std::time::Duration;

fn main(){
    let mut uart = Uart::new(115_200, Parity::None, 8, 1).expect("Error creating UART communication");
    uart.set_read_mode(1, Duration::default()).unwrap();
    loop {
        let read_string = uart.read_line().expect("Error reading bytes from UART");
        for character in read_string.chars() {
            character as u32;
            if character != '\r' {
                println!("\"{}\"", character);
            }
        }
        println!();
    }
}