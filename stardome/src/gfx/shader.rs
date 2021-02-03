#[repr(transparent)]
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
                let err = std::str::from_utf8(std::slice::from_raw_parts(
                    log.as_ptr() as *const _,
                    length as _,
                ))
                .unwrap()
                .to_owned();
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
                let err = std::str::from_utf8(std::slice::from_raw_parts(
                    log.as_ptr() as *const _,
                    length as _,
                ))
                .unwrap()
                .to_owned();
                Err(std::io::Error::new(std::io::ErrorKind::InvalidData, err))
            }
        }
    }

    pub fn use_gl(&self) {
        unsafe { gl::UseProgram(self.0) }
    }

    pub fn unuse_gl(&self) {
        unsafe { gl::UseProgram(0) }
    }

    // Error checking
    pub fn set_int(&self, name: &str, i: i32) -> Result<(), std::ffi::NulError> {
        let cstring = std::ffi::CString::new(name)?;
        unsafe { gl::Uniform1i(gl::GetUniformLocation(self.0, cstring.as_ptr()), i) }
        Ok(())
    }

    pub fn set_mat4(&self, name: &str, m: &na::Matrix4<f32>) -> Result<(), std::ffi::NulError> {
        let cstring = std::ffi::CString::new(name)?;
        unsafe {
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.0, cstring.as_ptr()),
                1,
                gl::FALSE,
                m.as_slice().as_ptr(),
            )
        }
        Ok(())
    }

    pub fn set_vec4(&self, name: &str, v: &na::Vector4<f32>) -> Result<(), std::ffi::NulError> {
        let cstring = std::ffi::CString::new(name)?;
        unsafe {
            gl::Uniform4fv(
                gl::GetUniformLocation(self.0, cstring.as_ptr()),
                1,
                v.as_slice().as_ptr(),
            )
        }
        Ok(())
    }

    pub fn set_vec3(&self, name: &str, v: &na::Vector3<f32>) -> Result<(), std::ffi::NulError> {
        let cstring = std::ffi::CString::new(name)?;
        unsafe {
            gl::Uniform3fv(
                gl::GetUniformLocation(self.0, cstring.as_ptr()),
                1,
                v.as_slice().as_ptr(),
            )
        }
        Ok(())
    }

    pub fn set_float(&self, name: &str, f: f32) -> Result<(), std::ffi::NulError> {
        let cstring = std::ffi::CString::new(name)?;
        unsafe { gl::Uniform1f(gl::GetUniformLocation(self.0, cstring.as_ptr()), f) }
        Ok(())
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.0) }
    }
}
