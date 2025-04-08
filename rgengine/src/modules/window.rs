use std::sync::mpsc::Receiver;

use glfw::{Callback, Context, Error, WindowHint};

use crate::log::rge_engine_error;

//todo need to make all the public stuff private and acessed form methods
pub struct WindowData {
    title: String,
    pub width: u32,
    pub height: u32,
    vsync: bool,
}

// window/mod.rs
pub struct WindowManager {
    pub data: WindowData,
    glfw: glfw::Glfw,
    pub window: glfw::Window,
    events: Receiver<(f64, glfw::WindowEvent)>,
}

impl WindowManager {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        // Set all window hints
        glfw.window_hint(WindowHint::ContextVersion(4, 5));
        glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(WindowHint::OpenGlForwardCompat(true));
        glfw.window_hint(WindowHint::DoubleBuffer(true));
        glfw.window_hint(WindowHint::Samples(Some(4))); // MSAA
        glfw.window_hint(WindowHint::OpenGlDebugContext(cfg!(debug_assertions)));

        // Set a custom error callback
        // In your initialization code
        glfw.set_error_callback(Some(Callback {
            f: glfw_error_callback,
            data: (),
        }));
        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");

        window.make_current();
        window.set_all_polling(true);
        glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

        let data = WindowData {
            title: title.to_string(),
            width,
            height,
            vsync: true,
        };
        Self {
            data,
            glfw,
            window,
            events,
        }
    }

    pub fn poll_events<F: FnMut(glfw::WindowEvent)>(&mut self, mut callback: F) {
        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
            callback(event);
        }
    }

    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }

    pub fn get_glfw(&self) -> &glfw::Glfw {
        &self.glfw
    }

    pub fn native_window(&mut self) -> &mut glfw::Window {
        &mut self.window
    }
    pub fn is_vsync(&self) -> bool {
        self.data.vsync
    }
    pub fn set_vsync(&mut self, b: bool) {
        self.data.vsync = b;
    }
}

fn glfw_error_callback<'a>(error: Error, description: String, _context: &'a ()) {
    rge_engine_error!("GLFW Error {:?}: {}", error, description);
}
