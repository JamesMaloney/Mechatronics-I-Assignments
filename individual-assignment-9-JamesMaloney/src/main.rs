use std::{thread, time::Duration};
use run_script::ScriptOptions;

fn main() {
    let mut powersaving = false;
    loop {
        println!("Charging!");
        for _i in 1..30 {
            if read_from_bluetooth() {
                powersaving = !powersaving;
            }
            send_battery_percentage("up".to_string(), Duration::from_millis(500));
        }
        println!("Discharging!");
        for _i in (1..30).rev() {
            if read_from_bluetooth() {
                powersaving = !powersaving;
            }
            if powersaving {
                send_battery_percentage("down".to_string(), Duration::from_millis(1000));
            }
            else {
                send_battery_percentage("down".to_string(), Duration::from_millis(500));
            }
        }
    }
}

fn read_from_bluetooth() -> bool {
    let result = run_script::run(
        r#"find /dev -empty -name rfcomm0"#,
        &vec![],
        &ScriptOptions::new(),
    ).expect("Failed Bluetooth communication");
    return result.1 != "";
}

fn send_battery_percentage(percentage: String, timing: Duration) {
    run_script::run(
        &("echo \"".to_string() + &percentage + "\" > /dev/rfcomm0"),
        &vec![],
        &ScriptOptions::new(),
    ).expect("Failed Bluetooth communication");
    thread::sleep(timing);
}