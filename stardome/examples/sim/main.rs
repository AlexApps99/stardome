#![allow(unused_variables, dead_code)]

use stardome::StarDome;
extern crate nalgebra as na;

fn mat3_to_mat4(m: &na::Matrix3<f64>) -> na::Matrix4<f64> {
    let mut m: na::Matrix4<f64> = m.fixed_resize(0.0);
    m.m44 = 1.0;
    m
}

fn rows_to_mat3(r: &[[f64; 3]; 3]) -> na::Matrix3<f64> {
    nalgebra::Matrix3::from_row_slice(unsafe {
        std::mem::transmute::<&[[f64; 3]; 3], &[f64; 9]>(&r)
    })
}

fn get_mat(et: f64) -> na::Matrix4<f64> {
    let m = rspice::pxform("ITRF93", "J2000", et);
    mat3_to_mat4(&rows_to_mat3(&m))
}

fn get_moon_pos(et: f64) -> na::Vector3<f64> {
    // This is km
    let (t, _) = rspice::spkpos("MOON", et, "J2000", "NONE", "EARTH");
    na::Vector3::new(t[0], t[1], t[2]) / 1000.0
}

fn get_moon_mat(et: f64) -> na::Matrix4<f64> {
    let m = rspice::pxform("MOON_ME", "J2000", et);
    mat3_to_mat4(&rows_to_mat3(&m)).append_translation(&get_moon_pos(et))
}

fn get_iss_pos(et: f64) -> na::Vector3<f64> {
    // This is km
    let (t, _) = rspice::spkgps(-6969, et, "J2000", 399);
    na::Vector3::new(t[0], t[1], t[2]) / 1000.0
}

fn get_iss_line(et: f64) -> Vec<na::Vector3<f32>> {
    let mut v = Vec::with_capacity(60 * 60);
    for x in -(60 * 60)..(60 * 60) {
        v.push(na::convert(get_iss_pos(et + (x as f64))))
    }
    v
}

fn get_sun_pos(et: f64) -> na::Vector3<f64> {
    // This is km
    let (t, _) = rspice::spkpos("SUN", et, "J2000", "NONE", "EARTH");
    na::Vector3::new(t[0], t[1], t[2]) / 1000.0
}

