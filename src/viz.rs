use kiss3d::window::Window;
use kiss3d::light::Light;

pub(crate) fn viz() {
    let mut win = Window::new("petratools debug viz");
    let mut c = win.add_cube(1.0, 1.0, 1.0);
    win.set_light(Light::StickToCamera);

    while win.render() {
    }
}
