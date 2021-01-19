use jpl_sys::*;

// TODO provide enums for all things

#[repr(transparent)]
pub struct JPL(*mut std::os::raw::c_void);

impl JPL {
    pub fn new() -> Result<Self, i32> {
        unsafe {
            let p = jpl_init_ephemeris(b"JPLEPH\0".as_ptr() as _, std::ptr::null_mut(), std::ptr::null_mut());
            if p.is_null() {
                Err(jpl_init_error_code())
            } else {
                Ok(Self(p))
            }
        }
    }

    fn pleph(&mut self, t: crate::time::TDB, ntarg: i32, ncent: i32) -> [f64; 3] {
        unsafe {
            let mut data = [0.0_f64; 6];
            jpl_pleph(self.0, t.0 + t.1, ntarg, ncent, data.as_mut_ptr(), 0);
            let mut pos = [0.0_f64; 3];
            pos.copy_from_slice(&data[..3]);
            pos
        }
    }

    fn pleph_vel(&mut self, t: crate::time::TDB, ntarg: i32, ncent: i32) -> ([f64; 3], [f64; 3]) {
        unsafe {
            let mut data = [0.0_f64; 6];
            jpl_pleph(self.0, t.0 + t.1, ntarg, ncent, data.as_mut_ptr(), 1);
            std::mem::transmute(data)
        }
    }

    pub fn moon(&mut self, t: crate::time::TDB) -> (crate::coord::GCRS, na::Vector3<f64>) {
        let pos = crate::coord::GCRS(na::Vector3::from_column_slice(&self.pleph(t, 10, 3)));
        let lib = na::Vector3::from_column_slice(&self.pleph(t, 15, 0));
        (pos, lib)
    }
}

impl Drop for JPL {
    fn drop(&mut self) {
        unsafe { jpl_close_ephemeris(self.0) }
    }
}