use std::sync::Mutex;

use crate::log::rge_engine_error;

use super::events::{
    Event, KeyPressedEvent, KeyReleasedEvent, MouseButtonPressedEvent, MouseButtonReleasedEvent,
    MouseMovedEvent, MouseScrolledEvent, WindowCloseEvent, WindowResizeEvent,
};
use glfw::{Action, Context};
use glfw::{Error, WindowEvent};

#[derive(Debug)]
pub struct WindowProps {
    title: String,
    width: u32,
    height: u32,
}

///Basic Proporties the client can set
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

///internal window data
struct WindowData {
    title: String,
    width: u32,
    height: u32,
    vsync: bool,
    event_callback: Mutex<Option<Box<dyn FnMut(&dyn Event) + Send>>>,
}

///window made specifically for xll
pub struct X11Window {
    data: WindowData,
    glfw: glfw::Glfw,
    window: glfw::PWindow,
    events: glfw::GlfwReceiver<(f64, WindowEvent)>,
}

impl X11Window {
    pub fn new(props: WindowProps) -> Self {
        let data = WindowData {
            title: props.title,
            width: props.width,
            height: props.height,
            vsync: true,
            event_callback: Mutex::new(None),
        };

        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        // Set a custom error callback
        glfw.set_error_callback(|error: Error, description: String| {
            rge_engine_error!("GLFW Error {:?}: {}", error, description);
        });

        let (mut window, events) = glfw
            .create_window(
                data.width,
                data.height,
                &data.title,
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window.");

        window.make_current();

        window.set_size_polling(true);
        window.set_key_polling(true);
        window.set_mouse_button_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_close_polling(true);
        window.set_scroll_polling(true);

        Self {
            data,
            glfw,
            window,
            events,
        }
    }
}
///Contains all basic functionality for all types of windows
pub trait Window {
    fn update(&mut self);
    fn handle_events(&mut self);
    fn get_name(&self) -> &str;
    fn get_size(&self) -> (u32, u32);
    fn set_vsync(&mut self, enabled: bool);
    fn is_vsync(&self) -> bool;
    fn should_close(&self) -> bool;
    fn set_event_callback<F>(&self, callback: F)
    where
        F: FnMut(&dyn Event) + Send + 'static;
    fn get_native_window(&self) -> &glfw::PWindow;
}
impl Window for X11Window {
    fn update(&mut self) {
        self.glfw.poll_events();
        self.handle_events();
        self.window.swap_buffers();
    }
    //handles all events with the help of a closer method given
    fn handle_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            let mut callback = self.data.event_callback.lock().unwrap();
            match event {
                WindowEvent::Close => {
                    let e = WindowCloseEvent::new();
                    if let Some(c) = callback.as_mut() {
                        c(&e)
                    }
                    self.window.set_should_close(true);
                }
                WindowEvent::Size(w, h) => {
                    let w = w as u32;
                    let h = h as u32;
                    self.data.width = w;
                    self.data.height = h;

                    let e = WindowResizeEvent::new(w, h);
                    if let Some(c) = callback.as_mut() {
                        c(&e)
                    }
                }
                WindowEvent::Key(key, _scode, action, _mods) => match action {
                    Action::Press => {
                        let e = KeyPressedEvent::new(key, false);

                        if let Some(c) = callback.as_mut() {
                            c(&e)
                        }
                    }

                    Action::Release => {
                        let e = KeyReleasedEvent::new(key);

                        if let Some(c) = callback.as_mut() {
                            c(&e)
                        }
                    }
                    Action::Repeat => {
                        let e = KeyPressedEvent::new(key, true);

                        if let Some(c) = callback.as_mut() {
                            c(&e)
                        }
                    }
                },
                WindowEvent::MouseButton(button, action, _mods) => match action {
                    Action::Press => {
                        let e = MouseButtonPressedEvent::new(button);

                        if let Some(c) = callback.as_mut() {
                            c(&e)
                        }
                    }

                    Action::Release => {
                        let e = MouseButtonReleasedEvent::new(button);

                        if let Some(c) = callback.as_mut() {
                            c(&e)
                        }
                    }
                    _ => {}
                },
                WindowEvent::Scroll(xoff, yoff) => {
                    let e = MouseScrolledEvent::new(xoff, yoff);

                    if let Some(c) = callback.as_mut() {
                        c(&e)
                    }
                }
                WindowEvent::CursorPos(xpos, ypos) => {
                    let e = MouseMovedEvent::new(xpos, ypos);

                    if let Some(c) = callback.as_mut() {
                        c(&e)
                    }
                }
                _ => {}
            }
        }
    }
    fn get_name(&self) -> &str {
        &self.data.title
    }
    fn get_size(&self) -> (u32, u32) {
        (self.data.width, self.data.height)
    }
    fn set_vsync(&mut self, enabled: bool) {
        self.data.vsync = enabled;
    }
    fn is_vsync(&self) -> bool {
        self.data.vsync
    }
    fn should_close(&self) -> bool {
        self.window.should_close()
    }
    fn get_native_window(&self) -> &glfw::PWindow {
        &self.window
    }
    fn set_event_callback<F>(&self, callback: F)
    where
        F: FnMut(&dyn Event) + Send + 'static,
    {
        *self.data.event_callback.lock().unwrap() = Some(Box::new(callback));
    }
}
