// TODO find a way to handle multiple concurrent instances?
// This is not a near-future issue to solve, though
pub struct GraphicsLibs {
    pub sdl: sdl2::Sdl,
    pub pump: sdl2::EventPump,
    pub video: sdl2::VideoSubsystem,
    // window is the only object that is PER window
    // everything else, only one is needed for the program
    pub window: sdl2::video::Window,
    pub ctx: sdl2::video::GLContext,
}

impl GraphicsLibs {
    pub fn load() -> crate::BoxResult<Self> {
        let sdl = sdl2::init()?;
        let pump = sdl.event_pump()?;
        let video = sdl.video()?;
        let attr = video.gl_attr();
        #[cfg(any(target_os = "emscripten", feature = "gles"))]
        attr.set_context_version(3, 0);
        #[cfg(all(not(target_os = "emscripten"), not(feature = "gles")))]
        attr.set_context_version(3, 3);
        #[cfg(any(target_os = "emscripten", feature = "gles"))]
        attr.set_context_profile(sdl2::video::GLProfile::GLES);
        #[cfg(all(not(target_os = "emscripten"), not(feature = "gles")))]
        attr.set_context_profile(sdl2::video::GLProfile::Core);
        attr.set_accelerated_visual(true);
        attr.set_multisample_buffers(1);
        attr.set_multisample_samples(4);
        #[cfg(all(not(target_os = "emscripten"), feature = "gles"))]
        attr.set_context_flags().debug().set();
        #[cfg(all(not(target_os = "emscripten"), not(feature = "gles")))]
        attr.set_context_flags().debug().forward_compatible().set();
        let window = video
            .window("Stardome", 960, 540)
            .resizable()
            .opengl()
            .position_centered()
            .hidden()
            .build()?;

        let ctx = window.gl_create_context()?;

        #[cfg(not(target_os = "emscripten"))]
        if video
            .gl_set_swap_interval(sdl2::video::SwapInterval::LateSwapTearing)
            .is_err()
        {
            video.gl_set_swap_interval(sdl2::video::SwapInterval::VSync)?;
        }

        gl::load_with(|s| video.gl_get_proc_address(s) as *const _);
        unsafe {
            #[cfg(not(target_os = "emscripten"))]
            let mut flags: i32 = 0;
            #[cfg(not(target_os = "emscripten"))]
            gl::GetIntegerv(gl::CONTEXT_FLAGS, &mut flags);
            // TODO check extension is loaded
            #[cfg(not(target_os = "emscripten"))]
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
            gl::Enable(gl::CULL_FACE);
            #[cfg(not(any(target_os = "emscripten", feature = "gles")))]
            gl::Enable(gl::TEXTURE_CUBE_MAP_SEAMLESS);
            #[cfg(not(any(target_os = "emscripten", feature = "gles")))]
            gl::Enable(gl::MULTISAMPLE);
            #[cfg(not(any(target_os = "emscripten", feature = "gles")))]
            gl::Enable(gl::LINE_SMOOTH);
            #[cfg(not(any(target_os = "emscripten", feature = "gles")))]
            gl::Disable(gl::PROGRAM_POINT_SIZE);
            #[cfg(not(any(target_os = "emscripten", feature = "gles")))]
            gl::Hint(gl::LINE_SMOOTH_HINT, gl::NICEST);
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
#[cfg(not(target_os = "emscripten"))]
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
