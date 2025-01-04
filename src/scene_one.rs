use std::time::Instant;

use beryllium::events::Event;
use nalgebra::{Matrix4, Point3, Vector3};

use crate::{
    buffers,
    camera::Camera,
    material, obj,
    point_light::PointLight,
    raycast::Ray,
    render::{Model, Object},
    scene::{Scene, Settings},
};

const ROTATION_SPEED: f32 = std::f32::consts::PI * 3.0;

pub fn scene_one(settings: Settings) -> Scene {
    let mut sc = Scene::new();
    sc.settings = settings;

    fn on_start(sc: &mut Scene) {
        sc.active_camera = "main".to_string();
        sc.cameras.insert(
            "main".to_string(),
            Camera::new(
                Point3::new(0.0, 105.0, 300.0),
                Point3::new(0.0, 0.0, 0.0),
                Vector3::new(0.0, 1.0, 0.0),
            ),
        );

        // OBJECTS
        let sphere_data = obj::parse_obj("resources/sphere-smooth.obj")
            .expect("unable to load obj file for sphere");

        let mut red: Object = Object::new(
            Model::new(),
            buffers::RenderBuffers::new(),
            sphere_data.vertices.clone(),
            sphere_data.normals.clone(),
            // Box::new(material::Physical::default()),
            Box::new(material::Physical::new(
                Vector3::new(0.5, 0.0, 0.0),
                3.0,
                0.1,
                1.5,
            )),
        );
        red.model.translate(Vector3::new(0.0, 25.0, -15.0));
        sc.object_map.insert("red".to_string(), red);

        let mut green = Object::new(
            Model::new(),
            buffers::RenderBuffers::new(),
            sphere_data.vertices.clone(),
            sphere_data.normals.clone(),
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
        let mut main_plane = Object::new(
            Model::new(),
            buffers::RenderBuffers::new(),
            plane_data.vertices.clone(),
            plane_data.normals.clone(),
            Box::new(material::Physical::new(
                Vector3::new(0.8, 0.6, 0.0),
                0.0,
                150.1,
                0.0001,
            )),
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
        light.strength = 2.0;

        sc.point_lights.push(light);

        let mut light2 = PointLight::new();
        light2.position.z = 40.0;
        light2.position.x = 100.0;
        light2.position.y = 25.0;
        light2.strength = 0.5;

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

        main_camera.look_at_target(Point3::new(
            player.model.position.x,
            player.model.position.y,
            player.model.position.z,
        ));

        // main_camera.position = Point3::new(
        //     player.model.position.x + 200.0, // Slight offset to ensure it's not at the same position
        //     player.model.position.x + 200.0,
        //     player.model.position.z + 200.0, // Slight offset along the z-axis
        // );
    }
    sc.on_update = on_update;

    fn on_event(sc: &mut Scene, e: Event) {
        match e {
            Event::MouseButton {
                win_id,
                mouse_id,
                button,
                pressed,
                clicks,
                x,
                y,
            } => {
                if pressed {
                    let aspect: f32 = (sc.settings.screen_width / sc.settings.screen_height) as f32;
                    let projection =
                        Matrix4::new_perspective(aspect, sc.settings.fovy, 0.1, 10000.0);
                    let main_camera = sc.cameras.get_mut(&sc.active_camera).unwrap();

                    let view_projection_matrix = projection * main_camera.view_matrix();
                    let inverse_vp_matrix = view_projection_matrix.try_inverse().unwrap();

                    println!("{}, {}, {}", button, x, y);

                    let ndc_x = (x as f32 / sc.settings.screen_width as f32) * 2.0 - 1.0;
                    let ndc_y = 1.0 - (y as f32 / sc.settings.screen_height as f32) * 2.0; // Flip Y-axis

                    // Convert NDC to world coordinates
                    let ndc_point = Point3::new(ndc_x, ndc_y, -1.0);
                    let world_point = inverse_vp_matrix.transform_point(&ndc_point);
                    let ray_direction = (world_point - main_camera.position).normalize();

                    // Create ray
                    let ray = Ray {
                        origin: main_camera.position,
                        direction: ray_direction,
                    };

                    // get the plane's bounding box and compare it here
                    let floor = sc.object_map.get_mut(&"main_plain".to_string()).unwrap();

                    // Check for intersection
                    if let Some((t_min, _)) = ray.intersect_bounding_box(&floor.bounding_box) {
                        println!("Intersection detected at t = {}", t_min);

                        // move the first light to that position
                        let first_light = sc.point_lights.get_mut(0).unwrap();
                        first_light.position =
                            Vector3::new(t_min.x, first_light.position.y, t_min.z)
                    } else {
                        println!("No intersection");
                    }
                }
            }
            Event::Key {
                win_id,
                pressed,
                repeat,
                scancode,
                keycode,
                modifiers,
            } => {
                if pressed {
                    // Check for WASD keys
                    let player_cube = sc.object_map.get_mut(&"player".to_string()).unwrap();
                    let move_speed: f32 = 2.5;

                    match keycode {
                        SDLK_w => {
                            println!("W key pressed");
                            // handle W key press
                            player_cube.model.translate(Vector3::new(
                                player_cube.model.position.x,
                                player_cube.model.position.y,
                                player_cube.model.position.z + move_speed,
                            ));
                        }
                        SDLK_a => {
                            println!("A key pressed");
                            // handle A key press
                            player_cube.model.translate(Vector3::new(
                                player_cube.model.position.x + move_speed,
                                player_cube.model.position.y,
                                player_cube.model.position.z,
                            ));
                        }
                        SDLK_s => {
                            println!("S key pressed");
                            // handle S key press
                            player_cube.model.translate(Vector3::new(
                                player_cube.model.position.x,
                                player_cube.model.position.y,
                                player_cube.model.position.z - move_speed,
                            ));
                        }
                        SDLK_d => {
                            println!("D key pressed");
                            // handle D key press
                            player_cube.model.translate(Vector3::new(
                                player_cube.model.position.x - move_speed,
                                player_cube.model.position.y,
                                player_cube.model.position.z,
                            ));
                        }
                        _ => {}
                    }
                }
            }
            _ => (),
        }
    }
    sc.on_event = on_event;

    return sc;
}
