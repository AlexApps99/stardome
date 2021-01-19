// https://www.iausofa.org/2020_0721_C/sofa/sofa_pn_c.pdf
use sofa_sys::*;

// TODO add a **function** to get the matrix
// Use **methods** to convert
// Choose a time to store with this

pub type Position = na::Vector3<f64>;

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct BCRS(pub Position);

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct ICRS(pub Position);

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct GCRS(pub Position);

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct ITRS(pub Position);

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct TEME(pub Position);

impl TEME {
    // http://www.celestrak.com/publications/AIAA/2006-6753/AIAA-2006-6753-Rev2.pdf#page=32
    // TODO seems to be less accurate
    pub fn into_itrs_r(self, t: &crate::time::UT1, xp: f64, yp: f64) -> ITRS {
        let gmst = unsafe { iauGmst82(t.0, t.1) };
        let mut w: na::Matrix3<f64> = na::Matrix3::identity();
        unsafe {
            // TODO what is rot1, rot2
            iauRx(-xp, w.as_mut_ptr() as *mut _);
            iauRy(-yp, w.as_mut_ptr() as *mut _);
        }
        let rot3: na::Matrix3<f64> = unsafe {
            let mut tmp = na::Matrix3::identity();
            iauRz(-gmst, tmp.as_mut_ptr() as *mut _);
            tmp
        };

        ITRS(&w * &rot3 * &self.0)
    }

    // http://www.celestrak.com/publications/AIAA/2006-6753/AIAA-2006-6753-Rev2.pdf#page=32
    // https://github.com/astropy/astropy/blob/ad40565/astropy/coordinates/builtin_frames/intermediate_rotation_transforms.py#L26-L42
    pub fn teme_to_itrs_mat(t: &crate::time::UT1, xp: f64, yp: f64) -> na::Matrix3<f64> {
        let gst = unsafe { iauGmst82(t.0, t.1) };
        let mut pmmat = [[0.0_f64; 3]; 3];
        unsafe {
            iauPom00(xp, yp, 0.0, pmmat.as_mut_ptr());

            let mut rc2i: [[f64; 3]; 3] = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
            let mut rc2t: [[f64; 3]; 3] = [[0.0; 3]; 3];
            iauC2tcio(
                rc2i.as_mut_ptr(),
                gst,
                pmmat.as_mut_ptr(),
                rc2t.as_mut_ptr(),
            );
            crate::sofa_matrix(&rc2t)
        }
    }
}
