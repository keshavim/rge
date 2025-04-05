#![allow(clippy::new_without_default)]

use std::{
    fmt,
    ops::Deref,
    sync::{Arc, Mutex, OnceLock},
};

use crate::rge_trace;

use super::{
    events::{Event, EventDispatcher, EventType, WindowCloseEvent},
    log::{rge_engine_info, rge_engine_trace},
    window::{Window, WindowProps, X11Window},
};
pub struct Application {
    window: Arc<Mutex<X11Window>>,
    dispatcher: Arc<EventDispatcher>,
    running: Arc<Mutex<bool>>,
}

impl Application {
    //need to beable se send the Application into the closure and mutate it with events
    pub fn new() -> Self {
        let window = Arc::new(Mutex::new(X11Window::new(WindowProps::default())));
        let dispatcher = Arc::new(EventDispatcher::new());

        // Clone Arcs for callback
        let dispatcher_clone = Arc::clone(&dispatcher);
        let window_clone = Arc::clone(&window);

        window_clone
            .lock()
            .unwrap()
            .set_event_callback(move |event| {
                rge_engine_trace!("{}", event.to_string());
                dispatcher_clone.dispatch(event);
            });
        let running = Arc::new(Mutex::new(true));
        Application {
            window,
            dispatcher,
            running,
        }
    }

    pub fn run(&mut self) {
        //temp stuff
        let running_clone = Arc::clone(&self.running);
        self.dispatcher
            .register(EventType::WindowClose, move |_: &WindowCloseEvent| {
                rge_engine_info!("should close");
                *running_clone.lock().unwrap() = false;
            });

        while *self.running.lock().unwrap().deref() {
            self.window.lock().unwrap().update();
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
