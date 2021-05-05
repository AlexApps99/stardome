#![allow(unused_variables, dead_code)]

use imgui::im_str;
use stardome::StarDome;
extern crate nalgebra as na;

#[cfg(target_os = "emscripten")]
extern "C" {
    pub fn emscripten_set_main_loop_arg(
        func: unsafe extern "C" fn(*mut std::ffi::c_void),
        arg: *mut std::ffi::c_void,
        fps: i32,
        simulate_infinite_loop: i32,
    );
    pub fn emscripten_cancel_main_loop();
}

pub struct State {
    pub win: bool,
    pub sd: StarDome,
    pub earth: stardome::Planet,
    pub iss_label: stardome::Text,
    pub sun_line: stardome::Points,
}

pub fn main() {
    let mut sd = StarDome::new().unwrap();

    let earth = stardome::Planet {
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
        tf: na::Matrix4::identity(),
    };

    let iss_label = stardome::Text {
        position: na::Vector3::zeros(),
        color: 0xFF00FF80,
        text: "International Space Station".to_string(),
    };

    let mut sun_line = stardome::Points::new(0xFF8000FF, 4.0, true, vec![na::Vector3::zeros(); 2]);
    sd.sun = na::Vector3::new(149597.87, 0., 0.);
    sun_line.modify_points(|p| {
        p[1] = sd.get_sun_dir() * 100.0;
    });

    let mut state = State {
        win: true,
        sd,
        earth,
        iss_label,
        sun_line,
    };

    #[cfg(not(target_os = "emscripten"))]
    loop {
        // No way to return stuff yet (too hacked)
        unsafe {
            loop_stuff(&mut state as *mut State as *mut _);
        }
    }
    #[cfg(target_os = "emscripten")]
    unsafe {
        emscripten_set_main_loop_arg(loop_stuff, &mut state as *mut State as *mut _, 0, 1);
    }
    #[cfg(target_os = "emscripten")]
    std::mem::forget(state);
}

#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn loop_stuff(state: *mut std::ffi::c_void) {
    let state = &mut *(state as *mut State);
    state.sd.draw(&mut state.earth);
    state.sd.draw(&mut state.iss_label);
    state.sd.draw(&mut state.sun_line);

    let mut rx = state.sd.cam.rx;
    let mut ry = state.sd.cam.ry;
    let mut rz = state.sd.cam.rz;
    let mut pos = [
        state.sd.cam.position.x,
        state.sd.cam.position.y,
        state.sd.cam.position.z,
    ];
    let mut fov = state.sd.cam.get_fov().to_radians();
    let mut win = state.win;

    #[allow(clippy::blocks_in_if_conditions)]
    if state
        .sd
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
                });
        })
        .is_err()
    {
        #[cfg(target_op = "emscripten")]
        emscripten_cancel_main_loop();
    }
    state.win = win;
    state.sd.cam.position.x = pos[0];
    state.sd.cam.position.y = pos[1];
    state.sd.cam.position.z = pos[2];
    state.sd.cam.rx = rx;
    state.sd.cam.ry = ry;
    state.sd.cam.rz = rz;
    state.sd.cam.set_fov(fov.to_degrees());
}
