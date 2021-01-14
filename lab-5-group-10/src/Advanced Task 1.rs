use rpi_embedded::uart::{Uart, Parity};
use std::{time::Duration, fs::OpenOptions, io::Write};

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
    let mut light;
    let mut temp;
    let mut file = OpenOptions::new().append(true).create(true).open("log.csv").expect("Error creating/opening file");
    file.write(b"Light,Temperature,Date,Time\n").unwrap();
    loop {
        light = uart.read_until(',').expect("Error reading from Arduino");
        temp = uart.read_until(',').expect("Error reading from Arduino");
        let timestamp = chrono::offset::Utc::now();
        file.write(light.as_bytes()).unwrap();
        file.write(b",").unwrap();
        file.write(temp.as_bytes()).unwrap();
        file.write(b",").unwrap();
        file.write(timestamp.date().to_string().as_bytes()).unwrap();
        file.write(b",").unwrap();
        file.write(timestamp.time().to_string().as_bytes()).unwrap();
        file.write(b"\n").unwrap();
    }
}