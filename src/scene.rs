// each scene will have data around the objects it has/lights etc, it will also have functions for "starting" the scene and "updating" the scene called each frame

use std::{collections::HashMap, time::Instant};

use beryllium::events::Event;

use crate::{camera::Camera, point_light::PointLight, render::Object};

#[derive(Clone)]
pub struct Settings {
    pub screen_width: i32,
    pub screen_height: i32,
    pub fovy: f32,
}

impl Settings {
    pub fn new(screen_width: i32, screen_height: i32, fovy: f32) -> Settings {
        return Settings {
            screen_width,
            screen_height,
            fovy,
        };
    }

    pub fn default() -> Settings {
        return Settings {
            screen_width: 1200,
            screen_height: 800,
            fovy: 45.0_f32.to_radians(),
        };
    }
}

pub struct Scene {
    pub scene_time: Instant,
    pub active_camera: String,
    pub object_map: HashMap<String, Object>,
    pub point_lights: Vec<PointLight>,
    pub cameras: HashMap<String, Camera>,
    pub settings: Settings,

    pub on_start: fn(&mut Scene),
    pub on_update: fn(&mut Scene),
    pub on_event: fn(&mut Scene, beryllium::events::Event),
}

impl Scene {
    pub fn new() -> Scene {
        fn no_op(sc: &mut Scene) {}
        fn no_op_event(sc: &mut Scene, ev: beryllium::events::Event) {}
        Scene {
            scene_time: Instant::now(),
            active_camera: "".to_string(),
            object_map: HashMap::new(),
            point_lights: Vec::new(),
            cameras: HashMap::new(),
            settings: Settings::default(),

            on_start: no_op,
            on_update: no_op,
            on_event: no_op_event,
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
