mod stoplight;
use stoplight::*;
use std::time::Duration;

fn main() {
    let mut stoplight = Stoplight::init(Duration::new(10,0));
    stoplight.run();
}