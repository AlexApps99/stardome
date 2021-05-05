#![allow(unused_variables, dead_code)]
extern crate nalgebra as na;
mod gfx;

pub use gfx::drawable::Atmosphere;
pub use gfx::drawable::Planet;
pub use gfx::drawable::Points;
pub use gfx::drawable::Text;
pub use gfx::texture::Texture;

pub type BoxError = Box<dyn std::error::Error + Send + Sync>;
pub type BoxResult<T> = Result<T, BoxError>;

pub struct StarDome {
    pub graphics: gfx::Graphics,
    pub cam: gfx::camera::Camera,
    pub sun: na::Vector3<f64>,
    begin: std::time::Instant,
    frame_t: std::time::Instant,
    #[cfg(not(target_os = "emscripten"))]
    imgui: imgui::Context,
    #[cfg(not(target_os = "emscripten"))]
    imgui_sdl: imgui_sdl2::ImguiSdl2,
    #[cfg(not(target_os = "emscripten"))]
    imgui_gl: imgui_opengl_renderer::Renderer,
    #[cfg(not(target_os = "emscripten"))]
    text: Vec<(na::Vector3<f64>, u32, imgui::ImString)>,
}

impl StarDome {
    pub fn new() -> BoxResult<Self> {
        let t = std::time::Instant::now();
        let graphics = gfx::Graphics::new()?;
        #[cfg(not(target_os = "emscripten"))]
        let mut imgui = imgui::Context::create();
        #[cfg(not(target_os = "emscripten"))]
        imgui.set_ini_filename(None);
        #[cfg(not(target_os = "emscripten"))]
        let imgui_sdl = imgui_sdl2::ImguiSdl2::new(&mut imgui, &graphics.libs.window);
        #[cfg(not(target_os = "emscripten"))]
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
            sun: na::Vector3::zeros(), // When it's zeros lighting is disabled
            begin: t,
            frame_t: t,
            #[cfg(not(target_os = "emscripten"))]
            imgui,
            #[cfg(not(target_os = "emscripten"))]
            imgui_sdl,
            #[cfg(not(target_os = "emscripten"))]
            imgui_gl,
            #[cfg(not(target_os = "emscripten"))]
            text: Vec::new(),
        };
        s.graphics.libs.window.show();
        Ok(s)
    }

    pub fn get_sun_dir(&self) -> na::Vector3<f32> {
        // Does normalize work with zeroes
        na::convert(self.sun.normalize())
    }

    pub fn draw<T: gfx::drawable::Drawable>(&mut self, d: &mut T) {
        let s = self.get_sun_dir();
        #[cfg(not(target_os = "emscripten"))]
        d.draw(&mut self.graphics, &self.cam, s, &mut self.text)
            .unwrap();
        #[cfg(target_os = "emscripten")]
        d.draw(&mut self.graphics, &self.cam, s).unwrap();
    }

    pub fn frame<F>(&mut self, mut f: F) -> BoxResult<std::time::Duration>
    where
        F: FnMut(&mut imgui::Ui),
    {
        let last_frame = self.frame_t;
        self.frame_t = std::time::Instant::now();
        let elapsed = self.frame_t.duration_since(last_frame);

        while let Some(e) = self.graphics.libs.pump.poll_event() {
            #[cfg(not(target_os = "emscripten"))]
            self.imgui_sdl.handle_event(&mut self.imgui, &e);
            #[cfg(not(target_os = "emscripten"))]
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

        #[cfg(not(target_os = "emscripten"))]
        self.imgui_sdl.prepare_frame(
            self.imgui.io_mut(),
            &self.graphics.libs.window,
            &self.graphics.libs.pump.mouse_state(),
        );
        #[cfg(not(target_os = "emscripten"))]
        self.imgui.io_mut().update_delta_time(elapsed);
        #[cfg(not(target_os = "emscripten"))]
        let mut ui = self.imgui.frame();
        //ui.show_demo_window(&mut true);

        #[cfg(not(target_os = "emscripten"))]
        f(&mut ui);
        #[cfg(not(target_os = "emscripten"))]
        if !self.text.is_empty() {
            let vc = std::mem::take(&mut self.text);
            let tf =
                self.cam.projection_matrix(self.graphics.aspect_ratio()) * self.cam.view_matrix();
            let (sx, sy) = self.graphics.libs.window.size();
            let sx = sx as f32;
            let sy = sy as f32;
            use imgui::WindowFlags;
            imgui::Window::new(imgui::im_str!("Text"))
                .flags(
                    WindowFlags::NO_MOVE
                        | WindowFlags::NO_SCROLL_WITH_MOUSE
                        | WindowFlags::NO_BACKGROUND
                        | WindowFlags::NO_SAVED_SETTINGS
                        | WindowFlags::NO_FOCUS_ON_APPEARING
                        | WindowFlags::NO_BRING_TO_FRONT_ON_FOCUS
                        | WindowFlags::NO_DECORATION
                        | WindowFlags::NO_INPUTS,
                )
                .position([0.0, 0.0], imgui::Condition::Always)
                .size([sx, sy], imgui::Condition::Always)
                .build(&ui, || {
                    let no_pad = ui.push_style_var(imgui::StyleVar::WindowPadding([0.0, 0.0]));
                    for (pos, color, text) in vc.iter() {
                        let pce =
                            tf * na::Vector4::new(pos.x as f32, pos.y as f32, pos.z as f32, 1.0);
                        let pos: [f32; 2] = [
                            (pce.x / pce.w / 2.0 + 0.5) * sx,
                            (-pce.y / pce.w / 2.0 + 0.5) * sy,
                        ];
                        let comp: [u8; 4] = unsafe { std::mem::transmute(*color) };
                        let col = [
                            comp[3] as f32 / 255.0,
                            comp[2] as f32 / 255.0,
                            comp[1] as f32 / 255.0,
                            comp[0] as f32 / 255.0,
                        ];

                        // Clip offscreen based on text width
                        let [tx, ty] = ui.calc_text_size(text, false, 0.0);

                        if pce.z > 0.0
                            && pos[0] + tx > 0.0
                            && pos[1] + ty > 0.0
                            && pos[0] < sx
                            && pos[1] < sy
                        {
                            ui.set_cursor_screen_pos(pos);
                            ui.text_colored(col, text);
                        }
                    }
                    no_pad.pop(&ui);
                });
        }

        //self.graphics
        //    .frame(&self.cam, self.begin.elapsed().as_secs_f32())?;
        self.graphics.draw_skybox(&self.cam, &self.sun);
        #[cfg(not(target_os = "emscripten"))]
        self.imgui_sdl
            .prepare_render(&ui, &self.graphics.libs.window);
        #[cfg(not(target_os = "emscripten"))]
        self.imgui_gl.render(ui);
        self.graphics.handle_frame();
        Ok(self.frame_t.elapsed())
    }
}
