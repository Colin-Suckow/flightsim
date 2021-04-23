use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use na::{UnitQuaternion, Vector3};
use nalgebra::{Translation3, Unit};
//phantom is 289.5mm square
pub struct Drone {
    model: SceneNode,
}

impl Drone {
    pub fn new(window: &mut Window) -> Self {
        let mut container = window.add_group();
        container
            .add_cylinder(0.01, 0.409)
            .prepend_to_local_rotation(&UnitQuaternion::from_axis_angle(
                &Unit::new_normalize(Vector3::new(1.0, 0.0, 0.0)),
                1.5708,
            ));
        container
            .add_cylinder(0.01, 0.409)
            .prepend_to_local_rotation(&UnitQuaternion::from_axis_angle(
                &Unit::new_normalize(Vector3::new(0.0, 0.0, 1.0)),
                1.5708,
            ));

        container
            .add_cube(0.1, 0.03, 0.1);

        container.add_cylinder(0.02, 0.03).append_translation(&Translation3::new(0.0, 0.0, 0.409 / 2.0));
        container.add_cylinder(0.02, 0.03).append_translation(&Translation3::new(0.0, 0.0, -0.409 / 2.0));
        container.add_cylinder(0.02, 0.03).append_translation(&Translation3::new(0.409 / 2.0, 0.0, 0.0));
        container.add_cylinder(0.02, 0.03).append_translation(&Translation3::new(-0.409 / 2.0, 0.0, 0.0));

        Self { model: container }
    }
}
