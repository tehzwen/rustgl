// each scene will have data around the objects it has/lights etc, it will also have functions for "starting" the scene and "updating" the scene called each frame

use std::{collections::HashMap, time::Instant};

use crate::{camera::Camera, point_light::PointLight, render::Object};

pub struct Scene {
    pub scene_time: Instant,
    pub active_camera: String,
    pub object_map: HashMap<String, Object>,
    pub point_lights: Vec<PointLight>,
    pub cameras: HashMap<String, Camera>,

    pub on_start: fn(&mut Scene),
    pub on_update: fn(&mut Scene),
}

impl Scene {
    pub fn new() -> Scene {
        fn no_op(sc: &mut Scene) {}
        Scene {
            scene_time: Instant::now(),
            active_camera: "".to_string(),
            object_map: HashMap::new(),
            point_lights: Vec::new(),
            cameras: HashMap::new(),

            on_start: no_op,
            on_update: no_op,
        }
    }

    pub fn set_on_start(&mut self, on_start: fn(&mut Scene)) {
        self.on_start = on_start;
    }

    pub fn set_on_update(&mut self, on_update: fn(&mut Scene)) {
        self.on_update = on_update;
    }

    pub fn get_active_cam(&mut self) -> &Camera {
        return self.cameras.get_mut(&self.active_camera).unwrap();
    }

    pub fn start(&mut self) {
        (self.on_start)(self)
    }
}
