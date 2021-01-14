use rpi_embedded::{pwm::{Pwm,Channel,Polarity}, servo::Servo, uart::Parity, uart::Uart};
use rust_gpiozero::Button;
use std::time::Duration;
use std::thread;
use rpi_embedded::gpio::Gpio;


fn main() {
    let mut uart = Uart::new(115_200, Parity::None, 8, 1).expect("Error creating UART communication");
    let mut pwm = Pwm::with_period(Channel::Pwm0, Duration::from_millis(50), Duration::from_millis(50), Polarity::Normal, true).unwrap();
    let mut servo= Servo::new(1);
    servo.write(180).expect("msg");
    let mut dir = Gpio::output(22).unwrap();
    let mut prev_state:u64=0;
    let mut position : i64 = 0;
    uart.set_read_mode(1, Duration::default()).unwrap();
    dir.set_high();
    loop {
        let string = uart.read_line().expect("Error reading from Arduino");
        let x: Vec<&str> = string.split(",").collect();
        //println!("POT UNO {}",x[0]);
        let v: Vec<&str> = x[1].split("\r").collect();
        if let Ok(y) = v[0].parse::<u64>(){
            println!("{}",y);
            if y > prev_state{
                dir.set_high();
                steps(&mut pwm);
                position=position+1;
            } else if y < prev_state{
                dir.set_low();
                steps(&mut pwm);
                position=position-1;
            }
        prev_state=y;
        }
        if let Ok(y) = x[0].parse::<u64>(){
            let angle= (y as f64/1023.0*180.0) as u8;
            //println!("{}",x);
            servo.write(angle).expect("msg");
        }

    }
}

fn steps(pwm:& mut Pwm){
    pwm.enable().unwrap();
    thread::sleep(Duration::from_nanos(10));
    pwm.disable().unwrap();
    thread::sleep(Duration::from_nanos(10));

}

fn stepper (){
    let pwm = Pwm::with_period(Channel::Pwm0, Duration::from_millis(100), Duration::from_millis(100), Polarity::Normal, true).unwrap();
    pwm.set_polarity(Polarity::Inverse).unwrap();
    let mut dir = Gpio::output(22).unwrap();
    dir.set_high();
    let mut right=0;
    let mut left=0;
    loop {
        if dir.is_set_high(){
            right= right+1;
        } else {
            left = left+1;
        }
        if right ==50 {
            dir.set_low();
        }
        pwm.enable().unwrap();
        thread::sleep(Duration::from_millis(100));
        pwm.disable().unwrap();
        thread::sleep(Duration::from_millis(100));
        println!("Right angle: {}",right);
        println!("Left angle : {}",left);

    }
}

fn read(){
    let mut uart = Uart::new(115_200, Parity::None, 8, 1).expect("Error creating UART communication");
    uart.set_read_mode(1, Duration::default()).unwrap();
    let mut input1;
    loop {
        input1 = uart.read_until(char::from(',')).expect("Error reading from Arduino");
        println!("input 1 : {}", input1);
    }
}