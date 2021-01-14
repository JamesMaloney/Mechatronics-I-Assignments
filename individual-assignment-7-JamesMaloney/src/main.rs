mod datagps;
use std::{thread, time::Duration};
use crate::datagps::*;

fn main(){
    let mut reading = DataGPS::new(Bauds::Baud9600);
    loop {
        reading.read_all_data();
        println!("{}", reading.get_altitude());
        thread::sleep(Duration::from_secs(1));
    }
}
