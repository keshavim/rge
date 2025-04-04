#![allow(clippy::new_without_default)]

use std::{
    fmt,
    sync::{Arc, Mutex},
};

use crate::events::EventType;

use super::{
    events::Event,
    log::rge_engine_info,
    window::{Window, WindowProps, X11Window},
};
pub struct Application {
    window: Box<dyn Window>,
    running: bool,
}

impl Application {
    //need to beable se send the Application into the closure and mutate it with events
    pub fn new() -> Arc<Mutex<Self>> {
        let window: Box<dyn Window> = Box::new(X11Window::new(WindowProps::default()));

        let app = Arc::new(Mutex::new(Application {
            window,
            running: true,
        }));

        // Wrap the method in a closure
        let callback = Box::new(move |event: &dyn Event| {
            rge_engine_info!("{}", event.to_string());
            if event.get_type() == EventType::WindowClose {}
        });
        app.lock().unwrap().window.set_event_callback(callback);
        app
    }

    pub fn run(&mut self) {
        while !self.window.should_close() {
            self.window.update();
        }
    }
}
impl fmt::Debug for Application {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Application")
            .field("window", &"<dyn Window>")
            .field("running", &self.running)
            .finish()
    }
}
