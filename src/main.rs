#![allow(unused_variables, dead_code)]

use stardome::gfx::GraphicsLibs;
use stardome::StarDome;

// Event loop should not be here, probably
// It should just be a loop with some function that hides away this
fn main() {
    let mut g = GraphicsLibs::load().unwrap();
    let mut sd = StarDome::new().unwrap();
    g.window.show();
    loop {
        sd.frame().unwrap();
        if !g.handle_event_loop() {
            break;
        }
        g.handle_frame();
    }
}
