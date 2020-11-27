mod mesh;
mod shader;
mod texture;
use shader::{Program, Shader};

type BoxError = Box<dyn std::error::Error + Send + Sync>;

pub struct StarDome {
    prog: Program,
    mesh: mesh::Mesh,
    tex1: texture::Texture,
    begin: std::time::Instant,
}

impl StarDome {
    pub fn new() -> Result<Self, BoxError> {
        let sphere = mesh::Mesh::uv_sphere(1.0, 360, 180);
        let tex1 = texture::Texture::open("warudo.png")?;
        // Keep hold of vertex shader as it will be reused a lot
        let prog = Program::new(&[
            &Shader::vertex(include_bytes!("0.vert.glsl"))?,
            &Shader::frag(include_bytes!("0.frag.glsl"))?,
        ])?;

        prog.use_gl();
        prog.set_int("texture1", 0)?;

        Ok(Self {
            prog,
            mesh: sphere,
            tex1,
            begin: std::time::Instant::now(),
        })
    }

    pub fn frame(&mut self) -> Result<std::time::Duration, BoxError> {
        let start = std::time::Instant::now();
        //unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE); }
        let mut model = glam::Mat4::from_rotation_x(-90.0_f32.to_radians());
        model = model * glam::Mat4::from_rotation_z(-self.begin.elapsed().as_secs_f32());
        let view = glam::Mat4::from_translation(glam::vec3(0.0, 0.0, -3.0));
        let projection =
            glam::Mat4::perspective_rh_gl(45.0_f32.to_radians(), 16.0 / 9.0, 0.1, 100.0);

        self.prog.set_mat4("model", model)?;
        self.prog.set_mat4("view", view)?;
        self.prog.set_mat4("projection", projection)?;
        self.prog.use_gl();

        self.tex1.bind(0);
        self.mesh.draw();

        Ok(start.elapsed())
    }
}
