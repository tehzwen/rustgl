use std::ffi::CString;
use std::fs::File;
use std::io::{self, Read};

use gl::types::*;

pub fn get_shader_location(program: u32, name: &str) -> i32 {
    let uniform_name = CString::new(name).unwrap();

    unsafe {
        return gl::GetUniformLocation(program, uniform_name.as_ptr());
    }
}

fn read_shader_to_string(file_path: String) -> io::Result<String> {
    // Open the file
    let mut file = File::open(file_path)?;

    // Create a string to hold the contents of the file
    let mut contents = String::new();

    // Read the file contents into the string
    file.read_to_string(&mut contents)?;

    // Return the contents of the file
    Ok(contents)
}

pub struct Shader {
    pub program: u32,
}

impl Shader {
    pub fn new(shader_name: String) -> Shader {
        // try and open the .vert file first
        let vert_src = read_shader_to_string(format!("shaders/{}.vert.glsl", shader_name))
            .expect("failed to read vert source");
        // now the frag file
        let frag_src = read_shader_to_string(format!("shaders/{}.frag.glsl", shader_name))
            .expect("failed to load frag source");

        // now we compile it
        let mut shader_program = 0;
        unsafe {
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            assert_ne!(vertex_shader, 0);
            gl::ShaderSource(
                vertex_shader,
                1,
                &vert_src.as_bytes().as_ptr().cast(),
                &vert_src.len().try_into().unwrap(),
            );
            gl::CompileShader(vertex_shader);
            let mut success = 0;
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetShaderInfoLog(vertex_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                panic!("Vertex Compile Error: {}", String::from_utf8_lossy(&v));
            }

            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            assert_ne!(fragment_shader, 0);
            gl::ShaderSource(
                fragment_shader,
                1,
                &frag_src.as_bytes().as_ptr().cast(),
                &frag_src.len().try_into().unwrap(),
            );
            gl::CompileShader(fragment_shader);
            let mut success = 0;
            gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetShaderInfoLog(fragment_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                panic!("Fragment Compile Error: {}", String::from_utf8_lossy(&v));
            }

            shader_program = gl::CreateProgram();
            assert_ne!(shader_program, 0);
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);
            let mut success = 0;
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetProgramInfoLog(shader_program, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
            }
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
        }

        Shader {
            program: shader_program,
        }
    }
}
