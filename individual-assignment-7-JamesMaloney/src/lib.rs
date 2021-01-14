mod datagps;

#[cfg(test)]
mod tests {
    use crate::datagps::DataGPS;
    use crate::datagps::Bauds;

    //Checks if updating uart to new value while executing works
    //by comparing the new value assigned and the current baud rate.
    #[test]
    fn set_baud_uart_test() {
        let mut data_gps = DataGPS::new(Bauds::Baud57600);
        let new_baud = Bauds::Baud9600;
        data_gps.set_baud_rate(&new_baud);
        assert_eq!(data_gps.get_baud_rate(),new_baud.get_value());
    }

    //Checks if the coordinates read are those that should be:
    //since I already know the right coordinates values by ohter means,
    //I can compare them to those I get from the GPS sensor.
    //The coordinates are checked ignoring the last two digits,
    //since these two fluctuate a lot and thus are not reliable.
    #[test]
    fn check_coordinates() {
        let mut data_gps = DataGPS::new(Bauds::Baud9600);
        while data_gps.get_latitude().trim() == "No value" {
            data_gps.read_all_data();
        }
        assert_eq!(&data_gps.get_latitude()[..7],"6407.45");
        assert_eq!(&data_gps.get_longitude()[..8],"02155.53");
    }

    //Checks the reading with an incorrect baudrate:
    //the GPS sensor sends data at 9600 baud, but we are reading it at 57600.
    //This should produce a panic, which is expected behavior and makes
    //the test pass.
    #[test]
    #[should_panic]
    fn uart_enabled(){
        let mut data_gps = DataGPS::new(Bauds::Baud57600);
        loop{
            data_gps.read_all_data();
        }
    }
}