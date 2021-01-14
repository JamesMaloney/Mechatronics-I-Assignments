mod larson_scanner;
use larson_scanner::*;

fn main() {
    let mut larsonscanner = LarsonScanner::init();
    larsonscanner.run();
}