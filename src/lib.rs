mod camera;
mod mesh;
mod shader;
mod sputils;
mod texture;
use shader::{Program, Shader};

type BoxError = Box<dyn std::error::Error + Send + Sync>;

pub struct StarDome {
    prog1: Program,
    prog2: Program,
    mesh1: mesh::Mesh,
    mesh2: mesh::Mesh,
    tex1: texture::Texture,
    tex2: texture::Texture,
    map1: texture::Cubemap,
    cam: camera::Camera,
    begin: std::time::Instant,
}

impl StarDome {
    pub fn new() -> Result<Self, BoxError> {
        let bsphere = mesh::Mesh::uv_sphere(1.0, 36, 18);
        let cube = mesh::Mesh::cube();

        let tex1 = texture::Texture::open("img/gen/earth_albedo.png")?;
        let tex2 = texture::Texture::open("img/gen/earth_bathymetry.png")?;
        let map1 = texture::Cubemap::open("img/gen/milky_way.png")?;
        // Keep hold of vertex shader as it will be reused a lot
        let prog1 = Program::new(&[
            &Shader::vertex(include_bytes!("0.vert.glsl"))?,
            &Shader::frag(include_bytes!("0.frag.glsl"))?,
        ])?;
        let prog2 = Program::new(&[
            &Shader::vertex(include_bytes!("box.vert.glsl"))?,
            &Shader::frag(include_bytes!("box.frag.glsl"))?,
        ])?;

        prog1.use_gl();
        prog1.set_int("texture1", 0)?;
        prog1.set_int("texture2", 1)?;
        prog1.unuse_gl();
        prog2.use_gl();
        prog2.set_int("skybox", 0)?;
        prog2.unuse_gl();

        Ok(Self {
            prog1,
            prog2,
            mesh1: bsphere,
            mesh2: cube,
            tex1,
            tex2,
            map1,
            cam: camera::Camera::new(
                glam::vec3(20000.0, 0.0, 0.0),
                90.0_f32.to_radians(),
                0.0_f32.to_radians(),
                90.0_f32.to_radians(),
            ),
            begin: std::time::Instant::now(),
        })
    }

    pub fn frame(&mut self) -> Result<std::time::Duration, BoxError> {
        let start = std::time::Instant::now();
        let elapsed_secs = self.begin.elapsed().as_secs_f32();
        let tw = self.begin.elapsed().as_secs_f64() * 0.1;
        //unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE); }
        let (djmjd0, tt, date, tut) = sputils::get_mjd(2020, 12, 10, 8, 0, 0.0, -0.2).unwrap();
        let tf = sputils::gcrs_to_itrs(
            djmjd0,
            tt + tw,
            date,
            tut + tw,
            0.093343 * sofa_sys::DAS2R,
            0.289699 * sofa_sys::DAS2R,
            0.115 * sofa_sys::DMAS2R,
            0.153 * sofa_sys::DMAS2R,
        );
        let model = tf * glam::Mat4::from_scale(glam::Vec3::splat(sgp4::WGS84.ae as f32));

        // Camera parameters
        self.cam.set_fov(60.0);
        let view = self.cam.view_matrix();
        let projection = self.cam.projection_matrix(16.0 / 9.0);

        self.prog1.use_gl();
        self.prog1.set_mat4("model", model)?;
        self.prog1.set_mat4("view", view)?;
        self.prog1.set_mat4("projection", projection)?;

        self.tex1.bind(0);
        self.tex2.bind(1);
        self.mesh1.draw();
        self.prog1.unuse_gl();

        // Cubemap
        unsafe {
            gl::DepthFunc(gl::LEQUAL);
        }
        self.prog2.use_gl();
        self.prog2.set_mat4("view", self.cam.rot_matrix())?;
        self.prog2.set_mat4("projection", projection)?;
        self.map1.bind(0);
        self.mesh2.draw();
        unsafe {
            gl::DepthFunc(gl::LESS);
        }

        Ok(start.elapsed())
    }
}
