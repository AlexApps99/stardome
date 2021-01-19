extern crate nalgebra as na;
use sofa_sys::DAS2R;
use sputils::coord::{ITRS, TEME};
use sputils::time::{UT1, UTC};

fn main() {
    let xp = -0.140682 * DAS2R;
    let yp = 0.333309 * DAS2R;
    let dut1 = -0.439961;
    let dat = 32.0;
    let t = UTC::from_ymdhms(2004, 04, 06, 07, 51, 28.386)
        .unwrap()
        .try_into_ut1(dut1)
        .unwrap();
    let r_teme = TEME(na::Vector3::new(
        5094.18016210,
        6127.64465950,
        6380.34453270,
    ));
    let v_teme = TEME(na::Vector3::new(-4.746131487, 0.785818041, 5.531931288));

    let r_itrf = r_teme.into_itrs_r(&t, xp, yp);
    //let v_itrf = v_teme.into_itrs_v(&t, xp, yp);

    let tf = TEME::teme_to_itrs_mat(&t, xp, yp);
    println!("{}", r_itrf.0);

    println!("{}", tf * r_teme.0);
    println!("{}", tf);
}
