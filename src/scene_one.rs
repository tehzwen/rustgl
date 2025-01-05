use std::time::Instant;

use nalgebra::{Matrix4, Point, Point3, Vector3};
use sdl2::event::Event as SDL2Event;
use sdl2::{event::Event, keyboard::Keycode, mouse::MouseButton, Sdl};

use crate::directional_light::DirectionalLight;
use crate::texture;
use crate::{
    buffers,
    camera::Camera,
    material, obj,
    point_light::PointLight,
    raycast::{ray_intersect_bb_projection, Ray},
    render::{Model, Object},
    scene::{Scene, Settings},
    texture::loadTexture,
};

const ROTATION_SPEED: f32 = std::f32::consts::PI * 3.0;

pub fn scene_one(settings: Settings) -> Scene {
    let mut sc = Scene::new();
    sc.settings = settings;

    let mut dir_light = DirectionalLight::new();
    sc.directional_light = Some(dir_light);
    sc.directional_light.as_mut().unwrap().direction.z = -1.0;

    fn on_start(sc: &mut Scene) {
        sc.active_camera = "main".to_string();
        sc.cameras.insert(
            "main".to_string(),
            Camera::new(
                Point3::new(0.0, 300.0, 300.0),
                Point3::new(0.0, 0.0, 0.0),
                Vector3::new(0.0, 1.0, 0.0),
            ),
        );

        // OBJECTS
        let sphere_data = obj::parse_obj("resources/sphere-smooth.obj")
            .expect("unable to load obj file for sphere");

        let mut red_material = material::Physical::new(Vector3::new(0.5, 0.0, 0.0), 3.0, 0.1, 1.5);

        // unsafe {
        //     let tex = loadTexture("resources/checkers.png");
        //     let n_tex = loadTexture("resources/checkers-normal.png");
        //     red_material.diffuse_texture = Some(tex);
        //     red_material.normal_texture = Some(n_tex);
        // }

        let mut red: Object = Object::new(
            Model::new(),
            buffers::RenderBuffers::new(),
            sphere_data.vertices.clone(),
            sphere_data.normals.clone(),
            sphere_data.tex_coords.clone(),
            // Box::new(material::Physical::default()),
            Box::new(red_material),
        );
        red.model.translate(Vector3::new(0.0, 25.0, -15.0));
        sc.object_map.insert("red".to_string(), red);

        let mut green = Object::new(
            Model::new(),
            buffers::RenderBuffers::new(),
            sphere_data.vertices.clone(),
            sphere_data.normals.clone(),
            sphere_data.tex_coords.clone(),
            Box::new(material::Physical::new(
                Vector3::new(0.0, 0.5, 0.0),
                3.0,
                17.1,
                1.5,
            )),
        );
        green.model.translate(Vector3::new(65.0, 25.0, -15.0));
        sc.object_map.insert("green".to_string(), green);

        let mut blue = Object::new(
            Model::new(),
            buffers::RenderBuffers::new(),
            sphere_data.vertices.clone(),
            sphere_data.normals.clone(),
            sphere_data.tex_coords.clone(),
            Box::new(material::Physical::new(
                Vector3::new(0.0, 0.0, 0.5),
                7.0,
                0.1,
                1.5,
            )),
        );
        blue.model.translate(Vector3::new(-65.0, 25.0, -15.0));
        sc.object_map.insert("blue".to_string(), blue);

        let plane_data = obj::parse_obj("resources/plane.obj").expect("unable to load plane data");

        let mut plane_material =
            material::Physical::new(Vector3::new(0.8, 0.6, 0.0), 0.0, 150.1, 0.1);

        unsafe {
            let tex = loadTexture("resources/grey-rocks.png");
            let n_tex = loadTexture("resources/grey-rocks-normal.png");
            let arm_tex = loadTexture("resources/grey-rocks-arm.png");
            plane_material.diffuse_texture.tex = Some(tex);
            plane_material.diffuse_texture.enabled = true;
            plane_material.diffuse_texture.scale = 25.0;

            plane_material.normal_texture.tex = Some(n_tex);
            plane_material.normal_texture.enabled = true;
            plane_material.normal_texture.scale = 25.0;

            plane_material.arm_texture.tex = Some(arm_tex);
            plane_material.arm_texture.enabled = true;
            plane_material.arm_texture.scale = 25.0;
        }

        let mut main_plane = Object::new(
            Model::new(),
            buffers::RenderBuffers::new(),
            plane_data.vertices.clone(),
            plane_data.normals.clone(),
            plane_data.tex_coords.clone(),
            Box::new(plane_material),
        );
        // main_plane.model.scale(Vector3::new(100.0, 100.0, 100.0));
        sc.object_map.insert("main_plain".to_string(), main_plane);

        // Player cube
        let cube_data = obj::parse_obj("resources/cube.obj").unwrap();
        let mut player_cube = Object::new(
            Model::new(),
            buffers::RenderBuffers::new(),
            cube_data.vertices.clone(),
            cube_data.normals.clone(),
            cube_data.tex_coords.clone(),
            Box::new(material::Physical::new(
                Vector3::new(0.0, 0.0, 0.5),
                7.0,
                0.1,
                1.5,
            )),
        );
        player_cube.model.translate(Vector3::new(0.0, 0.0, 20.0));
        sc.object_map.insert("player".to_string(), player_cube);

        // initialize the objects
        for (key, object) in sc.object_map.iter_mut() {
            object.init();
        }

        // LIGHTS
        let mut light = PointLight::new();
        light.position.z = 40.0;
        light.position.y = 25.0;
        light.strength = 200.0;

        sc.point_lights.push(light);

        let mut light2 = PointLight::new();
        light2.position.z = 40.0;
        light2.position.x = 100.0;
        light2.position.y = 25.0;
        light2.strength = 50.0;

        sc.point_lights.push(light2);
    }

    sc.set_on_start(on_start);

    fn on_update(sc: &mut Scene) {
        // for object in &mut sc.objects {
        //     // Rotate the model
        //     object
        //         .model
        //         .rotate(Vector3::y_axis(), ROTATION_SPEED * 0.0005);

        //     // Calculate movement value
        //     // let move_val = f32::sin(sc.scene_time.elapsed().as_secs_f32() * 5.5) * 0.5;

        //     // // Translate the model
        //     // object.model.translate(Vector3::new(
        //     //     object.model.position.x,
        //     //     object.model.position.y + move_val,
        //     //     object.model.position.z,
        //     // ));

        // }

        let light2 = sc.point_lights.get_mut(1).unwrap();
        light2.position.x += f32::sin(sc.scene_time.elapsed().as_secs_f32() - 5.0) * 0.5;

        // have the camera look at the player cube
        let mut player = sc.object_map.get_mut(&"player".to_string()).unwrap();
        let mut main_camera = sc.cameras.get_mut(&"main".to_string()).unwrap();

        // main_camera.look_at_target(Point3::new(
        //     player.model.position.x,
        //     player.model.position.y,
        //     player.model.position.z,
        // ));

        // main_camera.position = Point3::new(
        //     player.model.position.x + 200.0, // Slight offset to ensure it's not at the same position
        //     player.model.position.x + 200.0,
        //     player.model.position.z + 200.0, // Slight offset along the z-axis
        // );

        // move the player toward the target
        player.model.position = player.model.position.lerp(&sc.player_target, 0.01);
        // println!("{} {}", sc.player_target, player.model.position);
    }
    sc.on_update = on_update;

    fn on_event(sc: &mut Scene, e: SDL2Event) {
        match e {
            SDL2Event::MouseMotion {
                x, y, xrel, yrel, ..
            } => {
                if let Some((intersect_point)) = ray_intersect_bb_projection(sc, x, y) {
                    let first_light = sc.point_lights.get_mut(0).unwrap();
                    first_light.position =
                        Vector3::new(intersect_point.x, first_light.position.y, intersect_point.z)
                }
            }
            SDL2Event::MouseButtonDown {
                mouse_btn: MouseButton::Left,
                x,
                y,
                ..
            } => {
                if let Some((intersect_point)) = ray_intersect_bb_projection(sc, x, y) {
                    let player = sc.object_map.get_mut(&"player".to_string()).unwrap();
                    sc.player_target = Vector3::new(
                        intersect_point.x,
                        player.model.position.y,
                        intersect_point.z,
                    );
                    // println!("{} {}", sc.player_target, player.model.position);
                    // let new = player.model.position.slerp(&sc.player_target, 1.0);
                    // println!("{}", new);
                    // player.model.position = Vector3::new(
                    //     intersect_point.x,
                    //     player.model.position.y,
                    //     intersect_point.z,
                    // )
                    // sc.player_target = Vector3::new(intersect_point.x, 0.0, intersect_point.z)
                }
            }
            SDL2Event::KeyDown {
                keycode: Some(keycode),
                ..
            } => {
                // Check for WASD keys
                match keycode {
                    Keycode::A => {
                        let plane = sc.object_map.get_mut(&"main_plain".to_string()).unwrap();
                        plane.material.toggle_map(material::TextureType::ARM);
                    }
                    Keycode::R => {
                        let plane = sc.object_map.get_mut(&"main_plain".to_string()).unwrap();
                        plane.material.toggle_map(material::TextureType::ARM);
                    }
                    Keycode::N => {
                        let plane = sc.object_map.get_mut(&"main_plain".to_string()).unwrap();
                        plane.material.toggle_map(material::TextureType::NORMAL);
                    }
                    _ => {}
                }
            }
            _ => (),
        }
    }
    sc.on_event = on_event;

    return sc;
}
