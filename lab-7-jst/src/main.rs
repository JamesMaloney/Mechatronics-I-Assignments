mod datagps;
use std::io::{Write, stdin, stdout};
use crate::datagps::*;
use rust_gpiozero::LED;

fn main(){
    let mut x = DataGPS::new(Bauds::Baud9600);
    let direction_led = LED::new(23);
    let direction = get_string_from_console();
    //Grab destination coordinates from console reading (format: "69.420, -69.420")
    loop {
        x.create_csv();
        if x.is_direction_ok(direction.to_owned()) {
            //End up here if direction is right
            direction_led.on();
        } else {
            //End up here if wrong direction, malformed GPS or input data
            direction_led.off();
        }
    }
}

fn get_string_from_console() -> String {
    let mut input = String::new();
    print!("Enter input: ");
    let _= stdout().flush();
    stdin().read_line(&mut input).expect("Please enter a valid string!");
    return input;
}
