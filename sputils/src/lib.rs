extern crate nalgebra as na;
use sofa_sys::*;

pub mod coord;
pub mod eph;
pub mod time;

#[inline(always)]
pub unsafe fn sofa_matrix(m: &[[f64; 3]; 3]) -> na::Matrix3<f64> {
    na::Matrix3::from_row_slice(std::mem::transmute::<_, &[f64; 9]>(m))
}

pub fn get_mjd(
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
    second: f64,
    dut1: f64,
) -> Option<(f64, f64, f64, f64)> {
    use std::convert::TryFrom;
    let utc = time::UTC::from_ymdhms(year, month, day, hour, minute, second)?;

    let mjd: (time::TT, time::UT1) = (
        time::TAI::try_from(utc).ok()?.into(),
        utc.try_into_ut1(dut1).ok()?,
    );
    //Some(mjd)
    Some((mjd.0 .0, mjd.0 .1, mjd.1 .1, 0.0))
}

pub fn gcrs_to_itrs(
    djmjd0: f64,
    tt: f64,
    date: f64,
    tut: f64,
    xp: f64,
    yp: f64,
    dx06: f64,
    dy06: f64,
) -> na::Matrix3<f64> {
    unsafe {
        let mut rc2ti: [[f64; 3]; 3] = [[0.0; 3]; 3];
        let mut rpom: [[f64; 3]; 3] = [[0.0; 3]; 3];
        let mut rc2it: [[f64; 3]; 3] = [[0.0; 3]; 3];
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        let mut rc2i: [[f64; 3]; 3] = [[0.0; 3]; 3];

        // =========================================== //
        // IAU 2006/2000A, CIO based, using X,Y series //
        // =========================================== //

        // CIP and CIO, IAU 2006/2000A
        iauXy06(djmjd0, tt, &mut x, &mut y);
        let s: f64 = iauS06(djmjd0, tt, x, y);

        // Add CIP corrections
        x += dx06;
        y += dy06;

        // GCRS to CIRS matrix
        iauC2ixys(x, y, s, rc2i.as_mut_ptr());

        // Earth rotation angle
        let era: f64 = iauEra00(djmjd0 + date, tut);

        // Form celestial-terrestrial matrix (no polar motion yet)
        iauCr(rc2i.as_mut_ptr(), rc2ti.as_mut_ptr());
        iauRz(era, rc2ti.as_mut_ptr());

        // Polar motion matrix (TIRS->ITRS, IERS 2003)
        iauPom00(xp, yp, iauSp00(djmjd0, tt), rpom.as_mut_ptr());

        // Form celestial-terrestrial matrix (including polar motion)
        iauRxr(rpom.as_mut_ptr(), rc2ti.as_mut_ptr(), rc2it.as_mut_ptr());

        sofa_matrix(&rc2it)
    }
}
