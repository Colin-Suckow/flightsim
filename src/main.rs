extern crate kiss3d;
extern crate nalgebra as na;

mod environment;
mod drone;

use drone::Drone;
use environment::Environment;
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::window::{State, Window};
use na::{UnitQuaternion, Vector3};

struct AppState {
    //environment: Environment,
    drone: Drone,
}

impl State for AppState {
    fn step(&mut self, _: &mut Window) {
        
    }
}

fn main() {
    let mut window = Window::new("Kiss3d: wasm example");

    window.set_light(Light::StickToCamera);

    //let environment = Environment::new(&mut window);
    let drone = Drone::new(&mut window);

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);
    let state = AppState { drone };

    window.render_loop(state)
}