//! # test of DataGPS
//!
//! `DataGPS` is a collection of utilities to read GPS adafruit data.
//! Print of all important data of GPS adafruit like Altitude, Latitude, Longitude
//!
//! # Examples of getting data
//!
//! ```
//!
//! let x = DataGPS::new(Bauds::Baud9600);
//! x.read_all_data();
//!
//!
//! ```

#![allow(dead_code)]

use rust_gpiozero::LED;
use chrono::Utc;
use std::{fs::OpenOptions, io::Write, thread, time::Duration};
use rpi_embedded::uart::{Parity, Uart};
use nmea::Nmea;
extern crate nmea;
    /// Enum to be able to set the settings
pub enum Bauds{
    Baud9600,
    Baud57600,
    Baud115200
}

impl Bauds{
    pub fn get_value(&self) -> u32{
        let v: u32;
        match self{
            Bauds::Baud9600 => { v = 9600 }
            Bauds::Baud57600 => { v = 57600 }
            Bauds::Baud115200 => { v = 115200 }
        }
        v
    }
}
/// Struct of DataGPS for latitude, longitude, altitude, time and uart
pub struct DataGPS{ //$GPGGA
    uart: Uart,
    time:String,
    latitude:String,
    longitude:String,
    altitude:String,
    number_satellites: u32,
    hdop: f64
}
/// Implementation of functions for DataGPS
impl DataGPS{
    ///Create a new Uart connections and the object DataGPS
    pub fn new(baud: Bauds) -> Self{
        let mut _uart = Uart::new(baud.get_value(), Parity::None, 8, 1).expect("Error UART");
        _uart.set_read_mode(1, Duration::default()).expect("Error setting read mode");

        Self{
            uart: _uart,
            time: "".to_string(),
            latitude: "".to_string(),
            longitude: "".to_string(),
            altitude: "".to_string(),
            hdop: 0.0,
            number_satellites: 0
        }
    }
    ///Read the data from the GPS module and put in the values of the struct with a println
    pub fn read_all_data(&mut self) {
        let input = self.uart.read_line().expect("Error reading from Arduino");
        let mut nmea = Nmea::new();
        if input.contains("$GPGGA"){
            nmea.parse(&input).unwrap();
            let x : Vec<&str> = input.split(',').collect();
            let mut crash = false;

            for  i in 0..x.len()-2 {
                if x[i].to_string().is_empty(){
                    crash=true;
                    break;
                }
            }

            if !crash{
                self.set_data(x[1].to_string(), nmea.latitude.unwrap().to_string(), nmea.longitude.unwrap().to_string(), x[9].to_string()+x[10],
                x[8].to_string().parse::<f64>().unwrap(), x[7].to_string().parse::<u32>().unwrap());
            }else{
                self.set_no_signal();
            }
        }
    }
    // EXPERIMENTAL
    pub fn is_direction_ok(&mut self, destination: String) -> bool {
        //Check if GPS module has valid data, if not refresh it and exit function
        if let (Err(_a), Err(_b)) = (self.get_latitude().parse::<f64>(), self.get_longitude().parse::<f64>()) {
            self.read_all_data();
            return false;
        }
        let previous_lat = self.get_latitude().parse::<f64>().unwrap();
        let previous_lon = self.get_longitude().parse::<f64>().unwrap();
        while (previous_lat - self.get_latitude().parse::<f64>().unwrap()).abs() < 0.1 {
            self.read_all_data();
        }
        let coordinates = &destination.trim().split(", ").collect::<Vec<&str>>();
        //If this gives an error, the console input is malformed
        if let (Err(_a), Err(_b)) = (coordinates[0].parse::<f64>(), coordinates[1].parse::<f64>()) {
            println!("Input data not valid");
            return false;
        }
        //Some distances to calculate if I'm getting close to the destination or not
        let previous_distance_lat = coordinates[0].parse::<f64>().unwrap() - previous_lat;
        let current_distance_lat = coordinates[0].parse::<f64>().unwrap() - self.get_latitude().parse::<f64>().unwrap();
        let previous_distance_lon = coordinates[1].parse::<f64>().unwrap() - previous_lon;
        let current_distance_lon = coordinates[1].parse::<f64>().unwrap() - self.get_longitude().parse::<f64>().unwrap();
        println!("{}, {}, {}, {}", previous_distance_lat, current_distance_lat, previous_distance_lon, current_distance_lon);
        //Actual check which returns true or false
        if previous_distance_lat.abs() >= current_distance_lat.abs() || previous_distance_lon.abs() >= current_distance_lon.abs() {
            return true;
        }
        return false;
    }

