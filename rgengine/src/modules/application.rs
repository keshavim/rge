#![allow(clippy::new_without_default)]

use std::{
    cell::{OnceCell, RefCell},
    rc::Rc,
};

use super::{
    events::{EventDispatcher, EventType, WindowCloseEvent},
    layer::{Layer, LayerStack, SharedLayer},
    window::Window,
};

pub struct Application {
    pub window: Window,
    dispatcher: Rc<EventDispatcher>,
    running: Rc<RefCell<bool>>,
    layer_stack: Rc<RefCell<LayerStack>>,
}

impl Application {
    //need to beable se send the Application into the closure and mutate it with events
    pub fn new() -> Application {
        let window = Window::new("title", 800, 600).build();
        let dispatcher = Rc::new(EventDispatcher::new());

        let running = Rc::new(RefCell::new(true));
        let layer_stack = Rc::new(RefCell::new(LayerStack::new()));
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
        let running_clone = Rc::clone(&self.running);
        self.dispatcher
            .register(EventType::WindowClose, move |_: &WindowCloseEvent| {
                //rge_engine_info!("should close");

                *running_clone.borrow_mut() = false;
            });

        let dispatcher_clone = Rc::clone(&self.dispatcher);
        let layer_clone = Rc::clone(&self.layer_stack);

        self.window.set_event_callback(move |event| {
            //handles speciel events
            dispatcher_clone.dispatch(event);

            //handles events starting from the top layer

            for layer in layer_clone.borrow_mut().iter_mut() {
                layer.event(event);
                if event.is_handled() {
                    break;
                }
            }
        });

        //run loop
        while *self.running.borrow() {
            self.update_layers();
            self.window.update();
        }
    }

    pub fn push_layer(&mut self, layer: SharedLayer) {
        self.layer_stack.borrow_mut().push_layer(layer);
    }
    pub fn push_overlay(&mut self, overlay: SharedLayer) {
        self.layer_stack.borrow_mut().push_overlay(overlay);
    }

    fn update_layers(&self) {
        for layer in self.layer_stack.borrow_mut().iter_mut() {
            layer.update();
        }
    }
}
