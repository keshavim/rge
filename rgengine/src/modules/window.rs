use super::events::Event;
use glfw::WindowEvent;
use glfw::{Action, Context, Key};

extern crate glfw;

#[derive(Debug)]
pub struct WindowProps {
    title: String,
    width: u32,
    height: u32,
}

impl WindowProps {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Self {
            title: title.to_string(),
            width,
            height,
        }
    }
}

impl Default for WindowProps {
    fn default() -> Self {
        Self {
            title: "RGengine".to_string(),
            width: 800,
            height: 600,
        }
    }
}

pub trait Window {
    fn update(&mut self);
    fn get_name(&self) -> &str;
    fn get_size(&self) -> (u32, u32);
    fn set_vsync(&mut self, enabled: bool);
    fn is_vsync(&self) -> bool;
    fn should_close(&self) -> bool;
}

pub struct X11Window {
    data: WindowProps,
    vsync: bool,
    glfw: glfw::Glfw,
    window: glfw::PWindow,
    events: glfw::GlfwReceiver<(f64, WindowEvent)>,
}

impl X11Window {
    pub fn new(props: WindowProps) -> Self {
        let data = props;
        let vsync = false;

        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
        let (mut window, events) = glfw
            .create_window(
                data.width,
                data.height,
                &data.title,
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window.");

        window.make_current();
        window.set_key_polling(true);

        window.set_key_callback(key_callback);

        Self {
            data,
            vsync,
            glfw,
            window,
            events,
        }
    }
    fn set_event_callback<F>(&self, callback: F)
    where
        F: FnOnce(Box<dyn Event>),
    {
        todo!()
    }
}
///will change how call backs work
// Key callback function
fn key_callback(
    window: &mut glfw::Window,
    key: Key,
    scancode: i32,
    action: Action,
    mods: glfw::Modifiers,
) {
    match action {
        Action::Press => println!("Key {:?} pressed with scancode {}", key, scancode),
        Action::Release => println!("Key {:?} released", key),
        Action::Repeat => println!("Key {:?} repeated", key),
    }

    // Close the window when Escape is pressed
    if key == Key::Escape && action == Action::Press {
        window.set_should_close(true);
    }
}

impl Window for X11Window {
    fn update(&mut self) {
        self.glfw.poll_events();
        // Handle events
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                WindowEvent::Close => self.window.set_should_close(true),
                _ => {}
            }
        }
        self.window.swap_buffers();
    }
    fn get_name(&self) -> &str {
        &self.data.title
    }
    fn get_size(&self) -> (u32, u32) {
        (self.data.width, self.data.height)
    }
    fn set_vsync(&mut self, enabled: bool) {
        self.vsync = enabled;
    }
    fn is_vsync(&self) -> bool {
        self.vsync
    }
    fn should_close(&self) -> bool {
        self.window.should_close()
    }
}
