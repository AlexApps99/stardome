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
}

impl Graphics {
    pub fn new() -> crate::BoxResult<Self> {
        use shader::{Program, Shader};
        let libs = libs::GraphicsLibs::load()?;
        let meshes = vec![mesh::Mesh::uv_sphere(1.0, 360, 180), mesh::Mesh::cube()];
        let textures = vec![/*
            texture::Texture::open("img/gen/earth_albedo.png")?,
            texture::Texture::open("img/gen/earth_bathymetry.png")?,
            texture::Texture::open("img/gen/moon_albedo.png")?,
        */];
        let cubemap = texture::Cubemap::open("img/gen/milky_way.png")?;
        #[cfg(not(any(target_os = "emscripten", feature = "gles")))]
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
        ];
        #[cfg(any(target_os = "emscripten", feature = "gles"))]
        let progs = vec![
            Program::new(&[
                &Shader::vertex(include_bytes!("../glsl/planet.es.vert.glsl"))?,
                &Shader::frag(include_bytes!("../glsl/planet.es.frag.glsl"))?,
            ])?,
            Program::new(&[
                &Shader::vertex(include_bytes!("../glsl/box.es.vert.glsl"))?,
                &Shader::frag(include_bytes!("../glsl/box.es.frag.glsl"))?,
            ])?,
            Program::new(&[
                &Shader::vertex(include_bytes!("../glsl/simple.es.vert.glsl"))?,
                &Shader::frag(include_bytes!("../glsl/simple.es.frag.glsl"))?,
            ])?,
            Program::new(&[
                &Shader::vertex(include_bytes!("../glsl/atmosphere.es.vert.glsl"))?,
                &Shader::frag(include_bytes!("../glsl/atmosphere.es.frag.glsl"))?,
            ])?,
        ];
        progs[0].use_gl();
        progs[0].set_int("texture1", 0)?;
        progs[0].unuse_gl();
        progs[1].use_gl();
        progs[1].set_int("skybox", 0)?;
        progs[1].unuse_gl();

        Ok(Self {
            libs,
            progs,
            meshes,
            textures,
            cubemap,
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
            win_event: sdl2::event::WindowEvent::SizeChanged(x, y),
        } = event
        {
            unsafe { gl::Viewport(0, 0, *x, *y) }
        }
        false
    }

    pub fn handle_frame(&mut self) {
        self.libs.window.gl_swap_window();
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT) }
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
