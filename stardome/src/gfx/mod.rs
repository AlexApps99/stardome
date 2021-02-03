pub mod camera;
pub mod drawable;
pub mod libs;
pub mod material;
pub mod mesh;
pub mod shader;
pub mod texture;

pub struct Graphics {
    pub libs: libs::GraphicsLibs,
    pub progs: Vec<shader::Program>,
    pub meshes: Vec<mesh::Mesh>,
    pub textures: Vec<texture::Texture>,
    pub cubemap: texture::Cubemap,
    // TODO this buffer should NOT include imgui or other overlay type things
    // This does cause problems with depth, probably will need to be a texture not a render thingy
    fb: u32,
    tex_fb: u32,
    rbo: u32,
}

impl Graphics {
    pub fn new() -> crate::BoxResult<Self> {
        use shader::{Program, Shader};
        let libs = libs::GraphicsLibs::load()?;
        let meshes = vec![
            mesh::Mesh::uv_sphere(1.0, 360, 180),
            mesh::Mesh::cube(),
            mesh::Mesh::quad(),
        ];
        let textures = vec![/*
            texture::Texture::open("img/gen/earth_albedo.png")?,
            texture::Texture::open("img/gen/earth_bathymetry.png")?,
            texture::Texture::open("img/gen/moon_albedo.png")?,
        */];
        let cubemap = texture::Cubemap::open("img/gen/milky_way.png")?;
        let progs = vec![
            Program::new(&[
                &Shader::vertex(include_bytes!("../glsl/planet.vert.glsl"))?,
                &Shader::frag(include_bytes!("../glsl/planet.frag.glsl"))?,
            ])?,
            Program::new(&[
                &Shader::vertex(include_bytes!("../glsl/box.vert.glsl"))?,
                &Shader::frag(include_bytes!("../glsl/box.frag.glsl"))?,
            ])?,
            Program::new(&[
                &Shader::vertex(include_bytes!("../glsl/simple.vert.glsl"))?,
                &Shader::frag(include_bytes!("../glsl/simple.frag.glsl"))?,
            ])?,
            Program::new(&[
                &Shader::vertex(include_bytes!("../glsl/atmosphere.vert.glsl"))?,
                &Shader::frag(include_bytes!("../glsl/atmosphere.frag.glsl"))?,
            ])?,
            Program::new(&[
                &Shader::vertex(include_bytes!("../glsl/screen.vert.glsl"))?,
                &Shader::frag(include_bytes!("../glsl/screen.frag.glsl"))?,
            ])?,
        ];
        progs[0].use_gl();
        progs[0].set_int("texture1", 0)?;
        progs[0].unuse_gl();
        progs[1].use_gl();
        progs[1].set_int("skybox", 0)?;
        progs[1].unuse_gl();

        let mut fb: u32 = 0;
        let mut tex_fb: u32 = 0;
        let mut rbo: u32 = 0;
        unsafe {
            gl::GenFramebuffers(1, &mut fb);
            gl::BindFramebuffer(gl::FRAMEBUFFER, fb);
            gl::GenTextures(1, &mut tex_fb);
            gl::BindTexture(gl::TEXTURE_2D, tex_fb);
            // TODO make the texture not look gross (min mag wrap clamp etc)
            let (sx, sy) = libs.window.size();
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB16F as i32,
                sx as i32,
                sy as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                std::ptr::null(),
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::BindTexture(gl::TEXTURE_2D, 0);

            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                tex_fb,
                0,
            );

            gl::GenRenderbuffers(1, &mut rbo);
            gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
            gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH24_STENCIL8, sx as i32, sy as i32);
            gl::BindRenderbuffer(gl::RENDERBUFFER, 0);

            gl::FramebufferRenderbuffer(
                gl::FRAMEBUFFER,
                gl::DEPTH_STENCIL_ATTACHMENT,
                gl::RENDERBUFFER,
                rbo,
            );

            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                panic!("TODO considnenthdywons");
            }
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }

        Ok(Self {
            libs,
            progs,
            meshes,
            textures,
            cubemap,
            fb,
            tex_fb,
            rbo,
        })
    }

    pub fn aspect_ratio(&mut self) -> f32 {
        let (x, y) = self.libs.window.size();
        x as f32 / y as f32
    }

    pub fn handle_event(&mut self, event: &sdl2::event::Event) -> bool {
        if let sdl2::event::Event::Window {
            timestamp: _,
            window_id: _,
            win_event,
        } = event
        {
            if let sdl2::event::WindowEvent::SizeChanged(x, y) = win_event {
                unsafe {
                    gl::Viewport(0, 0, *x, *y);
                    let (sx, sy) = self.libs.window.size();
                    gl::BindTexture(gl::TEXTURE_2D, self.tex_fb);
                    gl::TexImage2D(
                        gl::TEXTURE_2D,
                        0,
                        gl::RGB16F as i32,
                        sx as i32,
                        sy as i32,
                        0,
                        gl::RGB,
                        gl::UNSIGNED_BYTE,
                        std::ptr::null(),
                    );
                    gl::BindTexture(gl::TEXTURE_2D, 0);
                    gl::BindRenderbuffer(gl::RENDERBUFFER, self.rbo);
                    gl::RenderbufferStorage(
                        gl::RENDERBUFFER,
                        gl::DEPTH24_STENCIL8,
                        sx as i32,
                        sy as i32,
                    );
                    gl::BindRenderbuffer(gl::RENDERBUFFER, 0);
                }
            }
        }
        false
    }

    pub fn handle_frame(&mut self, cam: &camera::Camera) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0); // back to default

            self.progs[4].use_gl();
            self.progs[4].set_float("exposure", cam.exposure);
            gl::Disable(gl::DEPTH_TEST);
            gl::BindTexture(gl::TEXTURE_2D, self.tex_fb);
            self.meshes[2].draw();
            gl::Enable(gl::DEPTH_TEST);
        }
    }

    pub fn flip_frame(&mut self) {
        unsafe {
            self.libs.window.gl_swap_window();

            gl::BindFramebuffer(gl::FRAMEBUFFER, self.fb);
            gl::Clear(gl::DEPTH_BUFFER_BIT | gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn draw_skybox(&mut self, cam: &camera::Camera, sun: &na::Vector3<f64>) {
        // Cubemap
        unsafe {
            gl::DepthFunc(gl::LEQUAL);
        }
        self.progs[1].use_gl();
        let _ = self.progs[1].set_mat4("view", &cam.rot_matrix());
        let projection = cam.projection_matrix(self.aspect_ratio());
        let _ = self.progs[1].set_mat4("projection", &projection);
        let _ = self.progs[1].set_vec3("sun_dir", &na::convert(sun.normalize()));
        let _ = self.progs[1].set_float("sun_angle_rad", (695.700 / sun.magnitude()).atan() as f32);
        self.cubemap.bind(0);
        self.meshes[1].draw();
        self.progs[1].unuse_gl();
        unsafe {
            gl::DepthFunc(gl::LESS);
        }
    }
}

impl Drop for Graphics {
    fn drop(&mut self) {
        // TODO
    }
}
