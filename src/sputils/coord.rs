// https://www.iausofa.org/2020_0721_C/sofa/sofa_pn_c.pdf
use sofa_sys::*;

// TODO add a **function** to get the matrix
// Use **methods** to convert
// Choose a time to store with this
#[derive(Debug, Copy, Clone)]
pub struct GCRS(glam::Vec3);

#[derive(Debug, Copy, Clone)]
pub struct ITRS(glam::Vec3);
