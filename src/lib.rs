#![allow(unused_variables, dead_code)]
extern crate nalgebra as na;
pub mod gfx; // Don't keep pub
pub mod sputils;

type BoxError = Box<dyn std::error::Error + Send + Sync>;

pub struct StarDome {
    pub graphics: gfx::Graphics,
    cam: gfx::camera::Camera,
    begin: std::time::Instant,
}

impl StarDome {
    pub fn new() -> Result<Self, BoxError> {
        let mut s = Self {
            graphics: gfx::Graphics::new()?,
            cam: gfx::camera::Camera::new(
                na::Vector3::new(20000.0, 0.0, 0.0),
                90.0_f32.to_radians(),
                0.0_f32.to_radians(),
                90.0_f32.to_radians(),
            ),
            begin: std::time::Instant::now(),
        };
        s.graphics.libs.window.show();
        Ok(s)
    }

    pub fn frame(&mut self) -> Result<std::time::Duration, BoxError> {
        let start = std::time::Instant::now();
        let elapsed_secs = self.begin.elapsed().as_secs_f32();
        self.graphics.frame(&self.cam, elapsed_secs)?;
        if !self.graphics.libs.handle_event_loop() {
            return Err("".into());
        }
        self.graphics.libs.handle_frame();
        Ok(start.elapsed())
    }
}
