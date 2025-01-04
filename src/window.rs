use sdl2::{
    sys::Window,
    video::{GLContext, GLProfile},
};
use sdl2::{Sdl, VideoSubsystem};

pub struct GameWindow {
    name: String,
    width: i32,
    height: i32,

    pub window: Option<sdl2::video::Window>,
    pub ctx: Option<Sdl>,
    pub video_subsystem: Option<VideoSubsystem>,
}

impl GameWindow {
    pub fn new(name: String, width: i32, height: i32) -> GameWindow {
        GameWindow {
            name,
            width,
            height,
            window: None,
            ctx: None,
            video_subsystem: None,
        }
    }

    pub fn init(&mut self) {
        // Initialize SDL context
        let sdl_context = sdl2::init().expect("unable to load sdl context");
        let video_subsystem = sdl_context.video().expect("unable to get video subsystem");

        // Set SDL GL attributes before creating window
        let gl_attr = video_subsystem.gl_attr();

        // Request an OpenGL 3.3 Core profile context
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        // Set color depth to 32 bits (RGBA)
        gl_attr.set_red_size(8);
        gl_attr.set_green_size(8);
        gl_attr.set_blue_size(8);
        gl_attr.set_alpha_size(8);
        gl_attr.set_multisample_samples(16);

        // Create an OpenGL window with the specified attributes
        let window = video_subsystem
            .window(&self.name, self.width as u32, self.height as u32)
            .opengl()
            .resizable()
            .build()
            .map_err(|e| e.to_string())
            .expect("unable to initialize window");

        // Create OpenGL context for the window
        let _gl_context = window.gl_create_context().unwrap();
        let _gl = gl::load_with(|s| {
            video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
        });
        // Set the window and context
        self.window = Some(window);

        self.ctx = Some(sdl_context);
        self.video_subsystem = Some(video_subsystem);
    }
}
