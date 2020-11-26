#[repr(transparent)]
// TODO uniforms
pub struct Shader(pub u32);

impl Shader {
    pub fn new(t: u32, s: &[u8]) -> Result<Self, std::io::Error> {
        unsafe {
            let id = gl::CreateShader(t);
            let length = s.len() as i32;
            let p1 = s.as_ptr() as *const i8;

            gl::ShaderSource(id, 1, &p1, &length);
            gl::CompileShader(id);
            let mut success: i32 = 0;
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
            if success != 0 {
                Ok(Self(id))
            } else {
                let mut log: [i8; 1024] = [0; 1024];
                let mut length: i32 = 0;
                gl::GetShaderInfoLog(id, 1024, &mut length, log.as_mut_ptr());
                gl::DeleteShader(id);
                let err = std::str::from_utf8(std::slice::from_raw_parts(log.as_ptr() as *const _, length as _)).unwrap().to_owned();
                Err(std::io::Error::new(std::io::ErrorKind::InvalidData, err))
            }
        }
    }

    pub fn vertex(s: &[u8]) -> Result<Self, std::io::Error> {
        Self::new(gl::VERTEX_SHADER, s)
    }

    pub fn frag(s: &[u8]) -> Result<Self, std::io::Error> {
        Self::new(gl::FRAGMENT_SHADER, s)
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.0) }
    }
}

#[repr(transparent)]
pub struct Program(pub u32);

impl Program {
    pub fn new(shaders: &[&Shader]) -> Result<Self, std::io::Error> {
        unsafe {
            let id = gl::CreateProgram();
            for shader in shaders {
                gl::AttachShader(id, shader.0);
            }
            gl::LinkProgram(id);
            let mut success: i32 = 0;
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
            if success != 0 {
                Ok(Self(id))
            } else {
                let mut log: [i8; 1024] = [0; 1024];
                let mut length: i32 = 0;
                gl::GetProgramInfoLog(id, 1024, &mut length, log.as_mut_ptr());
                gl::DeleteProgram(id);
                let err = std::str::from_utf8(std::slice::from_raw_parts(log.as_ptr() as *const _, length as _)).unwrap().to_owned();
                Err(std::io::Error::new(std::io::ErrorKind::InvalidData, err))
            }
        }
    }

    // Bad name, bad mutability
    pub fn r#use(&self) {
        unsafe { gl::UseProgram(self.0) }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.0) }
    }
}
