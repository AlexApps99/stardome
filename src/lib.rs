mod mesh;
mod shader;
mod texture;
use shader::{Program, Shader};

type BoxError = Box<dyn std::error::Error + Send + Sync>;

pub struct StarDome {
    prog: Program,
    mesh1: mesh::Mesh,
    tex1: texture::Texture,
    begin: std::time::Instant,
}

impl StarDome {
    pub fn new() -> Result<Self, BoxError> {
        let bsphere = mesh::Mesh::uv_sphere(1.0, 36, 18);

        let tex1 = texture::Texture::open("img/gen/earth_albedo.png")?;
        // Keep hold of vertex shader as it will be reused a lot
        let prog = Program::new(&[
            &Shader::vertex(include_bytes!("0.vert.glsl"))?,
            &Shader::frag(include_bytes!("0.frag.glsl"))?,
        ])?;

        prog.use_gl();
        prog.set_int("texture1", 0)?;
        prog.unuse_gl();

        Ok(Self {
            prog,
            mesh1: bsphere,
            tex1,
            begin: std::time::Instant::now(),
        })
    }

    pub fn frame(&mut self) -> Result<std::time::Duration, BoxError> {
        let start = std::time::Instant::now();
        //unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE); }
        let model = glam::Mat4::from_rotation_y(-self.begin.elapsed().as_secs_f32());
        let view = glam::Mat4::from_translation(glam::vec3(0.0, 0.0, -3.0));
        let projection =
            glam::Mat4::perspective_rh_gl(45.0_f32.to_radians(), 16.0 / 9.0, 0.1, 100.0);

        self.prog.use_gl();
        self.prog.set_mat4("model", model)?;
        self.prog.set_mat4("view", view)?;
        self.prog.set_mat4("projection", projection)?;

        self.tex1.bind(0);
        self.mesh1.draw();
        self.prog.unuse_gl();

        Ok(start.elapsed())
    }
}
