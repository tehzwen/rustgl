#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::single_match)]
#![allow(unused_imports)]
#![allow(clippy::zero_ptr)]

mod buffers;
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

const WINDOW_TITLE: &str = "Triangle: Draw Arrays";
// const VERTICES: [vertex::Vertex; 3] = [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]];
const VERTICES: [vertex::Vertex; 36] = [
    // Front face
    [-0.5, -0.5, 0.5],
    [0.5, -0.5, 0.5],
    [0.5, 0.5, 0.5],
    [-0.5, -0.5, 0.5],
    [0.5, 0.5, 0.5],
    [-0.5, 0.5, 0.5],
    // Back face
    [-0.5, -0.5, -0.5],
    [-0.5, 0.5, -0.5],
    [0.5, 0.5, -0.5],
    [-0.5, -0.5, -0.5],
    [0.5, 0.5, -0.5],
    [0.5, -0.5, -0.5],
    // Left face
    [-0.5, -0.5, -0.5],
    [-0.5, -0.5, 0.5],
    [-0.5, 0.5, 0.5],
    [-0.5, -0.5, -0.5],
    [-0.5, 0.5, 0.5],
    [-0.5, 0.5, -0.5],
    // Right face
    [0.5, -0.5, -0.5],
    [0.5, 0.5, -0.5],
    [0.5, 0.5, 0.5],
    [0.5, -0.5, -0.5],
    [0.5, 0.5, 0.5],
    [0.5, -0.5, 0.5],
    // Top face
    [-0.5, 0.5, -0.5],
    [-0.5, 0.5, 0.5],
    [0.5, 0.5, 0.5],
    [-0.5, 0.5, -0.5],
    [0.5, 0.5, 0.5],
    [0.5, 0.5, -0.5],
    // Bottom face
    [-0.5, -0.5, -0.5],
    [0.5, -0.5, -0.5],
    [0.5, -0.5, 0.5],
    [-0.5, -0.5, -0.5],
    [0.5, -0.5, 0.5],
    [-0.5, -0.5, 0.5],
];

const NORMALS: [vertex::Vertex; 36] = [
    // Front face (0, 0, 1)
    [0.0, 0.0, 1.0],
    [0.0, 0.0, 1.0],
    [0.0, 0.0, 1.0],
    [0.0, 0.0, 1.0],
    [0.0, 0.0, 1.0],
    [0.0, 0.0, 1.0],
    // Back face (0, 0, -1)
    [0.0, 0.0, -1.0],
    [0.0, 0.0, -1.0],
    [0.0, 0.0, -1.0],
    [0.0, 0.0, -1.0],
    [0.0, 0.0, -1.0],
    [0.0, 0.0, -1.0],
    // Left face (-1, 0, 0)
    [-1.0, 0.0, 0.0],
    [-1.0, 0.0, 0.0],
    [-1.0, 0.0, 0.0],
    [-1.0, 0.0, 0.0],
    [-1.0, 0.0, 0.0],
    [-1.0, 0.0, 0.0],
    // Right face (1, 0, 0)
    [1.0, 0.0, 0.0],
    [1.0, 0.0, 0.0],
    [1.0, 0.0, 0.0],
    [1.0, 0.0, 0.0],
    [1.0, 0.0, 0.0],
    [1.0, 0.0, 0.0],
    // Top face (0, 1, 0)
    [0.0, 1.0, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, 1.0, 0.0],
    // Bottom face (0, -1, 0)
    [0.0, -1.0, 0.0],
    [0.0, -1.0, 0.0],
    [0.0, -1.0, 0.0],
    [0.0, -1.0, 0.0],
    [0.0, -1.0, 0.0],
    [0.0, -1.0, 0.0],
];

fn main() {
    let mut gw = window::GameWindow::new(WINDOW_TITLE.to_string(), 800, 600);
    gw.init();
    let gl_win = gw.window.expect("failed to get opengl window");
    let sdl = gw.ctx.expect("failed to get window context");
    let render_buffers = buffers::RenderBuffers::new();
    let mut shader_program: u32 = 0;

    let mut triangle = Object::new(Model::new(), render_buffers, &VERTICES);
    let mut light = PointLight::new();
    light.position.z = 3.0;
    light.position.y = 3.0;
    light.strength = 3.0;

    unsafe {
        load_gl_with(|f_name| gl_win.get_proc_address(f_name.cast()));
        triangle.buffers.init(&VERTICES, &NORMALS);
        let blinn_shader = shader::Shader::new("blinn".to_string());
        shader_program = blinn_shader.program;

        glClearColor(0.2, 0.3, 0.3, 1.0);
    }

    // Define the camera parameters (position, target, up direction)
    let camera_position = Point3::new(0.0, 5.0, 3.0); // Example camera position
    let camera_target = Point3::new(0.0, 0.0, 0.0); // Target point to look at
    let up_direction: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0); // Up direction for the camera

    let rotation_speed = std::f32::consts::PI / 2.0; // 90 degrees per second
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
        triangle
            .model
            .rotate(Vector3::y_axis(), rotation_speed * 0.0005);

        // get the sin of the values to move up & down
        let move_val = f32::sin(start_time.elapsed().as_secs_f32() * 4.5) * 0.005;

        triangle.model.translate(Vector3::new(
            triangle.model.position.x,
            triangle.model.position.y + move_val,
            triangle.model.position.z,
        ));

        // and then draw!
        unsafe {
            // Clear the screen
            glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
            glEnable(GL_DEPTH_TEST);

            // Use the shader program
            glUseProgram(shader_program);

            // Get uniform locations for projection, view, and model matrices
            let projection_loc = shader::get_shader_location(shader_program, "projection");
            let view_loc = shader::get_shader_location(shader_program, "view");
            let model_loc = shader::get_shader_location(shader_program, "model");

            let light_position_loc = shader::get_shader_location(shader_program, "light.position");
            let light_color_loc = shader::get_shader_location(shader_program, "light.color");
            let light_strength_loc = shader::get_shader_location(shader_program, "light.strength");

            // Create the orthogonal projection matrix
            let projection: Matrix4<f32> =
                Matrix4::new_orthographic(-1.0, 1.0, -1.0, 1.0, 0.1, 100.0);

            // Create the view matrix using lookAt (right-handed coordinate system)
            let view = Matrix4::look_at_rh(&camera_position, &camera_target, &up_direction);

            // Create the model matrix (identity if no transformation)
            // let model: Matrix4<f32> = Matrix4::identity();
            let model = triangle.model.get_model_matrix();

            // Send the matrices to the shader
            glUniformMatrix4fv(projection_loc, 1, GL_FALSE, projection.as_ptr());
            glUniformMatrix4fv(view_loc, 1, GL_FALSE, view.as_ptr());
            glUniformMatrix4fv(model_loc, 1, GL_FALSE, model.as_ptr());

            // pointlight
            glUniform3f(
                light_position_loc,
                light.position.x,
                light.position.y,
                light.position.z,
            );
            glUniform3f(light_color_loc, light.color.x, light.color.y, light.color.z); // Light color (white)
            glUniform1f(light_strength_loc, light.strength); // Light strength

            // Bind buffers and draw
            triangle.buffers.bind();
            glDrawArrays(GL_TRIANGLES, 0, triangle.buffers.size);
            triangle.buffers.unbind();
        }
        gl_win.swap_window();
    }
}
