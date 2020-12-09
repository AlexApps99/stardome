mod camera;
mod mesh;
mod shader;
mod texture;
use shader::{Program, Shader};

type BoxError = Box<dyn std::error::Error + Send + Sync>;

pub struct StarDome {
    prog1: Program,
    prog2: Program,
    mesh1: mesh::Mesh,
    mesh2: mesh::Mesh,
    tex1: texture::Texture,
    map1: texture::Cubemap,
    cam: camera::Camera,
    begin: std::time::Instant,
}

impl StarDome {
    pub fn new() -> Result<Self, BoxError> {
        let bsphere = mesh::Mesh::uv_sphere(1.0, 36, 18);
        let cube = mesh::Mesh::cube();

        let tex1 = texture::Texture::open("img/gen/earth_albedo.png")?;
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
            map1,
            cam: camera::Camera::new(glam::vec3(0.0, 0.0, -3.0), 0.0, 0.0, 0.0),
            begin: std::time::Instant::now(),
        })
    }

    pub fn frame(&mut self) -> Result<std::time::Duration, BoxError> {
        let start = std::time::Instant::now();
        let elapsed_secs = self.begin.elapsed().as_secs_f32();
        //unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE); }
        let model = glam::Mat4::from_translation(glam::Vec3::zero())
            * glam::Mat4::from_rotation_y(-elapsed_secs);

        // Camera parameters
        let view = self.cam.view_matrix();
        let projection = self.cam.projection_matrix(16.0 / 9.0);

        self.prog1.use_gl();
        self.prog1.set_mat4("model", model)?;
        self.prog1.set_mat4("view", view)?;
        self.prog1.set_mat4("projection", projection)?;

        self.tex1.bind(0);
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
