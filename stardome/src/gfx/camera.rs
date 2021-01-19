pub struct Camera {
    // TODO position using a coordinate system
    // f64
    pub position: na::Vector3<f32>,
    pub rx: f32,
    pub ry: f32,
    pub rz: f32,
    fov: f32,
    near_plane: f32,
    far_plane: f32,
}

impl Camera {
    pub fn new(position: na::Vector3<f32>, rx: f32, ry: f32, rz: f32) -> Self {
        Self {
            position,
            rx,
            ry,
            rz,
            fov: 90.0,
            near_plane: 1.0,
            far_plane: 500.0,
        }
    }

    // [[1,0,0,0],[0,"cx",-"sx",0],[0,"sx","cx",0],[0,0,0,1]]*[["cy",0,"sy",0],[0,1,0,0],[-"sy",0,"cy",0],[0,0,0,1]]*[["cz",-"sz",0,0],["sz", "cz", 0, 0],[0,0,1,0],[0,0,0,1]]
    pub fn rot_matrix(&self) -> na::Matrix4<f32> {
        let sx = (-self.rx).sin();
        let cx = (-self.rx).cos();
        let sy = (-self.ry).sin();
        let cy = (-self.ry).cos();
        let sz = (-self.rz).sin();
        let cz = (-self.rz).cos();
        na::Matrix4::new(
            cy * cz,
            -cy * sz,
            sy,
            0.0,
            cx * sz + cz * sx * sy,
            cx * cz - sx * sy * sz,
            -cy * sx,
            0.0,
            sx * sz - cx * cz * sy,
            cx * sy * sz + cz * sx,
            cx * cy,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        )
    }

    pub fn view_matrix(&self) -> na::Matrix4<f32> {
        self.rot_matrix().prepend_translation(&-self.position)
    }

    // https://docs.rs/glam/0.11.2/glam/struct.Mat4.html#method.perspective_rh_gl
    pub fn projection_matrix(&self, aspect_ratio: f32) -> na::Matrix4<f32> {
        let inv_length = 1.0 / (self.near_plane - self.far_plane);
        let f = 1.0 / (self.fov.to_radians() / 2.0).tan();
        let a = f / aspect_ratio;
        let b = (self.near_plane + self.far_plane) * inv_length;
        let c = (2.0 * self.near_plane * self.far_plane) * inv_length;
        na::Matrix4::new(
            a, 0.0, 0.0, 0.0, 0.0, f, 0.0, 0.0, 0.0, 0.0, b, c, 0.0, 0.0, -1.0, 0.0,
        )
    }

    pub fn matrix(&self, aspect_ratio: f32) -> na::Matrix4<f32> {
        self.projection_matrix(aspect_ratio) * self.view_matrix()
    }

    pub fn set_fov(&mut self, fov: f32) {
        if fov > 0.0 && fov < 180.0 {
            self.fov = fov;
        }
    }

    pub fn get_fov(&self) -> f32 {
        self.fov
    }

    pub fn set_clipping_planes(&mut self, near: f32, far: f32) {
        if near > 0.0 && (near < far || near < self.far_plane) {
            self.near_plane = near;
        }

        if far > self.near_plane {
            self.far_plane = far;
        }
    }

    pub fn get_clipping_planes(&self) -> (f32, f32) {
        (self.near_plane, self.far_plane)
    }
}
