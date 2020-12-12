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
    pub fn new() -> Result<Self, crate::BoxError> {
        use shader::{Program, Shader};
        let libs = libs::GraphicsLibs::load()?;
        let meshes = vec![mesh::Mesh::uv_sphere(1.0, 36, 18), mesh::Mesh::cube()];
        let textures = vec![
            texture::Texture::open("img/gen/earth_albedo.png")?,
            texture::Texture::open("img/gen/earth_bathymetry.png")?,
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

    pub fn frame(
        &mut self,
        cam: &camera::Camera,
        elapsed_secs: f32,
    ) -> Result<(), crate::BoxError> {
        let tw = elapsed_secs as f64 * 0.1;
        //unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE); }
        let (djmjd0, tt, date, tut) =
            crate::sputils::get_mjd(2020, 12, 10, 8, 0, 0.0, -0.2).unwrap();
        let tf = crate::sputils::gcrs_to_itrs(
            djmjd0,
            tt + tw,
            date,
            tut + tw,
            0.093343 * sofa_sys::DAS2R,
            0.289699 * sofa_sys::DAS2R,
            0.115 * sofa_sys::DMAS2R,
            0.153 * sofa_sys::DMAS2R,
        );
        // INVERSE
        let model = tf * glam::Mat4::from_scale(glam::Vec3::splat(sgp4::WGS84.ae as f32));

        // Camera parameters
        let view = cam.view_matrix();
        let projection = cam.projection_matrix(self.libs.aspect_ratio());

        self.progs[0].use_gl();
        self.progs[0].set_mat4("model", model)?;
        self.progs[0].set_mat4("view", view)?;
        self.progs[0].set_mat4("projection", projection)?;

        self.textures[0].bind(0);
        self.textures[1].bind(1);
        self.meshes[0].draw();
        self.progs[0].unuse_gl();

        // Cubemap
        unsafe {
            gl::DepthFunc(gl::LEQUAL);
        }
        self.progs[1].use_gl();
        self.progs[1].set_mat4("view", cam.rot_matrix())?;
        self.progs[1].set_mat4("projection", projection)?;
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
