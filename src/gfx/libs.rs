// TODO find a way to handle multiple concurrent instances?
// This is not a near-future issue to solve, though
pub struct GraphicsLibs {
    pub sdl: sdl2::Sdl,
    pub pump: sdl2::EventPump,
    pub video: sdl2::VideoSubsystem,
    pub window: sdl2::video::Window,
    pub ctx: sdl2::video::GLContext,
    pub imgui: imgui::Context,
    pub imgui_sdl: imgui_sdl2::ImguiSdl2,
    pub imgui_gl: imgui_opengl_renderer::Renderer,
}

impl GraphicsLibs {
    pub fn load() -> Result<Self, crate::BoxError> {
        let sdl = sdl2::init()?;
        let pump = sdl.event_pump()?;
        let video = sdl.video()?;
        let attr = video.gl_attr();
        attr.set_context_version(3, 3);
        attr.set_context_profile(sdl2::video::GLProfile::Core);
        attr.set_accelerated_visual(true);
        attr.set_multisample_buffers(1);
        attr.set_multisample_samples(4);
        attr.set_context_flags().forward_compatible().debug().set();
        let window = video
            .window("Stardome", 960, 540)
            .resizable()
            .opengl()
            .position_centered()
            .hidden()
            .build()?;

        let ctx = window.gl_create_context()?;

        if video
            .gl_set_swap_interval(sdl2::video::SwapInterval::LateSwapTearing)
            .is_err()
        {
            video.gl_set_swap_interval(sdl2::video::SwapInterval::VSync)?;
        }

        gl::load_with(|s| video.gl_get_proc_address(s) as *const _);
        unsafe {
            let mut flags: i32 = 0;
            gl::GetIntegerv(gl::CONTEXT_FLAGS, &mut flags);
            // TODO check extension is loaded
            if ((flags & gl::CONTEXT_FLAG_DEBUG_BIT as i32) != 0)
                && gl::DebugMessageCallback::is_loaded()
            {
                gl::Enable(gl::DEBUG_OUTPUT);
                gl::DebugMessageCallback(Some(gl_debug_message_callback), std::ptr::null());
                gl::DebugMessageControl(
                    gl::DONT_CARE,
                    gl::DONT_CARE,
                    gl::DONT_CARE,
                    0,
                    std::ptr::null(),
                    gl::TRUE,
                );
            } else {
                // Problems with renderdoc
                //panic!("OpenGL debug could not be enabled");
            }

            gl::Viewport(0, 0, 960, 540);
            gl::Enable(gl::DEPTH_TEST);
            //gl::Enable(gl::CULL_FACE);
            gl::Enable(gl::TEXTURE_CUBE_MAP_SEAMLESS);
            gl::Enable(gl::MULTISAMPLE);
            gl::ClearColor(0.0, 0.0, 0.05, 1.0);
        }

        let mut imgui = imgui::Context::create();
        imgui.set_ini_filename(None);
        let imgui_sdl = imgui_sdl2::ImguiSdl2::new(&mut imgui, &window);
        let imgui_gl =
            imgui_opengl_renderer::Renderer::new(&mut imgui, |s| video.gl_get_proc_address(s) as _);

        Ok(Self {
            sdl,
            pump,
            video,
            window,
            ctx,
            imgui,
            imgui_sdl,
            imgui_gl,
        })
    }

    // Probably doesn't go here oh well
    pub fn handle_event_loop(&mut self) -> bool {
        use sdl2::event::{Event, WindowEvent};
        for e in self.pump.poll_iter() {
            self.imgui_sdl.handle_event(&mut self.imgui, &e);
            if self.imgui_sdl.ignore_event(&e) {
                continue;
            }
            GraphicsLibs::handle_event(&e);
            match e {
                Event::Quit { timestamp: _ } => return false,
                Event::Window {
                    timestamp: _,
                    window_id: _,
                    win_event,
                } => match win_event {
                    WindowEvent::SizeChanged(x, y) => (),
                    _ => (),
                },
                _ => (),
            }
        }
        self.imgui_sdl
            .prepare_frame(self.imgui.io_mut(), &self.window, &self.pump.mouse_state());
        self.imgui.io_mut().delta_time = 0.016;
        let ui = self.imgui.frame();
        ui.show_demo_window(&mut true);
        self.imgui_sdl.prepare_render(&ui, &self.window);
        self.imgui_gl.render(ui);
        true
    }

    // Should be run in event pump iter loop, but not exclusively
    // That way other code can use events
    pub fn handle_event(event: &sdl2::event::Event) {
        if let sdl2::event::Event::Window {
            timestamp: _,
            window_id: _,
            win_event,
        } = event
        {
            if let sdl2::event::WindowEvent::SizeChanged(x, y) = win_event {
                unsafe { gl::Viewport(0, 0, x.clone(), y.clone()) }
                // Get the camera updated here
                // Unless it will be handled by another function in the iter loop
            }
        }
    }

    // Same idea as above
    pub fn handle_frame(&mut self) {
        self.window.gl_swap_window();
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT) }
    }

    pub fn aspect_ratio(&mut self) -> f32 {
        let (x, y) = self.window.size();
        x as f32 / y as f32
    }
}

impl Drop for GraphicsLibs {
    fn drop(&mut self) {
        // TODO pack up properly, in correct order
    }
}

// As this will be a "library" in a sense, using log! is more smart
extern "system" fn gl_debug_message_callback(
    source: u32,
    t: u32,
    id: u32,
    severity: u32,
    _length: i32,
    message: *const i8,
    _user_param: *mut std::ffi::c_void,
) {
    let src = match source {
        gl::DEBUG_SOURCE_API => "API",
        gl::DEBUG_SOURCE_WINDOW_SYSTEM => "Window System",
        gl::DEBUG_SOURCE_SHADER_COMPILER => "Shader Compiler",
        gl::DEBUG_SOURCE_THIRD_PARTY => "Third-party",
        gl::DEBUG_SOURCE_APPLICATION => "Application",
        _ => "Other",
    };

    let ty = match t {
        gl::DEBUG_TYPE_ERROR => "Error",
        gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => "Deprecated behavior",
        gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => "Undefined behavior",
        gl::DEBUG_TYPE_PORTABILITY => "Portability",
        gl::DEBUG_TYPE_PERFORMANCE => "Performance",
        gl::DEBUG_TYPE_MARKER => "Marker",
        gl::DEBUG_TYPE_PUSH_GROUP => "Push group",
        gl::DEBUG_TYPE_POP_GROUP => "Pop group",
        _ => "Other",
    };

    let msg = unsafe {
        std::ffi::CStr::from_ptr(message)
            .to_str()
            .unwrap_or_default()
    };
    eprintln!("OpenGL {} {}: `{}`", src, ty, msg);
}
