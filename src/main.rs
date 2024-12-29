#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::single_match)]
#![allow(unused_imports)]
#![allow(clippy::zero_ptr)]

mod buffers;
mod camera;
mod material;
mod obj;
mod point_light;
mod render;
mod shader;
mod vertex;
mod window;

use beryllium::events::Event;
use core::{
    convert::{TryFrom, TryInto},
    mem::{size_of, size_of_val},
};
use nalgebra::{Matrix4, Point3, Vector3};
use point_light::PointLight;

use ogl33::*;
use render::{Model, Object};
use std::{ffi::CString, time::Instant};

const WINDOW_TITLE: &str = "VS Clone";

fn main() {
    let mut gw = window::GameWindow::new(WINDOW_TITLE.to_string(), 800, 600);
    gw.init();
    let gl_win = gw.window.expect("failed to get opengl window");
    let sdl = gw.ctx.expect("failed to get window context");
    let mut shader_program: u32 = 0;

    let cam = camera::Camera::new(
        Point3::new(0.0, 15.0, 30.0),
        Point3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );

    // load sphere obj
    let sphere_data =
        obj::parse_obj("resources/sphere-smooth.obj").expect("unable to load obj file for sphere");

    // initialize objects list
    let mut objects: Vec<Object> = Vec::new();

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
    red.model.translate(Vector3::new(0.0, 0.0, -15.0));
    objects.push(red);

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
    green.model.translate(Vector3::new(65.0, 0.0, -15.0));
    objects.push(green);

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
    // another.model.scale(Vector3::new(0.4, 0.4, 0.4));
    blue.model.translate(Vector3::new(-65.0, 0.0, -15.0));
    objects.push(blue);





    let mut light = PointLight::new();
    light.position.z = 40.0;
    light.position.y = 25.0;
    light.strength = 2.0;

    unsafe {
        load_gl_with(|f_name| gl_win.get_proc_address(f_name.cast()));
        // initialize all of the objects buffers
        for object in &mut objects {
            object.init();
        }

        let blinn_shader = shader::Shader::new("pbr".to_string());
        shader_program = blinn_shader.program;

        glClearColor(0.2, 0.3, 0.3, 1.0);
    }

    // Define the camera parameters (position, target, up direction)
    let camera_position = Point3::new(0.0, 15.0, 30.0); // Example camera position
    let camera_target = Point3::new(0.0, 0.0, 0.0); // Target point to look at
    let up_direction: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0); // Up direction for the camera

    let rotation_speed = std::f32::consts::PI * 3.0;
    let start_time = Instant::now();

    'main_loop: loop {
        // handle events this frame
        while let Some((event, _timestamp)) = sdl.poll_events() {
            match event {
                Event::Quit => {
                    break 'main_loop;
                }
                Event::Key {
                    win_id,
                    pressed,
                    repeat,
                    scancode,
                    keycode,
                    modifiers,
                } => {
                    println!("{:?}", keycode);
                }
                _ => (),
            }
        }
        // now the events are clear.

        // here's where we could change the world state if we had some.
        for object in &mut objects {
            // Rotate the model
            object
                .model
                .rotate(Vector3::y_axis(), rotation_speed * 0.0005);

            // Calculate movement value
            let move_val = f32::sin(start_time.elapsed().as_secs_f32() * 5.5) * 0.5;

            // Translate the model
            object.model.translate(Vector3::new(
                object.model.position.x,
                object.model.position.y + move_val,
                object.model.position.z,
            ));
        }

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
            let size = 100.0;
            let projection: Matrix4<f32> =
                Matrix4::new_orthographic(-size, size, -size, size, 0.1, 10000.0);

            glUniformMatrix4fv(projection_loc, 1, GL_FALSE, projection.as_ptr());
            cam.link_shader(shader_program);
            light.link_shader(shader_program);

            // now loop over the objects and get specific uniform fields for the object
            for object in &mut objects {
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
