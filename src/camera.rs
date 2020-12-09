pub struct Camera {
    pub position: glam::Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
    fov: f32,
    near_plane: f32,
    far_plane: f32,
}

impl Camera {
    pub fn new(position: glam::Vec3, yaw: f32, pitch: f32, roll: f32) -> Self {
        Self {
            position,
            yaw,
            pitch,
            roll,
            fov: 90.0,
            near_plane: 0.01,
            far_plane: 1000.0,
        }
    }

    pub fn rot_matrix(&self) -> glam::Mat4 {
        glam::Mat4::from_rotation_ypr(self.yaw, self.pitch, self.roll)
    }

    pub fn view_matrix(&self) -> glam::Mat4 {
        self.rot_matrix() * glam::Mat4::from_translation(self.position)
    }

    pub fn projection_matrix(&self, aspect_ratio: f32) -> glam::Mat4 {
        glam::Mat4::perspective_rh_gl(
            self.fov.to_radians(),
            aspect_ratio,
            self.near_plane,
            self.far_plane,
        )
    }

    pub fn matrix(&self, aspect_ratio: f32) -> glam::Mat4 {
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
