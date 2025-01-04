use std::time::Instant;

use nalgebra::{Point3, Vector3};

use crate::{
    buffers,
    camera::Camera,
    material, obj,
    point_light::PointLight,
    render::{Model, Object},
    scene::Scene,
};

const ROTATION_SPEED: f32 = std::f32::consts::PI * 3.0;

pub fn scene_one() -> Scene {
    let mut sc = Scene::new();

    fn on_start(sc: &mut Scene) {
        sc.active_camera = "main".to_string();
        sc.cameras.insert(
            "main".to_string(),
            Camera::new(
                Point3::new(0.0, 150.0, 300.0),
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
        main_plane.model.scale(Vector3::new(100.0, 100.0, 100.0));
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

        main_camera.position = Point3::new(
            player.model.position.x + 200.0, // Slight offset to ensure it's not at the same position
            player.model.position.x + 200.0,
            player.model.position.z + 200.0, // Slight offset along the z-axis
        );
    }
    sc.on_update = on_update;

    return sc;
}
