#![allow(unused_variables, dead_code)]
extern crate nalgebra as na;
mod gfx;

pub type BoxError = Box<dyn std::error::Error + Send + Sync>;
pub type BoxResult<T> = Result<T, BoxError>;

macro_rules! cstr {
    ($s:expr) => {
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(concat!($s, "\0").as_bytes()) }
    };
}

pub struct StarDome {
    pub graphics: gfx::Graphics,
    pub cam: gfx::camera::Camera,
    begin: std::time::Instant,
    frame_t: std::time::Instant,
    imgui: imgui::Context,
    imgui_sdl: imgui_sdl2::ImguiSdl2,
    imgui_gl: imgui_opengl_renderer::Renderer,
}

impl StarDome {
    pub fn new() -> BoxResult<Self> {
        let t = std::time::Instant::now();
        let graphics = gfx::Graphics::new()?;
        let mut imgui = imgui::Context::create();
        imgui.set_ini_filename(None);
        let imgui_sdl = imgui_sdl2::ImguiSdl2::new(&mut imgui, &graphics.libs.window);
        let imgui_gl = imgui_opengl_renderer::Renderer::new(&mut imgui, |s| {
            graphics.libs.video.gl_get_proc_address(s) as _
        });

        let mut s = Self {
            graphics,
            cam: gfx::camera::Camera::new(
                na::Vector3::new(10.0, 0.0, 0.0),
                90.0_f32.to_radians(),
                0.0_f32.to_radians(),
                90.0_f32.to_radians(),
            ),
            begin: t,
            frame_t: t,
            imgui,
            imgui_sdl,
            imgui_gl,
        };
        s.graphics.libs.window.show();
        Ok(s)
    }

    pub fn frame<F>(&mut self, mut f: F) -> BoxResult<std::time::Duration>
    where
        F: FnMut(&mut imgui::Ui),
    {
        let last_frame = self.frame_t;
        self.frame_t = std::time::Instant::now();
        let elapsed = self.frame_t.duration_since(last_frame);

        while let Some(e) = self.graphics.libs.pump.poll_event() {
            self.imgui_sdl.handle_event(&mut self.imgui, &e);
            if self.imgui_sdl.ignore_event(&e) {
                continue;
            }
            if self.graphics.handle_event(&e) {
                continue;
            }
            if let sdl2::event::Event::Quit { .. } = e {
                return Err("Quitting".into());
            }
        }

        self.imgui_sdl.prepare_frame(
            self.imgui.io_mut(),
            &self.graphics.libs.window,
            &self.graphics.libs.pump.mouse_state(),
        );
        self.imgui.io_mut().update_delta_time(elapsed);
        let mut ui = self.imgui.frame();
        //ui.show_demo_window(&mut true);

        f(&mut ui);

        self.graphics
            .frame(&self.cam, self.begin.elapsed().as_secs_f32())?;
        self.imgui_sdl
            .prepare_render(&ui, &self.graphics.libs.window);
        self.imgui_gl.render(ui);
        self.graphics.handle_frame();
        Ok(self.frame_t.elapsed())
    }
}
