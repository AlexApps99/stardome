pub struct Mesh {
    vbo: u32,
    vao: u32,
    ebo: u32,
    vertices: i32,
    indices: i32,
}

use std::f32::consts::{PI, TAU};

// Cube-sphere would be nice
// Less distortion around poles
// And easy use of cubemap
impl Mesh {
    // http://www.songho.ca/opengl/gl_sphere.html
    #[allow(clippy::many_single_char_names)]
    pub fn uv_sphere(radius: f32, h_div: u32, v_div: u32) -> Self {
        let v_div = v_div + 1;
        if radius <= 0.0 || h_div < 3 || v_div < 2 {
            panic!("Invalid parameters");
        }
        let mut vertices: Vec<f32> = Vec::with_capacity(8 * ((h_div + 1) * (v_div + 1)) as usize);
        let mut indices: Vec<u32> = Vec::with_capacity(6 * h_div as usize * (v_div as usize - 1));

        let h_step: f32 = TAU / h_div as f32;
        let v_step: f32 = PI / v_div as f32;

        for i in 0..=v_div {
            let v_angle: f32 = PI / 2.0 - (i as f32) * v_step;
            let nxy: f32 = v_angle.cos();
            let nz: f32 = v_angle.sin();
            for j in 0..=h_div {
                let h_angle: f32 = (j as f32) * h_step;
                let nx: f32 = nxy * -h_angle.cos();
                let ny: f32 = nxy * -h_angle.sin();
                let x: f32 = nx * radius;
                let y: f32 = ny * radius;
                let z: f32 = nz * radius;
                let s: f32 = if j == h_div {
                    1.0
                } else {
                    (h_angle / TAU).rem_euclid(1.0)
                };
                let t: f32 = if i == 0 {
                    1.0
                } else {
                    // nz / 2.0 + 0.5 // Mercator
                    (v_angle / PI + 0.5).rem_euclid(1.0) // Equirectangular
                };

                vertices.push(x);
                vertices.push(y);
                vertices.push(z);
                vertices.push(nx);
                vertices.push(ny);
                vertices.push(nz);
                vertices.push(s);
                vertices.push(t);
            }
        }

        for i in 0..v_div {
            let k1 = i * (h_div + 1);
            let k2 = k1 + h_div + 1;

            for j in 0..h_div {
                if i != 0 {
                    indices.push(k1 + j);
                    indices.push(k2 + j);
                    indices.push(k1 + j + 1);
                }

                if i != (v_div - 1) {
                    indices.push(k1 + j + 1);
                    indices.push(k2 + j);
                    indices.push(k2 + j + 1);
                }
            }
        }

        unsafe { Self::load_gl(vertices.as_slice(), indices.as_slice()) }
    }

    // TODO this is a different format (no normals or texture coordinates)
    // try to make more consistent and flexible
    pub fn cube() -> Self {
        let vert: Vec<f32> = vec![
            -0.5, -0.5, -0.5, 0.5, -0.5, -0.5, 0.5, 0.5, -0.5, -0.5, 0.5, -0.5, -0.5, -0.5, 0.5,
            0.5, -0.5, 0.5, 0.5, 0.5, 0.5, -0.5, 0.5, 0.5,
        ];

        let indi: Vec<u32> = vec![
            0, 1, 3, 3, 1, 2, 1, 5, 2, 2, 5, 6, 5, 4, 6, 6, 4, 7, 4, 0, 7, 7, 0, 3, 3, 2, 7, 7, 2,
            6, 4, 5, 0, 0, 5, 1,
        ];

        let vertices = vert.as_slice();
        let indices = indi.as_slice();

        // TODO WRONG FORMAT FOR `load_gl`
        unsafe {
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
                std::mem::size_of_val(vertices) as isize,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                std::mem::size_of_val(indices) as isize,
                indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 12, std::ptr::null());
            gl::EnableVertexAttribArray(0);

            Self {
                vbo,
                vao,
                ebo,
                vertices: vertices.len() as _,
                indices: indices.len() as _,
            }
        }
    }

    // TODO this is a different format (no normals)
    // try to make more consistent and flexible
    pub fn quad() -> Self {
        let vert: Vec<f32> = vec![
            -1.0, 1.0, 0.0, 0.0, 1.0, // top left
            -1.0, -1.0, 0.0, 0.0, 0.0, // bottom left
            1.0, 1.0, 0.0, 1.0, 1.0, // top right
            1.0, -1.0, 0.0, 1.0, 0.0, // bottom right
        ];

        let indi: Vec<u32> = vec![0, 1, 2, 3, 2, 1];

        let vertices = vert.as_slice();
        let indices = indi.as_slice();

        // TODO WRONG FORMAT FOR `load_gl`
        unsafe {
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
                std::mem::size_of_val(vertices) as isize,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                std::mem::size_of_val(indices) as isize,
                indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 20, std::ptr::null());
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 20, 12 as *const _);
            gl::EnableVertexAttribArray(1);

            Self {
                vbo,
                vao,
                ebo,
                vertices: vertices.len() as _,
                indices: indices.len() as _,
            }
        }
    }

    pub unsafe fn load_gl(vertices: &[f32], indices: &[u32]) -> Self {
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
            std::mem::size_of_val(vertices) as isize,
            vertices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            std::mem::size_of_val(indices) as isize,
            indices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 32, std::ptr::null());
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 32, 12 as *const _);
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, 32, 24 as *const _);
        gl::EnableVertexAttribArray(2);

        //gl::BindVertexArray(0);
        //gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        //gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);

        Self {
            vbo,
            vao,
            ebo,
            vertices: vertices.len() as _,
            indices: indices.len() as _,
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawElements(
                gl::TRIANGLES,
                self.indices,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
            //gl::BindVertexArray(0);
        }
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ebo);
        }
    }
}
