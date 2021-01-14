use std::{thread, time::Duration};
use rust_gpiozero::LED;

fn main() {
    let username = "JamesMaloney";
    let aiokey = "aio_OhFD768D8W0dfp2O2HVBe2usgUlB";
    let mut ada = adafruit_io_http::ada_io_http::AdaClient::set(username.to_string(), aiokey.to_string());
    let led = LED::new(18);
    let mut data = 0;
    loop {
        let input_feedkey = "input";
        let trigger_feedkey = "trigger";
        ada.post(input_feedkey.to_string(), data.to_string());
        thread::sleep(Duration::from_secs(2));
        let data_new = ada.get(trigger_feedkey.to_string());
        if data_new.parse::<i8>().unwrap() == 1 {
            led.on();
        }
        if data_new.parse::<i8>().unwrap() == 0 {
            led.off();
        }
        println!("Trigger: {:}", data_new);
        thread::sleep(Duration::from_secs(2));
        data += 100;
    }
}