#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::single_match)]
#![allow(unused_imports)]
#![allow(clippy::zero_ptr)]

mod buffers;
mod camera;
mod collision;
mod material;
mod obj;
mod point_light;
mod raycast;
mod render;
mod scene;
mod scene_one;
mod shader;
mod vertex;
mod window;

use beryllium::events::{Event, SDLK_a, SDLK_d, SDLK_s, SDLK_w, SDL_Keycode, SDLK_1};
use core::{
    convert::{TryFrom, TryInto},
    mem::{size_of, size_of_val},
};
use nalgebra::{Matrix4, Point3, Vector3};
use point_light::PointLight;
use scene::Scene;

use ogl33::*;
use render::{Model, Object};
use std::{collections::HashMap, ffi::CString, time::Instant};

const WINDOW_TITLE: &str = "VS Clone";
const WINDOW_HEIGHT: i32 = 1000;
const WINDOW_WIDTH: i32 = 1000;

fn main() {
    let mut gw = window::GameWindow::new(WINDOW_TITLE.to_string(), WINDOW_WIDTH, WINDOW_HEIGHT);
    gw.init();
    let gl_win = gw.window.expect("failed to get opengl window");
    let sdl = gw.ctx.expect("failed to get window context");
    let active_scene: String = "scene-01".to_string();
    let mut scenes: HashMap<String, Scene> = HashMap::new();
    scenes.insert("scene-01".to_string(), scene_one::scene_one());

    let mut shader_program: u32 = 0;

    unsafe {
        load_gl_with(|f_name| gl_win.get_proc_address(f_name.cast()));
        let blinn_shader = shader::Shader::new("pbr".to_string());
        shader_program = blinn_shader.program;

        glClearColor(0.2, 0.3, 0.3, 1.0);
    }

    let scene = scenes
        .get_mut(&active_scene)
        .expect("unable to get active-scene");

    scene.start();

    'main_loop: loop {
        let aspect: f32 = (WINDOW_WIDTH / WINDOW_HEIGHT) as f32;
        let projection = Matrix4::new_perspective(aspect, 45.0_f32.to_radians(), 0.1, 10000.0);

        // get camera
        let mut main_camera = scene.cameras.get_mut(&scene.active_camera).unwrap();

        let view_projection_matrix = projection * main_camera.view_matrix();
        let inverse_vp_matrix = view_projection_matrix.try_inverse().unwrap();

        // handle events this frame
        while let Some((event, _timestamp)) = sdl.poll_events() {
            match event {
                Event::Quit => {
                    break 'main_loop;
                }
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
                        println!("{}, {}, {}", button, x, y);

                        let ndc_x = (x as f32 / WINDOW_HEIGHT as f32) * 2.0 - 1.0;
                        let ndc_y = 1.0 - (y as f32 / WINDOW_WIDTH as f32) * 2.0; // Flip Y-axis

                        // Convert NDC to world coordinates
                        let ndc_point = Point3::new(ndc_x, ndc_y, -1.0);
                        let world_point = inverse_vp_matrix.transform_point(&ndc_point);
                        let ray_direction = (world_point - main_camera.position).normalize();

                        // Create ray
                        let ray = raycast::Ray {
                            origin: main_camera.position,
                            direction: ray_direction,
                        };

                        // get the plane's bounding box and compare it here
                        let floor = scene.object_map.get_mut(&"main_plain".to_string()).unwrap();

                        // Check for intersection
                        if let Some((t_min, _)) = ray.intersect_bounding_box(&floor.bounding_box) {
                            println!("Intersection detected at t = {}", t_min);

                            // move the first light to that position
                            let first_light = scene.point_lights.get_mut(0).unwrap();
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
                        let player_cube = scene.object_map.get_mut(&"player".to_string()).unwrap();
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
                                let player_cube =
                                    scene.object_map.get_mut(&"player".to_string()).unwrap();
                                let move_speed: f32 = 0.5;
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

        // now the events are clear, update our scene
        (scene.on_update)(scene);

        // and then draw!
        unsafe {
            // Clear the screen
            glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
            glEnable(GL_DEPTH_TEST);

            // Use the shader program
            glUseProgram(shader_program);

            // Get uniform locations for projection, view, and model matrices
            let projection_loc = shader::get_shader_location(shader_program, "projection");
            let model_loc = shader::get_shader_location(shader_program, "model");

            // Create the orthogonal projection matrix
            // let size = 100.0;
            // let projection: Matrix4<f32> =
            //     Matrix4::new_orthographic(-size, size, -size, size, 0.1, 10000.0);

            glUniformMatrix4fv(projection_loc, 1, GL_FALSE, projection.as_ptr());
            scene
                .cameras
                .get_mut(&scene.active_camera)
                .unwrap()
                .link_shader(shader_program);
            for (index, pl) in scene.point_lights.iter_mut().enumerate() {
                pl.link_shader(shader_program, index.try_into().unwrap());
            }

            glUniform1i(
                shader::get_shader_location(shader_program, "numPointLights"),
                scene.point_lights.len().try_into().unwrap(),
            );

            // now loop over the objects and get specific uniform fields for the object
            for (_, object) in &mut scene.object_map.iter_mut() {
                let model = object.model.get_model_matrix();
                // Send the matrices to the shader
                glUniformMatrix4fv(model_loc, 1, GL_FALSE, model.as_ptr());

                // link the material here
                object.material.link_shader(shader_program);

                // Bind buffers and draw
                object.buffers.bind();
                glDrawArrays(GL_TRIANGLES, 0, object.buffers.size);
                object.buffers.unbind();
            }
        }
        gl_win.swap_window();
    }
}
