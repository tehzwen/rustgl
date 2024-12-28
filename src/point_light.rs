use nalgebra::Vector3;

pub struct PointLight {
    pub position: Vector3<f32>,
    pub color: Vector3<f32>,
    pub strength: f32,
}
impl PointLight {
    pub fn new() -> PointLight {
        PointLight {
            position: Vector3::new(0.0, 0.0, 0.0),
            color: Vector3::new(0.5, 0.5, 0.5),
            strength: 1.0,
        }
    }
}
