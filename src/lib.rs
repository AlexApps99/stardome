// TODO clean up and make more like example before continuing
// encapsulate behavior

type BoxError = Box<dyn std::error::Error + Send + Sync>;

pub struct StarDome {
    shader_program: u32,
    ebo: u32,
    vao: u32,
}

impl StarDome {
    pub fn new() -> Self {
        Self {
            shader_program: 0,
            ebo: 0,
            vao: 0,
        }
    }

    pub fn setup(&mut self) -> Result<(), BoxError> {
        unsafe {
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            // Crash here
            let vtx = std::ffi::CString::new(include_str!("0.vert.glsl"))?;
            let frag = std::ffi::CString::new(include_str!("0.frag.glsl"))?;
            gl::ShaderSource(
                vertex_shader,
                1,
                &vtx.as_c_str().as_ptr() as *const _,
                std::ptr::null(),
            );
            gl::CompileShader(vertex_shader);

            let frag_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(
                frag_shader,
                1,
                &frag.as_c_str().as_ptr() as *const _,
                std::ptr::null(),
            );
            gl::CompileShader(frag_shader);

            let shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, frag_shader);
            gl::LinkProgram(shader_program);
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(frag_shader);

            let vertices: [f32; 12] = [
                0.5, 0.5, 0.0, 0.5, -0.5, 0.0, -0.5, -0.5, 0.0, -0.5, 0.5, 0.0,
            ];
            let indices: [u32; 6] = [0, 1, 3, 1, 2, 3];
            let mut vbo: u32 = 0;
            let mut vao: u32 = 0;
            let mut ebo: u32 = 0;
            gl::GenBuffers(1, &mut ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                4 * 6,
                indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                48,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 12, std::ptr::null());
            gl::EnableVertexAttribArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

            gl::BindVertexArray(0);
            self.shader_program = shader_program;
            self.ebo = ebo;
            self.vao = vao;
        }
        Ok(())
    }

    pub fn frame(&mut self) -> Result<std::time::Duration, BoxError> {
        let start = std::time::Instant::now();
        unsafe {
            gl::UseProgram(self.shader_program);
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }
        return Ok(std::time::Instant::now() - start);
    }
}
