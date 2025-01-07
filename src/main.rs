#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::singl::e_match)]
#![allow(unused_imports)]
#![allow(clippy::zero_ptr)]

mod buffers;
mod camera;
mod collision;
mod directional_light;
mod material;
mod obj;
mod point_light;
mod raycast;
mod render;
mod scene;
mod scene_one;
mod shader;
mod texture;
mod vertex;
mod window;
mod particle;

use core::{
    convert::{TryFrom, TryInto},
    mem::{size_of, size_of_val},
};
use nalgebra::{Matrix4, Point3, Vector3};
use point_light::PointLight;
use scene::{Scene, Settings};

use gl::types::*;
use render::{Model, Object};
use sdl2::event::WindowEvent;
use std::{collections::HashMap, ffi::CString, time::Instant};

const WINDOW_TITLE: &str = "VS Clone";
const WINDOW_HEIGHT: i32 = 878;
const WINDOW_WIDTH: i32 = 1352;

fn main() {
    let mut gw = window::GameWindow::new(WINDOW_TITLE.to_string(), WINDOW_WIDTH, WINDOW_HEIGHT);
    gw.init();
    let gl_win = gw.window.expect("failed to get opengl:: window");
    let video_subsys = gw.video_subsystem.expect("unable to get video subsys");
    let sdl = gw.ctx.expect("failed to get window context");
    let active_scene: String = "scene-01".to_string();
    let mut scenes: HashMap<String, Scene> = HashMap::new();
    let mut scene_settings = Settings::new(WINDOW_WIDTH, WINDOW_HEIGHT, 60.0_f32.to_radians());

    let _gl_context = gl_win.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsys.gl_get_proc_address(s) as *const std::os::raw::c_void);

    scenes.insert("scene-01".to_string(), scene_one::scene_one(scene_settings));

    let mut shader_program: u32 = 0;
    unsafe {
        let blinn_shader = shader::Shader::new("pbr".to_string());
        shader_program = blinn_shader.program;

        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
    }

    let scene = scenes
        .get_mut(&active_scene)
        .expect("unable to get active-scene");

    scene.start();

    'main_loop: loop {
        let aspect: f32 = scene.settings.screen_width as f32 / scene.settings.screen_height as f32;
        let projection = Matrix4::new_perspective(aspect, scene.settings.fovy, 0.1, 10000.0);

        // handle events this frame
        let mut event_pump = sdl.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            (scene.on_event)(scene, event.clone());
            match event {
                sdl2::event::Event::Quit { .. } => break 'main_loop,
                sdl2::event::Event::Window {
                    timestamp,
                    window_id,
                    win_event,
                } => match win_event {
                    sdl2::event::WindowEvent::Resized(new_width, new_height) => {
                        scene.settings.screen_width = new_width;
                        scene.settings.screen_height = new_height;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        // now the events are clear, update our scene
        (scene.on_update)(scene);

        // and then draw!
        unsafe {
            // Clear the screen
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::MULTISAMPLE);
            gl::Viewport(
                0,
                0,
                scene.settings.screen_width,
                scene.settings.screen_height,
            );

            // Use the shader program
            gl::UseProgram(shader_program);

            // Get uniform locations for projection, view, and model matrices
            let projection_loc = shader::get_shader_location(shader_program, "projection");
            let model_loc = shader::get_shader_location(shader_program, "model");

            gl::UniformMatrix4fv(projection_loc, 1, gl::FALSE, projection.as_ptr());
            scene
                .cameras
                .get_mut(&scene.active_camera)
                .unwrap()
                .link_shader(shader_program);
            for (index, pl) in scene.point_lights.iter_mut().enumerate() {
                pl.link_shader(shader_program, index.try_into().unwrap());
            }

            // link the dir light if it is present
            if let Some(dir_light) = &scene.directional_light {
                dir_light.link_shader(shader_program);
            }

            gl::Uniform1i(
                shader::get_shader_location(shader_program, "numPointLights"),
                scene.point_lights.len().try_into().unwrap(),
            );

            gl::Uniform2f(
                shader::get_shader_location(shader_program, "resolution"),
                scene.settings.screen_width as f32,
                scene.settings.screen_height as f32,
            );

            // now loop over the objects and get specific uniform fields for the object
            for (_, object) in &mut scene.object_map.iter_mut() {
                let model = object.model.get_model_matrix();
                gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, model.as_ptr());

                // link the material here
                object.material.link_shader(shader_program);

                // Bind buffers and draw
                object.buffers.bind();
                gl::DrawArrays(gl::TRIANGLES, 0, object.buffers.size);
                object.buffers.unbind();
            }
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        gl_win.gl_swap_window();
    }
}
