use rust_gpiozero::*;
use std::{thread, time::*, io::*};

pub struct LarsonScanner {
    led_list: Vec<LED>,
    led_colours: Vec<String>,
    led_number: u32,
    stop_button: Button,
    select_button: Button,
    toggle_button: Button,
    toggle_led: LED,
    speed: Duration,
    going_up: bool,
    state: i16
}

impl LarsonScanner {
    pub fn init() -> LarsonScanner {
        LarsonScanner {
            led_list: vec![LED::new(5), LED::new(6), LED::new(13), LED::new(19), LED::new(26)],
            led_colours: vec![String::from("red"), String::from("white"), String::from("green"), String::from("white"), String::from("red")],
            led_number: 5,
            stop_button: Button::new(21),
            select_button: Button::new(20),
            toggle_button: Button::new(27),
            toggle_led: LED::new(22),
            speed: Duration::from_millis(400),
            going_up: true,
            state: 1
        }
    }

    fn state_machine(&mut self) {
        let base: i16 = 2;
        if self.state == base.pow(self.led_number -1) || self.state == 1 {
            self.going_up = !self.going_up;
        }
        if self.going_up {
            self.state = self.state >> 1;
        }
        else {
            self.state = self.state << 1;
        }
        let mut binary = format!("{:b}", self.state);
        for led in self.led_list.iter().rev() {
            let character = binary.pop();
            match character {
                Some(c) => {
                    if c.to_digit(2).unwrap()==1 {
                        led.on();
                    }
                    else {
                        led.off();
                    }
                },
                None => led.off()
            }
        }
        thread::park_timeout(self.speed);
    }

    pub fn run(&mut self) {
        loop {
            if self.stop_button.is_active() {
                self.stop_lights();
            }
            if self.select_button.is_active() {
                self.change_speed();
            }
            if self.toggle_button.is_active() {
                self.toggle_lights();
            }
            self.state_machine();
        }
    }

    fn stop_lights (&mut self) {
        println!("The led on which you stopped is {}", self.led_colours[self.led_number as usize - format!("{:b}", self.state).len()]);
        let main_thread = thread::current();
        thread::spawn(move || loop {
            let mut input = String::new();
            print!("Do you want stop the program? [Y/n] ");
            let _= stdout().flush();
            stdin().read_line(&mut input).expect("The input you entered is invalid!");
            match input.trim().to_uppercase().as_str() {
                "Y" | "" => {
                    std::process::exit(0);
                },
                "N" => {
                    main_thread.unpark();
                    break;
                }
                _ => {
                    println!("The input you entered is invalid!");
                }
            }
        });
        thread::park();
    }

    fn change_speed (&mut self) {
        let selected_led = format!("{:b}", self.state).len();
        if self.led_number as usize / 2 + 1 == selected_led {
            self.speed /= 2;
        }
        if selected_led == self.led_number as usize || selected_led == 1 {
            self.stop_lights();
            self.speed *= 2;
        }
    }

    fn toggle_lights(&mut self) {
        for led in self.led_list.iter() {
            led.off();
        }
        self.toggle_led.on();
        thread::park_timeout(Duration::from_millis(250));
        self.toggle_button.wait_for_press(None);
        self.toggle_led.off();
    }
}