fn main() {
    // TODO LRO Hubble
    // avoid crash when data ran out (everything not just iss)
    // Use error checking version not hacky one
    // Verify position of everything (sun and moon) by looking at eclipse
    // Oh and stars
    rspice::furnsh("../cspice/kernels/all.tm");
    use imgui::im_str;
    let mut sd = StarDome::new().unwrap();

    let mut win = true;
    let mut pos = [sd.cam.position.x, sd.cam.position.y, sd.cam.position.z];
    let mut rx = sd.cam.rx;
    let mut ry = sd.cam.ry;
    let mut rz = sd.cam.rz;
    let mut fov = sd.cam.get_fov().to_radians();

    let beninging = std::time::Instant::now();
    //let (djmjd0, tt, date, tut) = sputils::get_mjd(2020, 12, 10, 8, 0, 0.0, -0.2).unwrap();
    let et = rspice::str2et("2021-01-18T12:00:00");

    let mut earth = stardome::Planet {
        r_equatorial: 6.3781,
        r_polar: 6.3568,
        lighting: true,
        texture: stardome::Texture::open("img/gen/earth.png").unwrap(),
        atm: Some(stardome::Atmosphere {
            offset: 0.06,
            sun_intensity: 20.0,
            scale_height_r: 7994.0,
            scatter_coeff_r: na::Vector3::new(3.8e-6, 13.5e-6, 33.1e-6),
            scale_height_m: 1200.0,
            scatter_coeff_m: na::Vector3::from_element(21e-6),
            asymmetry_m: 0.76,
        }),
        clouds: None,
        tf: get_mat(et),
    };

    // TODO spice provides constants get it from them
    let mut moon = stardome::Planet {
        r_equatorial: 1.7381,
        r_polar: 1.736,
        lighting: true,
        texture: stardome::Texture::open("img/gen/moon.png").unwrap(),
        atm: None,
        clouds: None,
        tf: get_moon_mat(et),
    };

    let mut iss_label = stardome::Text {
        position: na::Vector3::zeros(),
        color: 0xFF00FF80,
        text: "International Space Station".to_string(),
    };

    let mut moon_label = stardome::Text {
        position: na::Vector3::zeros(),
        color: 0xFFFFFF20,
        text: "Moon".to_string(),
    };

    let mut test_line = stardome::Points::new(0xABCDEFFF, 4.0, true, vec![na::Vector3::zeros(); 2]);
    let mut sun_line = stardome::Points::new(0xFF8000FF, 4.0, true, vec![na::Vector3::zeros(); 2]);
    let mut iss = stardome::Points::new(0xFF00FF80, 8.0, false, vec![na::Vector3::zeros()]);
    let mut orbit = stardome::Points::new(0x00FF0080, 1.0, true, get_iss_line(et));
    loop {
        sd.sun = get_sun_pos(et);
        let tw = beninging.elapsed().as_secs_f64() * 60.0;
        // This makes some things a bit problematic because of borrowing
        // Consider using this for imgui only, and having rest of stuff just be functions
        sd.draw(&mut earth);
        sd.draw(&mut moon);
        test_line.modify_points(|p| {
            p[1].copy_from(&na::convert::<na::Vector3<f64>, na::Vector3<f32>>(
                get_moon_pos(et + tw),
            ));
        });
        sun_line.modify_points(|p| {
            p[1] = sd.get_sun_dir() * 100.0;
        });
        iss.modify_points(|p| {
            p[0].copy_from(&na::convert::<na::Vector3<f64>, na::Vector3<f32>>(
                get_iss_pos(et + tw),
            ));
        });
        iss_label.position = na::convert(iss.get_points()[0].clone_owned());
        moon_label.position = na::convert(get_moon_pos(et + tw));
        sd.draw(&mut test_line);
        sd.draw(&mut iss);
        sd.draw(&mut orbit);
        sd.draw(&mut iss_label);
        sd.draw(&mut moon_label);
        sd.draw(&mut sun_line);

        if sd
            .frame(|ui| {
                if win {
                    ui.show_demo_window(&mut win);
                }
                imgui::Window::new(im_str!("Camera"))
                    .size([420.0, 250.0], imgui::Condition::FirstUseEver)
                    .build(&ui, || {
                        imgui::InputFloat3::new(&ui, im_str!("Position"), &mut pos).build();
                        ui.separator();
                        imgui::AngleSlider::new(im_str!("RX"))
                            .min_degrees(0.0)
                            .build(&ui, &mut rx);
                        imgui::AngleSlider::new(im_str!("RY"))
                            .min_degrees(0.0)
                            .build(&ui, &mut ry);
                        imgui::AngleSlider::new(im_str!("RZ"))
                            .min_degrees(0.0)
                            .build(&ui, &mut rz);
                        ui.separator();
                        imgui::AngleSlider::new(im_str!("FoV"))
                            .min_degrees(1.0)
                            .max_degrees(179.0)
                            .flags(imgui::SliderFlags::ALWAYS_CLAMP)
                            .build(&ui, &mut fov);

                        ui.text(format!("Moon: {:?}", test_line.get_points()[1].as_slice()));
                        ui.text(format!("Time: {}", et + tw));
                    });
            })
            .is_err()
        {
            break;
        }
        sd.cam.position.x = pos[0];
        sd.cam.position.y = pos[1];
        sd.cam.position.z = pos[2];
        sd.cam.rx = rx;
        sd.cam.ry = ry;
        sd.cam.rz = rz;
        sd.cam.set_fov(fov.to_degrees());
        earth.tf = get_mat(et + tw);
        moon.tf = get_moon_mat(et + tw);
    }
}
