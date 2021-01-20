// TODO find a way to handle multiple concurrent instances?
// This is not a near-future issue to solve, though
pub struct GraphicsLibs {
    pub sdl: sdl2::Sdl,
    pub pump: sdl2::EventPump,
    pub video: sdl2::VideoSubsystem,
    pub window: sdl2::video::Window,
    pub ctx: sdl2::video::GLContext,
}

impl GraphicsLibs {
    pub fn load() -> crate::BoxResult<Self> {
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
            gl::Enable(gl::LINE_SMOOTH);
            gl::Hint(gl::LINE_SMOOTH_HINT, gl::NICEST);
            gl::ClearColor(0.0, 0.0, 0.05, 1.0);
        }

        Ok(Self {
            sdl,
            pump,
            video,
            window,
            ctx,
        })
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
