#![allow(clippy::new_without_default)]

use core::panic;
use std::{
    fmt,
    ops::Deref,
    rc::Rc,
    sync::{Arc, Mutex},
};

use super::{
    events::{EventDispatcher, EventType, WindowCloseEvent},
    layer::{Layer, LayerStack},
    log::{rge_engine_info, rge_engine_trace},
    window::Window,
};

pub struct Application {
    window: Rc<Mutex<Window>>,
    dispatcher: Arc<EventDispatcher>,
    running: Arc<Mutex<bool>>,
    layer_stack: Arc<Mutex<LayerStack>>,
}

impl Application {
    //need to beable se send the Application into the closure and mutate it with events
    pub fn new() -> Self {
        let window = Rc::new(Mutex::new(Window::new("title", 800, 600).build()));
        let dispatcher = Arc::new(EventDispatcher::new());

        let running = Arc::new(Mutex::new(true));
        let layer_stack = Arc::new(Mutex::new(LayerStack::new()));
        Application {
            window,
            dispatcher,
            running,
            layer_stack,
        }
    }

    pub fn run(&mut self) {
        //may move this some where else
        // Clone Arcs for callback
        let running_clone = Arc::clone(&self.running);
        self.dispatcher
            .register(EventType::WindowClose, move |_: &WindowCloseEvent| {
                rge_engine_info!("should close");
                *running_clone.lock().unwrap() = false;
            });

        let dispatcher_clone = Arc::clone(&self.dispatcher);
        let window_clone = Rc::clone(&self.window);
        let layer_clone = Arc::clone(&self.layer_stack);

        window_clone
            .lock()
            .unwrap()
            .set_event_callback(move |event| {
                //handles speciel events
                dispatcher_clone.dispatch(event);

                //handles events starting from the top layer
                layer_clone
                    .lock()
                    .unwrap_or_else(|e| panic!("layer stack access failed {}", e))
                    .iter()
                    .rev()
                    .take_while(|layer| {
                        layer.on_event(event);
                        !event.is_handled()
                    })
                    .for_each(drop);
            });

        //run loop
        while self.is_running() {
            self.update_layers();
            self.update_window();
        }
    }

    pub fn push_layer(&mut self, layer: Box<dyn Layer>) {
        self.layer_stack
            .lock()
            .expect("layer_stack poisoned")
            .push_layer(layer);
    }
    pub fn push_overlay(&mut self, overlay: Box<dyn Layer>) {
        self.layer_stack
            .lock()
            .expect("layer_stack poisoned")
            .push_overlay(overlay);
    }

    fn is_running(&self) -> bool {
        *self
            .running
            .lock()
            .unwrap_or_else(|e| panic!("runing state mutex poisoned {}", e))
            .deref()
    }
    fn update_window(&self) {
        self.window
            .lock()
            .unwrap_or_else(|e| panic!("window mutex poisoned {}", e))
            .update();
    }
    fn update_layers(&self) {
        self.layer_stack
            .lock()
            .unwrap_or_else(|e| panic!("Layer stack mutex poisoned: {}", e))
            .iter()
            .for_each(|l| l.on_update());
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