    pub fn create_csv(&mut self){
        let mut file = OpenOptions::new().write(true).truncate(true).create(true).open("log.csv").expect("Error creating/opening file");
        file.write_all(b"Latitude,Longitude,Altitude,Date,Time\n").unwrap();
        self.read_all_data();
        let timestamp = chrono::offset::Utc::now();
        file.write_all(self.get_latitude().as_bytes()).unwrap();
        file.write_all(b",").unwrap();
        file.write_all(self.get_longitude().as_bytes()).unwrap();
        file.write_all(b",").unwrap();
        file.write_all(self.get_altitude().as_bytes()).unwrap();
        file.write_all(b",").unwrap();
        file.write_all(timestamp.date().to_string().as_bytes()).unwrap();
        file.write_all(b",").unwrap();
        file.write_all(timestamp.time().to_string().as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
        thread::sleep(Duration::from_millis(4500));
    }

    pub fn print_all_data(&mut self) {
        println!("Time: {:?}, Altitude: {:?}, Longitude: {:?}, Latitude: {:?}", &Utc::now().to_rfc2822()[..25], self.get_altitude(), self.get_longitude(), self.get_latitude());
    }

    pub fn check_signal(&mut self,led_warning: &mut LED){
        if self.number_satellites<3 && self.hdop>5.0{
            println!("Bad signal");
            led_warning.on();
            thread::sleep(Duration::from_millis(1000));
        }
        led_warning.off();
    }

    fn set_no_signal(&mut self){
        self.hdop = 20.0;
        self.number_satellites = 0;
    }

    ///Set the important data
    fn set_data(&mut self, time:String, latitude: String, longitude: String, altitude: String, hdop: f64, number_satellites: u32){
        self.time = time;
        self.latitude = latitude;
        self.longitude = longitude;
        self.altitude = altitude;
        self.hdop = hdop;
        self.number_satellites = number_satellites;

    }
    ///Return altitude if it is present
    pub fn get_altitude(&mut self) -> String{
        if self.altitude.is_empty(){
           "No altitude".to_string()
        }else{
           self.altitude.to_string()
        }
    }

/// # Examples of get_altitude
///
/// ```
///
/// let x = DataGPS::new(Bauds::Baud9600);
/// x.read_all_data();
/// println!("Altitude: {:?}",x.get_altitude());
///
///
/// ```

    ///Return latitude if it is present
    pub fn get_latitude(&mut self) -> String{
        if self.latitude.is_empty(){
            "No latitude".to_string()
        }else{
            self.latitude.to_string()
        }
    }
    ///Return time if it is present
    pub fn get_time(&mut self) -> String{
        if self.time.is_empty(){
            "No value".to_string()
        }else{
            self.time.to_string()
        }
    }
    ///Return longitude if it is present
    pub fn get_longitude(&mut self) -> String{
        if self.longitude.is_empty(){
            "No value".to_string()
        }else{
            self.longitude.to_string()
        }
    }
    ///Return baud rate
    pub fn get_baud_rate(&self) -> u32{
        self.uart.baud_rate()
    }
    ///Set baud rate of UART connection
    pub fn set_baud_rate(&mut self,baud_rate:&Bauds) {
        self.uart.set_baud_rate(baud_rate.get_value()).expect("Error setting baud rate");
    }
}