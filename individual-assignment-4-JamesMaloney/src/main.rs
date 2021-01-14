use rpi_embedded::i2c::I2c;
use rpi_embedded::pwm::{Channel, Polarity, Pwm};
use std::panic;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

//Task 4
fn led_brightness() {
    let pwm = Pwm::with_frequency(Channel::Pwm0, 100.0, 0.0, Polarity::Normal, true).unwrap();
    for step in 1..100 {
        let mut duty_cycle = step as f64;
        duty_cycle /= 100 as f64;
        pwm.set_duty_cycle(duty_cycle).unwrap();
        thread::sleep(Duration::from_millis(50));
    }
    for step in (0..100).rev() {
        let mut duty_cycle = step as f64;
        duty_cycle /= 100 as f64;
        pwm.set_duty_cycle(duty_cycle).unwrap();
        thread::sleep(Duration::from_millis(50));
    }
}

//Task 5
fn i2c_id_read() {
    let mut i2c = I2c::new().expect("Couldn't initialize i2c device");
    i2c.set_slave_address(0x53).expect("Couldn't set slave address");
    let mut buffer = [0u8;1];
    i2c.block_read(0x00 as u8,&mut buffer).expect("Couldn't read block");
    println!("Accelerometer ID is {:?} ", buffer);
}

//Task 6
fn i2c_power_ctl_write() {
    let mut i2c = I2c::new().expect("Couldn't initialize i2c device");
    i2c.set_slave_address(0x53).expect("Couldn't set slave address");
    //Writes "wakeup" to power_ctl
    i2c.write(&[0x01 as u8]).expect("Couldn't write");
    i2c.cmd_write(0x2D as u8,0x08 as u8).expect("Couldn't write cmd");
    //Reads to check if value is now "8"
    let mut buffer = [0u8;1];
    i2c.block_read(0x2D, &mut buffer).expect("Couldn't read block");
    println!("POWER_CTL value is now {} (should be 8 if awake)", buffer[0]);
    //Tests the accelerometer functionality by getting some values
    let mut x_buffer = [0u8;6];
    let mut y_buffer = [0u8;6];
    let mut z_buffer = [0u8;6];
    loop {
        i2c.block_read(50, &mut x_buffer).expect("Couldn't read");
        i2c.block_read(52, &mut y_buffer).expect("Couldn't read");
        i2c.block_read(54, &mut z_buffer).expect("Couldn't read");
        println!("Accelerometer readings: X = {:03?}, Y = {:03?}, Z = {:03?}", x_buffer, y_buffer, z_buffer);
        thread::sleep(Duration::from_millis(500));
    }
}

//Task 7 and 8
fn accelerometer_controlled_led() {
    let mut i2c = I2c::new().expect("Couldn't initialize i2c device");
    i2c.set_slave_address(0x53).expect("Couldn't set slave address");
    //Writes "wakeup" to power_ctl
    i2c.write(&[0x01 as u8]).expect("Couldn't write");
    i2c.cmd_write(0x2D as u8,0x08 as u8).expect("Couldn't write cmd");
    //LED setup
    let pwm = Pwm::with_frequency(Channel::Pwm0, 100.0, 0.0, Polarity::Normal, true).unwrap();
    //Master-slave setup
    let (sender, receiver) = mpsc::channel();
    //Declares reading buffer
    let mut buffer = [0u8;6];

    //Sender (accelerometer input)
    thread::spawn(move || loop {
        //Uses X coordinates to control LED
        i2c.block_read(50, &mut buffer).expect("Couldn't read");
        sender.send(buffer).unwrap();
        thread::sleep(Duration::from_millis(500));
    });

    //Receiver (led output)
    loop {
        let received = receiver.recv().unwrap();
        pwm.set_duty_cycle(received[0] as f64 /255 as f64).unwrap();
        println!("Accelerometer readings: {:?}", received);
    }
}

//Task 9
fn main() {
    let mut i2c = I2c::new().expect("Couldn't initialize i2c device");
    i2c.set_slave_address(0x53).expect("Couldn't set slave address");
    //Writes "wakeup" to power_ctl
    i2c.write(&[0x01 as u8]).expect("Couldn't write");
    i2c.cmd_write(0x2D as u8,0x08 as u8).expect("Couldn't write cmd");
    //LED setup
    let pwm = Pwm::with_frequency(Channel::Pwm0, 100.0, 0.0, Polarity::Normal, true).unwrap();
    //Master-slave setup
    let (sender, receiver) = mpsc::channel();
    //Sets a timer to terminate threads
    let mut timer: f64 = 0.0;
    //Declares reading buffer
    let mut buffer = [0u8;6];

    //Receiver (led output)
    thread::Builder::new().name("LED thread".to_string()).spawn(move || loop {
        let received = receiver.recv().unwrap();
        pwm.set_duty_cycle(received as f64 /255 as f64).unwrap();
        timer += 0.5;
        //After 20 seconds panic thread
        if (timer - 20.0).abs() < 0.1 {
            assert!(panic::catch_unwind(|| { panic!("LED thread panic after {} seconds.", timer) }).is_err());
            break;
        }
    }).unwrap();

    //Sender (accelerometer input)
    loop {
        //Uses X coordinates to control LED
        i2c.block_read(50, &mut buffer).expect("Couldn't read");
        println!("Accelerometer readings: {:?}", buffer);
        sender.send(buffer[0]).ok();
        thread::sleep(Duration::from_millis(500));
        timer += 0.5;
        //After 30 seconds stop thread
        if (timer - 30.0).abs() < 0.1 {
            print!("I2C thread stopped after {} seconds", timer);
            std::process::exit(0);
        }
    }
}