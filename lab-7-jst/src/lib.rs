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

mod datagps;

#[cfg(test)]
mod tests {
    use crate::datagps::DataGPS;
    use crate::datagps::Bauds;
///test for baud rate of UART connection
    #[test]
    fn set_baud_uart_test() {
        let mut data_gps = DataGPS::new(Bauds::Baud57600);
        let new_baud = Bauds::Baud9600;
        data_gps.set_baud_rate(&new_baud);
        assert_eq!(data_gps.get_baud_rate(),new_baud.get_value());
    }
///test the coordinates of input
    #[test]
    #[should_panic]
    fn uart_test(){
        let mut data_gps = DataGPS::new(Bauds::Baud57600);
        loop{
            data_gps.read_all_data();
            assert_eq!(data_gps.get_latitude(),"6407.4482N");
            assert_eq!(data_gps.get_longitude(),"02155.5337W");
        }
    }
}