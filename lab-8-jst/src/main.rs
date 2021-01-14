use adafruit_io_http;
use rpi_embedded::uart::{Uart, Parity};
use std::{error::Error, thread, time::Duration};

#[tokio::main]
async fn main(){
    let api_key: String = "GULdU6YfSVAbL0qG3YbSMx2zRSbGsfwT".to_string();
    let parsed;
    let mut uart = Uart::new(115_200, Parity::None, 8, 1).expect("Error creating UART communication");
    uart.set_read_mode(1, Duration::default()).unwrap();
    parsed = get_values(&api_key).await.unwrap();
    loop {
        let temp = parsed[0]["Temperature"]["Metric"]["Value"].to_string();
        let hum= parsed[0]["RelativeHumidity"].to_string();
        uart.write(temp).expect("Error writing to Arduino");
        uart.write(String::from(",")).expect("Error writing to Arduino");
        uart.write(hum).expect("Error writing to Arduino");
        uart.write(String::from(",")).expect("Error writing to Arduino");
        thread::sleep(Duration::from_secs(1));
    }
}

async fn get_values(api_key: &String) -> Result<json::JsonValue, Box<dyn Error>>{
    let mut url = "http://dataservice.accuweather.com/currentconditions/v1/190390?apikey=".to_string()+&api_key;
    url=url+&"&details=true".to_string();
    let body = reqwest::get(&url)
        .await?
        .text()
        .await?;
    let parsed = json::parse(&body).unwrap();
    Ok(parsed)
}

//Hard Task 1
fn upload_to_ada(humidity: String, temperature: String) {
    let username = "JamesMaloney";
    let aiokey = "aio_OhFD768D8W0dfp2O2HVBe2usgUlB";
    let mut ada = adafruit_io_http::ada_io_http::AdaClient::set(username.to_string(), aiokey.to_string());
    loop {
        let humidity_feedkey = "humidity";
        let temperature_feedkey = "temperature";
        ada.post(humidity_feedkey.to_string(), humidity.to_string());
        ada.post(temperature_feedkey.to_string(), temperature.to_string());
        thread::sleep(Duration::from_secs(5));
    }
}