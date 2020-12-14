#![allow(unused_variables, dead_code)]

use stardome::StarDome;

// Event loop should not be here, probably
// It should just be a loop with some function that hides away this
fn main() {
    let mut sd = StarDome::new().unwrap();

    loop {
        if sd.frame().is_err() {
            break;
        }
    }
}
