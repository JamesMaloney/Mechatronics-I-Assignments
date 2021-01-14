use std::{time::Duration};
use rpi_embedded::uart::{Parity, Uart};

///Defines and implements some Baud rates, which are the most common while communicating using uart.
pub enum Bauds {
    Baud9600,
    Baud57600,
    Baud115200
}

impl Bauds{
    pub fn get_value(&self) -> u32 {
        let v: u32;

        match self {
            Bauds::Baud9600 => { v = 9600 }
            Bauds::Baud57600 => { v = 57600 }
            Bauds::Baud115200 => { v = 115200 }
        }

        v
    }
}

pub struct DataGPS { //$GPGGA
    uart: Uart,
    time:String,
    latitude:String,
    longitude:String,
    altitude:String,
}

impl DataGPS {
    /// Initializes a new uart communication using the specified settings (baud rate) and the default ones (Parity, data_bits, stop_bits).
    /// After opening the connection, the Struct holding the receivable values is also initialized.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut connection = DataGPS::new(Bauds::Baud9600);
    /// let reading = connection.read_all_data();
    ///
    /// assert_ne!("", reading);
    /// ```
    pub fn new(baud: Bauds) -> Self {
        let mut _uart = Uart::new(baud.get_value(), Parity::None, 8, 1).expect("Error UART");
        _uart.set_read_mode(1, Duration::default()).expect("Error setting read mode");
        Self {
            uart: _uart,
            time: "".to_string(),
            latitude: "".to_string(),
            longitude: "".to_string(),
            altitude: "".to_string()
        }
    }

    /// Reads all the data at once from the uart, splits it in single information and puts it in the corresponding Struct fields.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut connection = DataGPS::new(Bauds::Baud9600);
    /// let reading = connection.read_all_data();
    ///
    /// assert_ne!("", reading);
    /// ```
    pub fn read_all_data(&mut self) {
        let input = self.uart.read_line().expect("Error reading from Arduino");
        if input.contains("$GPGGA") {
            let x : Vec<&str> = input.split(",").collect();
            self.set_data(x[0].to_string(), x[2].to_string()+x[3], x[4].to_string()+x[5], x[9].to_string()+x[10])
        }

    }

    /// Puts received data in the Struct
    fn set_data(&mut self, time:String, latitude: String, longitude: String, altitude: String) {
        self.time = time;
        self.latitude = latitude;
        self.longitude = longitude;
        self.altitude = altitude;

    }

    ///Returns altitude
    pub fn get_altitude(&mut self) -> String {
        if self.altitude.is_empty() {
           "No value".to_string()
        }
        else {
           self.altitude.to_string()
        }
    }

    ///Returns latitude
    pub fn get_latitude(&mut self) -> String {
            if self.latitude.is_empty() {
                "No value".to_string()
            }
            else {
                self.latitude.to_string()
            }
    }
    
    ///Returns time
    pub fn get_time(&mut self) -> String {
            if self.time.is_empty() {
                "No value".to_string()
            }
            else {
                self.time.to_string()
            }
    }

    ///Returns longitude
    pub fn get_longitude(&mut self) -> String {
            if self.longitude.is_empty() {
                "No value".to_string()
            }
            else {
                self.longitude.to_string()
            }
    }

    ///Returns baud rate
    pub fn get_baud_rate(&self) -> u32 {
        self.uart.baud_rate()
    }

    ///Returns uart status
    pub fn get_uart_status(&self) -> rpi_embedded::uart::Status {
        self.uart.status().expect("Error getting status")
    }

    ///Sets the new baud rate if modified during execution
    pub fn set_baud_rate(&mut self,baud_rate:&Bauds) {
        self.uart.set_baud_rate(baud_rate.get_value()).expect("Error setting baud rate");
    }
}
