use rpi_embedded::{pwm::{Pwm,Channel,Polarity}, uart::Parity, uart::Uart};
use std::time::Duration;
use std::thread;
use rpi_embedded::gpio::Gpio;

fn main() {
    let mut uart = Uart::new(115_200, Parity::None, 8, 1).expect("Error creating UART communication");
    let mut pwm = Pwm::with_period(Channel::Pwm0, Duration::from_millis(100), Duration::from_millis(50), Polarity::Normal, true).unwrap();
    let mut dir = Gpio::output(22).unwrap();
    let dc = Pwm::with_frequency(Channel::Pwm1, 120.0, 0.7, Polarity::Normal, true).expect("Err");
    let mut pot1;
    let mut pot2;
    let mut prev_state:u64=0;
    uart.set_read_mode(1, Duration::default()).unwrap();
    dir.set_high();
    thread::sleep(Duration::from_millis(1000));
    loop {
        pot1 = uart.read_until(char::from(',')).expect("Error reading from Arduino");
        pot2 = uart.read_until(char::from(',')).expect("Error reading from Arduino");
        if let Ok(x) = pot1.parse::<u64>(){
            if x > prev_state{
                dir.set_high();
                steps(&mut pwm);
            } else if x < prev_state{
                dir.set_low();
                steps(&mut pwm);
            }
            prev_state=x;
            }
    if let Ok(x) = pot2.parse::<f64>(){
        dc.set_duty_cycle(x/1000.0).expect("dury err ");
        }
}
}


fn steps(pwm:& mut Pwm){
    pwm.enable().unwrap();
    thread::sleep(Duration::from_nanos(50));
    pwm.disable().unwrap();
    thread::sleep(Duration::from_nanos(50));

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
    let mut input2;
    loop {
        input1 = uart.read_until(char::from(',')).expect("Error reading from Arduino");
        input2 = uart.read_until(char::from(',')).expect("Error reading from Arduino");
        println!("input 1 : {}", input1);
        println!("input 2 : {}", input2);
    }
}