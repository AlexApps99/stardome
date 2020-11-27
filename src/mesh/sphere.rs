use std::f32::consts::{PI, TAU};

// h_div = latitude lines (sector), v_div = longitude lines (stack)
// http://www.songho.ca/opengl/gl_sphere.html
// add the comments
pub fn sphere(radius: f32, h_div: u32, v_div: u32) -> (Vec<f32>, Vec<u32>) {
    let mut vertices: Vec<f32> = Vec::with_capacity(8 * ((h_div + 1) * (v_div + 1)) as usize);
    let mut indices: Vec<u32> = Vec::with_capacity(6 * h_div as usize * (v_div as usize - 1));

    let length_inv: f32 = 1.0 / radius;

    let h_step: f32 = TAU / h_div as f32;
    let v_step: f32 = PI / v_div as f32;

    for i in 0..=v_div {
        let v_angle: f32 = PI / 2.0 - (i as f32) * v_step;
        let xy: f32 = radius * v_angle.cos();
        let z: f32 = radius * v_angle.sin();
        for j in 0..=h_div {
            let h_angle: f32 = (j as f32) * h_step;
            let x: f32 = xy * h_angle.cos();
            let y: f32 = xy * h_angle.sin();
            vertices.push(x);
            vertices.push(y);
            vertices.push(z);

            let nx: f32 = x * length_inv;
            let ny: f32 = y * length_inv;
            let nz: f32 = z * length_inv;
            vertices.push(nx);
            vertices.push(ny);
            vertices.push(nz);

            let s: f32 = (j as f32) / (h_div as f32);
            let t: f32 = (i as f32) / (v_div as f32);
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

    (vertices, indices)
}
