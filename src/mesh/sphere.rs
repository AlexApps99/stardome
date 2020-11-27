use std::f32::consts::{PI, TAU};

// h_div = latitude lines (sector), v_div = longitude lines (stack)
// http://www.songho.ca/opengl/gl_sphere.html
// Z axis may need to be swapped
pub fn sphere(radius: f32, h_div: u32, v_div: u32) -> (Vec<f32>, Vec<u32>) {
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
            let nx: f32 = nxy * h_angle.cos();
            let ny: f32 = nxy * h_angle.sin();
            let x: f32 = nx * radius;
            let y: f32 = ny * radius;
            let z: f32 = nz * radius;
            let s: f32 = (h_angle / TAU).rem_euclid(1.0);
            // Mercator
            //let t: f32 = nz / 2.0 + 0.5;
            // Equirectangular
            let t: f32 = (v_angle / PI + 0.5);

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

    (vertices, indices)
}
