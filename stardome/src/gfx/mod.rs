pub mod camera;
pub mod libs;
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
        let textures = vec![
            texture::Texture::open("img/gen/earth_albedo.png")?,
            texture::Texture::open("img/gen/earth_bathymetry.png")?,
            texture::Texture::open("img/gen/moon_albedo.png")?,
        ];
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
        ];
        progs[0].use_gl();
        progs[0].set_int("texture1", 0)?;
        progs[0].set_int("texture2", 1)?;
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
            win_event,
        } = event
        {
            if let sdl2::event::WindowEvent::SizeChanged(x, y) = win_event {
                unsafe { gl::Viewport(0, 0, *x, *y) }
            }
        }
        false
    }

    pub fn handle_frame(&mut self) {
        self.libs.window.gl_swap_window();
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT) }
    }

    pub fn frame(
        &mut self,
        cam: &camera::Camera,
        elapsed_secs: f32,
    ) -> Result<(), crate::BoxError> {
        let tw = elapsed_secs as f64 * 0.1;
        //unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE); }
        let (djmjd0, tt, date, tut) = sputils::get_mjd(2020, 12, 10, 8, 0, 0.0, -0.2).unwrap();
        let tf = sputils::gcrs_to_itrs(
            djmjd0,
            tt + tw,
            date,
            tut + tw,
            0.093343 * 4.848136811095359935899141e-6,
            0.289699 * 4.848136811095359935899141e-6,
            0.115 * (4.848136811095359935899141e-6 / 1e3),
            0.153 * (4.848136811095359935899141e-6 / 1e3),
        );

        // TODO THIS IS ASS MOON CODE >:/
        let mut jpl = sputils::eph::JPL::new().unwrap();
        let (mut pos, lib) = jpl.moon(sputils::time::TT(djmjd0, tt + tw).into_tdb(0.0));
        pos.0 *= 149597.8707; // From AU to megameters

        let mut model: na::Matrix4<f32> =
            na::convert::<na::Matrix3<f64>, na::Matrix3<f32>>(tf.transpose()).fixed_resize(0.0);
        model.m44 = 1.0;
        // Oblate spheroid
        model *= na::Matrix4::new_scaling(6.37814);

        // Camera parameters
        let view = cam.view_matrix();
        let projection = cam.projection_matrix(self.aspect_ratio());

        self.progs[0].use_gl();
        self.progs[0].set_mat4("model", &model)?;
        self.progs[0].set_mat4("view", &view)?;
        self.progs[0].set_mat4("projection", &projection)?;

        self.textures[0].bind(0);
        self.textures[1].bind(1);
        self.meshes[0].draw();
        // Drawing the moon
        model = na::convert::<na::Matrix4<f64>, na::Matrix4<f32>>(na::Matrix4::new_translation(&pos.0) * na::Matrix4::new_scaling(1.7371));
        self.progs[0].set_mat4("model", &model)?;
        self.textures[2].bind(0);
        unsafe {
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        self.meshes[0].draw();
        // TODO Bogus sun should be drawn as flat texture
        // drawn onto skybox (don't draw it at far coordinates due to FP issues)
        //model = na::Matrix4::new_translation(&na::Vector3::new(147120.0, 0.0, 0.0)) * na::Matrix4::new_scaling(696.34);
        //self.progs[0].set_mat4("model", &model)?;
        //self.meshes[0].draw();

        self.progs[0].unuse_gl();

        // Cubemap
        unsafe {
            gl::DepthFunc(gl::LEQUAL);
        }
        self.progs[1].use_gl();
        self.progs[1].set_mat4("view", &cam.rot_matrix())?;
        self.progs[1].set_mat4("projection", &projection)?;
        self.cubemap.bind(0);
        self.meshes[1].draw();
        unsafe {
            gl::DepthFunc(gl::LESS);
        }

        Ok(())
    }
}

impl Drop for Graphics {
    fn drop(&mut self) {
        // TODO
    }
}
