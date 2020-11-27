// encapsulate behavior
mod mesh;
mod shader;
mod texture;
use shader::{Program, Shader};

type BoxError = Box<dyn std::error::Error + Send + Sync>;

pub struct StarDome {
    shader_program: Option<Program>,
    ebo: u32,
    vao: u32,
    tex1: u32,
    tex2: u32,
    begin: std::time::Instant,
}

impl StarDome {
    pub fn new() -> Self {
        Self {
            shader_program: None,
            ebo: 0,
            vao: 0,
            tex1: 0,
            tex2: 0,
            begin: std::time::Instant::now(),
        }
    }

    pub fn setup(&mut self) -> Result<(), BoxError> {
        unsafe {
            let prog = Program::new(&[
                &Shader::vertex(include_bytes!("0.vert.glsl"))?,
                &Shader::frag(include_bytes!("0.frag.glsl"))?,
            ])?;

            let vertices: [f32; 32] = [
                // positions          // colors           // texture coords
                0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, // top right
                0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, // bottom right
                -0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, // bottom left
                -0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, // top left
            ];
            let indices: [u32; 6] = [0, 1, 3, 1, 2, 3];
            let mut vbo: u32 = 0;
            let mut vao: u32 = 0;
            let mut ebo: u32 = 0;
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                std::mem::size_of_val(&vertices) as isize,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                std::mem::size_of_val(&indices) as isize,
                indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 8 * 4, (0 * 4) as *const _);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 8 * 4, (3 * 4) as *const _);
            gl::EnableVertexAttribArray(1);

            gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, 8 * 4, (6 * 4) as *const _);
            gl::EnableVertexAttribArray(2);

            //gl::BindBuffer(gl::ARRAY_BUFFER, 0);

            //gl::BindVertexArray(0);

            let tex1 = texture::Texture::load(&mut std::fs::File::open("container.png")?)?.to_gl();
            let tex2 =
                texture::Texture::load(&mut std::fs::File::open("awesomeface.png")?)?.to_gl();
            prog.r#use();
            prog.set_int("texture1", 0);
            prog.set_int("texture2", 1);

            self.shader_program = Some(prog);
            self.ebo = ebo;
            self.vao = vao;
            self.tex1 = tex1;
            self.tex2 = tex2;
        }
        Ok(())
    }

    pub fn frame(&mut self) -> Result<std::time::Duration, BoxError> {
        let start = std::time::Instant::now();
        let model = glam::Mat4::from_rotation_x(-55.0_f32.to_radians());
        let view = glam::Mat4::from_translation(glam::vec3(0.0, 0.0, -3.0));
        let projection = glam::Mat4::perspective_rh_gl(
            45.0_f32.to_radians(),
            16.0/9.0,
            0.1,
            100.0
        );

        if let Some(prog) = self.shader_program.as_ref() {
            prog.set_mat4("model", model);
            prog.set_mat4("view", view);
            prog.set_mat4("projection", projection);

            prog.r#use();
        }
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.tex1);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, self.tex2);
            //gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BindVertexArray(self.vao);
            // Hard coding is bad >:(
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }
        return Ok(start.elapsed());
    }
}
