use rust_gpiozero::LED;
use std::{thread, time::Duration};

/* fn main() {
    let img = image::open("gg.jpg").unwrap();
    println!("{:?}", img.to_bytes());
} */

//Hard Task 2
fn main() {
    let username = "JamesMaloney";
    let aiokey = "aio_OhFD768D8W0dfp2O2HVBe2usgUlB";
    let threshold = 6.0;
    let led = LED::new(18);
    let mut ada = adafruit_io_http::ada_io_http::AdaClient::set(username.to_string(), aiokey.to_string());
    loop {
        let temperature_feedkey = "temperature";
        let temperature = ada.get(temperature_feedkey.to_string());
        if temperature.parse::<f64>().unwrap() > threshold {
            led.on();
            println!("Warning: temperature above threshold!");
        }
        if temperature.parse::<f64>().unwrap() <= threshold {
            led.off();
        }
        thread::sleep(Duration::from_secs(5));
    }
}