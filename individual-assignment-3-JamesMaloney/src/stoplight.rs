use std::{thread, time::Duration};
use rust_gpiozero::*;

pub struct Stoplight {
    pub red: bool,
    pub yellow: bool,
    pub green: bool,
    pub yellow_wait: Duration,
    pub max_wait: Duration,
    pub state: State,
    red_pin: LED,
    yellow_pin: LED,
    green_pin: LED,
}

pub enum State {
    Red,
    RedYellow,
    Yellow,
    Green
}

pub enum Region {
    EU,
    US
}

impl Stoplight{
    pub fn init(max_wait: Duration) -> Stoplight {
        Stoplight{
            red: true,
            yellow: false,
            green: false,
            yellow_wait: Duration::new(3, 0),
            max_wait,
            state: State::Red,
            red_pin: LED::new(4),
            yellow_pin: LED::new(6),
            green_pin: LED::new(20),
        }
    }

    fn state_machine(&mut self, region: Region) {
        match region {
            Region::EU => {
                match self.state {
                    State::Red => {
                        self.state=State::RedYellow;
                        self.change_state(region);
                        self.yellow_pin.on();
                        thread::park_timeout(self.yellow_wait);
                    }
                    State::RedYellow => {
                        self.state=State::Green;
                        self.change_state(region);
                        self.red_pin.off();
                        self.yellow_pin.off();
                        self.green_pin.on();
                        thread::park_timeout(self.max_wait);
                    }
                    State::Green => {
                        self.state = State::Yellow;
                        self.change_state(region);
                        self.green_pin.off();
                        self.yellow_pin.on();
                        thread::park_timeout(self.yellow_wait);
                    }
                    State::Yellow => {
                        self.state=State::Red;
                        self.change_state(region);
                        self.yellow_pin.off();
                        self.red_pin.on();
                        thread::park_timeout(self.max_wait);
                    }
                }
            }
            Region::US => {
                match self.state {
                    State::Red => {
                        self.state=State::RedYellow;
                        self.change_state(region);
                        self.red_pin.off();
                        self.yellow_pin.on();
                        thread::park_timeout(self.yellow_wait);
                    }
                    State::RedYellow => {
                        self.state=State::Green;
                        self.change_state(region);
                        self.yellow_pin.off();
                        self.green_pin.on();
                        thread::park_timeout(self.max_wait);
                    }
                    State::Green => {
                        self.state = State::Yellow;
                        self.change_state(region);
                        self.green_pin.off();
                        self.yellow_pin.on();
                        thread::park_timeout(self.yellow_wait);
                    }
                    State::Yellow => {
                        self.state=State::Red;
                        self.change_state(region);
                        self.yellow_pin.off();
                        self.red_pin.on();
                        thread::park_timeout(self.max_wait);
                    }
                }
            }
        }
    }

    fn change_state(&mut self, region: Region) {
        match region {
            Region::EU => {
                match self.state {
                    State::Red => {
                        self.red = true;
                        self.yellow = false;
                        self.green = false;
                    }
                    State::RedYellow => {
                        self.red = true;
                        self.yellow = true;
                        self.green = false;
                    }
                    State::Yellow => {
                        self.red = false;
                        self.yellow = true;
                        self.green = false;
                    }
                    State::Green => {
                        self.red = false;
                        self.yellow = false;
                        self.green = true;
                    }
                }
            }
            Region::US => {
                match self.state {
                    State::Red => {
                        self.red = true;
                        self.yellow = false;
                        self.green = false;
                    }
                    State::RedYellow => {
                        self.red = false;
                        self.yellow = true;
                        self.green = false;
                    }
                    State::Yellow => {
                        self.red = false;
                        self.yellow = true;
                        self.green = false;
                    }
                    State::Green => {
                        self.red = false;
                        self.yellow = false;
                        self.green = true;
                    }
                }
            }
        }
    }

    pub fn run (&mut self) {
        let mut walk_button = Button::new(19);
        let main_thread = thread::current();
        thread::spawn(move || loop {
            walk_button.wait_for_press(None);
            main_thread.unpark();
        });
        loop {
           self.state_machine(Region::EU);
        }
    }
}