use kiss3d::{window::Window};
use kiss3d::scene::SceneNode;

pub(crate) struct Environment {
    ground: SceneNode,
}

impl Environment {
    pub fn new(window: &mut Window) -> Self {
        Self {
            ground: window.add_cube(10.0, 0.1, 10.0)
        }
    }
}