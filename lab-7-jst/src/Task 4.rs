use std::{io::{Write, stdin, stdout}, time::Duration};

use nmea::Nmea;
use rpi_embedded::uart::{Parity, Uart};

fn main() {
    //64.12416166666667 , -21.9271
    let mut uart = Uart::new(9600, Parity::None, 8, 1).expect("Error UART");
    uart.set_read_mode(1, Duration::default()).expect("Error setting read mode");
    let mut input = String::new();
    print!("Enter coordinates: ");
    let _= stdout().flush();
    stdin().read_line(&mut input).expect("Please enter a string!");
    let split = &input.trim().split(", ").collect::<Vec<&str>>();
    let x1 = split[0].parse::<f64>().expect("First coordinate isn't valid!");
    let y1 = split[1].parse::<f64>().expect("Second coordinate isn't valid!");
    loop {
        let reading = uart.read_line().expect("Error reading from Arduino");
        let mut nmea = Nmea::new();
        if reading.contains("$GPGGA") {
            nmea.parse(&reading).unwrap();
            let data: Vec<&str> = reading.split(",").collect();
            let x2 = nmea.latitude.unwrap();
            let y2 = nmea.longitude.unwrap();
            //Haversine formula to calculate distance in metres
            let phi_1 = x1 * std::f64::consts::PI/180.0;
            let phi_2 = x2 * std::f64::consts::PI/180.0;
            let delta_phi = (x2 - x1) * std::f64::consts::PI/180.0;
            let delta_gamma = (y2 - y1) * std::f64::consts::PI/180.0;
            let a = (delta_phi/2.0).sin().powi(2) + phi_1.cos() * phi_2.cos() * (delta_gamma/2.0).sin().powi(2);
            let distance = 6371.0 * 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
            let mut direction: String = "".to_owned();
            if x1 > x2 {
                direction.push_str("north");
            }
            else {
                direction.push_str("south");
            }
            if y1 > y2 {
                direction.push_str("-east");
            }
            else {
                direction.push_str("-west");
            }
            println!("Your distance from the coordinates is {} meters towards {}", distance, direction);
        }
    }
}