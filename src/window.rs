use beryllium::{
    events::Event,
    init::InitFlags,
    video::{CreateWinArgs, GlContextFlags, GlProfile, GlSwapInterval, GlWindow},
    *,
};

use core::{
    convert::{TryFrom, TryInto},
    mem::{size_of, size_of_val},
};
use ogl33::*;

use std::ptr::null;

pub struct GameWindow<'a> {
    name: &'a str,
    width: i32,
    height: i32,

    pub window: Option<GlWindow>,
    pub ctx: Option<Sdl>,
}

impl GameWindow<'_> {
    pub fn new(name: &str, width: i32, height: i32) -> GameWindow {
        GameWindow {
            name: name,
            width: width,
            height: height,
            window: None,
            ctx: None,
        }
    }

    pub fn init(&mut self) {
        let sdl = Sdl::init(InitFlags::EVERYTHING);
        sdl.set_gl_context_major_version(3).unwrap();
        sdl.set_gl_context_minor_version(3).unwrap();
        sdl.set_gl_profile(GlProfile::Core).unwrap();

        let mut flags = GlContextFlags::default();
        if cfg!(target_os = "macos") {
            flags |= GlContextFlags::FORWARD_COMPATIBLE;
        }
        if cfg!(debug_assertions) {
            flags |= GlContextFlags::DEBUG;
        }
        sdl.set_gl_context_flags(flags).unwrap();

        let gl_window = sdl
            .create_gl_window(CreateWinArgs {
                title: &self.name,
                width: self.width,
                height: self.height,
                ..Default::default()
            })
            .expect("Couldn't make a window and context");

        // Set the swap interval
        if let Some(window) = &self.window {
            window.set_swap_interval(GlSwapInterval::Vsync).unwrap();
        }

        // Assign the created window to the `window` field
        self.window = Some(gl_window);
        self.ctx = Some(sdl);
    }
}
