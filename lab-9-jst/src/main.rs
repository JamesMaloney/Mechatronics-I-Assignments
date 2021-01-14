use rpi_embedded::uart::{Parity, Uart};
use rust_gpiozero::{Button, LED};
use std::{error::Error, thread, time::Duration};

#[allow(dead_code)]
fn mt3() {
    let mut uart= Uart::with_path("/dev/rfcomm0", 115_200, Parity::None, 8, 1).expect("Uart failed");
    uart.set_read_mode(1, Duration::default()).unwrap();
    loop{
        let input = uart.read_line().expect("Error reading from Arduino");
        println!("{:?}",input.replace("\r", ""));
        uart.write("Matteo".to_string()).expect("Error writing to Arduino");
    }
}

#[tokio::main]
async fn mt4() {
    let threshold = 3.0;
    let tolerance = 5.0;

    let red = LED::new(12);
    let yellow = LED::new(25);
    let green = LED::new(24);
    let button = Button::new(18);

    let mut uart= Uart::with_path("/dev/rfcomm0", 115_200, Parity::None, 8, 1).expect("Uart failed");
    uart.set_read_mode(1, Duration::default()).unwrap();

    let api_key: String = "GULdU6YfSVAbL0qG3YbSMx2zRSbGsfwT".to_string();
    let parsed = get_values(&api_key).await.unwrap();
    let temperature = parsed[0]["Temperature"]["Metric"]["Value"].to_string();
    println!("Temperature: {}", temperature);

    if temperature.parse::<f32>().unwrap() >= threshold + tolerance {
        red.on();
        yellow.off();
        green.off();
        uart.write("red".to_string()).expect("Error writing to Arduino");
    }
    else if temperature.parse::<f32>().unwrap() < threshold - tolerance {
        red.off();
        yellow.off();
        green.on();
        uart.write("green".to_string()).expect("Error writing to Arduino");
    }
    else {
        red.off();
        yellow.on();
        green.off();
        uart.write("yellow".to_string()).expect("Error writing to Arduino");
    }
    let mut was_lit = &red;
    loop {
        let input = uart.read_until('!').expect("Error reading from Arduino");
        println!("Read: {:?}",input);
        if button.is_active() || input.contains("Power-saving") {
            thread::sleep(Duration::from_micros(500));
            if button.is_active() || input.contains("Power-saving") {
                if red.is_lit() {
                    red.off();
                    was_lit = &red;
                }
                else if yellow.is_lit() {
                    yellow.off();
                    was_lit = &yellow;
                }
                else if green.is_lit() {
                    green.off();
                    was_lit = &green;
                }
                else {
                    was_lit.on();
                }
            }
            thread::sleep(Duration::from_millis(500));
        }
    }
}


async fn get_values(api_key: &String) -> Result<json::JsonValue, Box<dyn Error>> {
    let mut url = "http://dataservice.accuweather.com/currentconditions/v1/190390?apikey=".to_string();
    url += &api_key;
    url += &"&details=true".to_string();
    let body = reqwest::get(&url).await?.text().await?;
    let parsed = json::parse(&body).unwrap();
    Ok(parsed)
}



#[tokio::main]
async fn main(){
    let mut threshold = 21.0;
    let tolerance = 5.0;

    let red = LED::new(12);
    let yellow = LED::new(25);
    let green = LED::new(24);

    let mut uart= Uart::with_path("/dev/rfcomm0", 115_200, Parity::None, 8, 1).expect("Uart failed");
    uart.set_read_mode(1, Duration::default()).unwrap();

    let api_key: String = "GULdU6YfSVAbL0qG3YbSMx2zRSbGsfwT".to_string();
    let parsed = get_values(&api_key).await.unwrap();
    uart.write("STATUS: Api call for forecast".to_string()).expect("Error writing to Arduino");
    let temperature = parsed[0]["Temperature"]["Metric"]["Value"].to_string();
    println!("Temperature: {}", temperature);
    thread::sleep(Duration::from_secs(1));

    let mut was_lit = &red;
    loop {
        if temperature.parse::<f32>().unwrap() >= threshold + tolerance {
            red.on();
            yellow.off();
            green.off();
            uart.write("COLOR: red".to_string()).expect("Error writing to Arduino");
        }
        else if temperature.parse::<f32>().unwrap() < threshold - tolerance {
            red.off();
            yellow.off();
            green.on();
            uart.write("COLOR: green".to_string()).expect("Error writing to Arduino");
        }
        else {
            red.off();
            yellow.on();
            green.off();
            uart.write("COLOR: yellow".to_string()).expect("Error writing to Arduino");
        }

        let input = uart.read_until('!').expect("Error reading from Arduino");
        if input.contains("THRESHOLD:"){
            threshold = input.replace("THRESHOLD: ", "").replace("!", "").parse::<f32>().unwrap();
            println!("Read: {:?}",input);
        }

        if input.contains("DANCE"){
            red.on();
            thread::sleep(Duration::from_millis(100));
            yellow.on();
            thread::sleep(Duration::from_millis(100));
            green.on();
            thread::sleep(Duration::from_millis(100));
            red.off();
            thread::sleep(Duration::from_millis(100));
            yellow.off();
            thread::sleep(Duration::from_millis(100));
            green.off();
            thread::sleep(Duration::from_millis(100));
            println!("Read: {:?}",input);
        }
        if input.contains("ONOFF"){
            red.on();
            yellow.on();
            green.on();
            thread::sleep(Duration::from_millis(100));
            red.off();
            yellow.off();
            green.off();
            thread::sleep(Duration::from_millis(100));
            red.on();
            yellow.on();
            green.on();
            thread::sleep(Duration::from_millis(100));
            red.off();
            yellow.off();
            green.off();
            thread::sleep(Duration::from_millis(100));
            red.on();
            yellow.on();
            green.on();
            thread::sleep(Duration::from_millis(100));
            red.off();
            yellow.off();
            green.off();
            thread::sleep(Duration::from_millis(100));
            println!("Read: {:?}",input);
        }

        if input.contains("Power-saving") {
            uart.write("STATUS: Power saving changed".to_string()).expect("Error writing to Arduino");
            if red.is_lit() {
                red.off();
                was_lit = &red;
            }
            else if yellow.is_lit() {
                yellow.off();
                was_lit = &yellow;
            }
            else if green.is_lit() {
                green.off();
                was_lit = &green;
            }
            else {
                was_lit.on();
            }
        }
    }
}