#![allow(unused_variables, dead_code)]

use stardome::StarDome;

fn main() {
    use imgui::im_str;
    let mut sd = StarDome::new().unwrap();

    let mut win = true;
    let mut pos = [sd.cam.position.x, sd.cam.position.y, sd.cam.position.z];
    let mut rx = sd.cam.rx;
    let mut ry = sd.cam.ry;
    let mut rz = sd.cam.rz;
    let mut fov = sd.cam.get_fov().to_radians();
    loop {
        // This makes some things a bit problematic because of borrowing
        // Consider using this for imgui only, and having rest of stuff just be functions
        if sd
            .frame(|ui| {
                // TODO put all code for stuff here
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
            break;
        }
        sd.cam.position.x = pos[0];
        sd.cam.position.y = pos[1];
        sd.cam.position.z = pos[2];
        sd.cam.rx = rx;
        sd.cam.ry = ry;
        sd.cam.rz = rz;
        sd.cam.set_fov(fov.to_degrees());
    }
}
