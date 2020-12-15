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

// http://www.celestrak.com/publications/AIAA/2006-6753/AIAA-2006-6753-Rev2.pdf
// Page 32
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct TEME(pub Position);
