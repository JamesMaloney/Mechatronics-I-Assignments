//Advanced Task 2

use rpi_embedded::uart::{Uart, Parity};
use std::{time::Duration, fs::OpenOptions, io::Write, process::Command, env};

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
    env::set_current_dir("lab-5-group-10/").unwrap();
    let mut file = OpenOptions::new().append(true).create(true).open("extra/log.csv").expect("Error creating/opening file");
    let mut commit_now = 0;
    loop {
        commit_now += 1;
        let light = uart.read_until(',').expect("Error reading from Arduino");
        let temp = uart.read_until(',').expect("Error reading from Arduino");
        let timestamp = chrono::offset::Utc::now();
        let mut date = timestamp.date().to_string();
        date.truncate(10);
        let mut time = timestamp.time().to_string();
        time.truncate(8);
        file.write(light.as_bytes()).unwrap();
        file.write(b",").unwrap();
        file.write(temp.as_bytes()).unwrap();
        file.write(b",").unwrap();
        file.write(date.as_bytes()).unwrap();
        file.write(b",").unwrap();
        file.write(time.as_bytes()).unwrap();
        file.write(b"\n").unwrap();
        if commit_now == 30 {
            Command::new("git").args(&["add", "."]).output().expect("Failed adding");
            let commit = String::from("Log update ") + &date + " " + &time;
            Command::new("git").args(&["commit", "-m", &commit]).output().expect("Failed committing");
            Command::new("git").args(&["push"]).output().expect("Failed pushing");
            commit_now = 0;
        }
    }
}