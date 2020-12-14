// https://www.iausofa.org/2020_0721_C/sofa/sofa_pn_c.pdf
//use sofa_sys::*;

// TODO add a **function** to get the matrix
// Use **methods** to convert
// Choose a time to store with this

// Consider polar (or its orbital equivalent)
pub type Position = na::Vector3<f64>;

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct ICRS(Position);

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct BCRS(Position);

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct GCRS(Position);

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct CIRS(Position);

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct TIRS(Position);

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct ITRS(Position);

// Probably some more like ecliptic, galactic, equatorial etc
// See the manual (NOT the cookbooks)
