use std::cell::RefCell;
use std::rc::Rc;

use crate::log::rge_engine_error;

use super::events::{
    Event, KeyPressedEvent, KeyReleasedEvent, MouseButtonPressedEvent, MouseButtonReleasedEvent,
    MouseMovedEvent, MouseScrolledEvent, WindowCloseEvent, WindowResizeEvent,
};
use glfw::{Action, Context, Glfw, GlfwReceiver, PWindow};
use glfw::{Error, WindowEvent, WindowHint};

pub struct WindowBuilder {
    width: u32,
    height: u32,
    title: String,
    vsync: Option<bool>,
}

impl WindowBuilder {
    pub fn vsync(&mut self, vsync: bool) -> &mut Self {
        self.vsync = Some(vsync);
        self
    }
    fn init_glfw(&self) -> (Glfw, PWindow, GlfwReceiver<(f64, WindowEvent)>) {
        let mut glfw = glfw::init(glfw::fail_on_errors).expect("Failed to init Glfw");

        // Set all window hints
        glfw.window_hint(WindowHint::ContextVersion(4, 5));
        glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(WindowHint::OpenGlForwardCompat(true));
        glfw.window_hint(WindowHint::DoubleBuffer(true));
        glfw.window_hint(WindowHint::Samples(Some(4))); // MSAA
        glfw.window_hint(WindowHint::OpenGlDebugContext(cfg!(debug_assertions))); // Set a custom error callback
        glfw.set_error_callback(|error: Error, description: String| {
            rge_engine_error!("GLFW Error {:?}: {}", error, description);
        });

        let (mut window, events) = glfw
            .create_window(
                self.width,
                self.height,
                &self.title,
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
        window.set_framebuffer_size_polling(true);

        gl::load_with(|s| window.get_proc_address(s) as *const _);

        (glfw, window, events)
    }
    pub fn build(self) -> Window {
        let (glfw, window, events) = self.init_glfw();

        Window {
            title: self.title,
            width: self.width,
            height: self.height,
            vsync: self.vsync.unwrap_or(true),
            event_callback: RefCell::new(None),
            glfw,
            window: Rc::new(RefCell::new(window)),
            events,
        }
    }
}

type EventCallback = RefCell<Option<Box<dyn FnMut(&dyn Event)>>>;
///window made specifically for xll
pub struct Window {
    title: String,
    width: u32,
    height: u32,
    vsync: bool,
    event_callback: EventCallback,
    glfw: glfw::Glfw,
    window: Rc<RefCell<glfw::PWindow>>,
    events: glfw::GlfwReceiver<(f64, WindowEvent)>,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> WindowBuilder {
        WindowBuilder {
            width,
            height,
            title: title.to_string(),
            vsync: None,
        }
    }
    pub fn update(&mut self) {
        unsafe {
            gl::ClearColor(0.2, 0.1, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        self.glfw.poll_events();
        self.handle_events();
        self.window.borrow_mut().swap_buffers();
    }
    //handles all events with the help of a closer method given
    fn handle_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            let mut callback = self.event_callback.borrow_mut();
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                    gl::Viewport(0, 0, width, height)
                },
                WindowEvent::Close => {
                    let e = WindowCloseEvent::new();
                    if let Some(c) = callback.as_mut() {
                        c(&e)
                    }
                }
                WindowEvent::Size(w, h) => {
                    let w = w as u32;
                    let h = h as u32;
                    self.width = w;
                    self.height = h;

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
    pub fn get_name(&self) -> &str {
        &self.title
    }
    pub fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
    pub fn set_vsync(&mut self, enabled: bool) {
        self.vsync = enabled;
    }
    pub fn is_vsync(&self) -> bool {
        self.vsync
    }
    pub fn get_native_window(&self) -> Rc<RefCell<glfw::PWindow>> {
        self.window.clone()
    }
    pub fn set_event_callback<F>(&self, callback: F)
    where
        F: FnMut(&dyn Event) + 'static,
    {
        *self.event_callback.borrow_mut() = Some(Box::new(callback));
    }
}
