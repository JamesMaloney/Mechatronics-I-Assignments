use std::thread;
use std::time::Duration;
use rpi_embedded::servo;

fn slow_move(angle: u8, speed_time: Duration) {
    let mut servo = servo::Servo::new(0);
    let millis = speed_time.as_millis() as u64;
    let step = millis/angle as u64;

    for x in 0..angle {
        servo.write(x).unwrap();
        thread::sleep(Duration::from_millis(step));
    }
}
fn main(){
    slow_move(90, Duration::from_secs(10));
}