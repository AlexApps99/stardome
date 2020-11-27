mod mesh;
mod shader;
mod texture;
use shader::{Program, Shader};

type BoxError = Box<dyn std::error::Error + Send + Sync>;

pub struct StarDome {
    prog: Program,
    mesh1: mesh::Mesh,
    //mesh2: mesh::Mesh,
    tex1: texture::Texture,
    begin: std::time::Instant,
}

impl StarDome {
    pub fn new() -> Result<Self, BoxError> {
        let bsphere = mesh::Mesh::uv_sphere(1.0, 36, 18);

        // This section attempts to reproduce the effect of the workaround but it doesn't work somehow
        //let v = vec![1.0; 8*700];
        //let i = vec![1,2, 3,4,5, 6,7,8, 9,10,11, 12,13,14, 15,16,17, 18];
        //unsafe { mesh::Mesh::load_gl(v.as_slice(), i.as_slice()); }

        // This is the workaround. Uncomment it, and it will magically start working, who knows how
        //mesh::Mesh::uv_sphere(1.0, 360, 180);

        let tex1 = texture::Texture::open("warudo.png")?;
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
            //mesh2: gsphere,
            tex1,
            begin: std::time::Instant::now(),
        })
    }

    pub fn frame(&mut self) -> Result<std::time::Duration, BoxError> {
        let start = std::time::Instant::now();
        //unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE); }
        //let mut model = glam::Mat4::from_rotation_x(-90.0_f32.to_radians());
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
        //self.prog.set_mat4("view", glam::Mat4::from_translation(glam::vec3(-2.0, 0.0, -3.0)));
        //self.mesh2.draw();
        self.prog.unuse_gl();

        Ok(start.elapsed())
    }
}
