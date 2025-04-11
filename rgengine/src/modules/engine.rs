use std::{cell::RefCell, rc::Rc};

use crate::events::*;

use super::{
    imgui::ImGuiLayer,
    input::{self, InputHandler},
    layers::LayerStack,
    log::rge_engine_info,
    renderer::Renderer,
    window::WindowManager,
};

// engine/mod.rs
pub struct GameEngine {
    window: Rc<RefCell<WindowManager>>,
    input: InputHandler,
    renderer: Renderer,
    layer_stack: LayerStack,
    is_running: bool,
}

impl GameEngine {
    pub fn new() -> Self {
        let mut window = Rc::new(RefCell::new(WindowManager::new("Game Engine", 1280, 720)));
        let renderer = Renderer::new(&window);
        let mut layer_stack = LayerStack::new();

        layer_stack.push_overlay(Box::new(ImGuiLayer::new(&window, 1)));

        let input = InputHandler::new(&window);
        Self {
            window,
            renderer,
            layer_stack,
            is_running: true,
            input,
        }
    }

    pub fn run(&mut self) {
        while self.is_running {
            //events
            let event = self.window.borrow_mut().handle_events();

            if let Some(e) = event {
                if e.get_type() == EventType::WindowClose {
                    self.is_running = false;
                }
                self.layer_stack.on_event(&e);
            }

            let (x, y) = self.input.get_mouse_pos();
            //rge_engine_info!("mouse {x} {y}");

            //rendering
            self.layer_stack.update();

            self.renderer.render_frame();
            self.layer_stack.render();

            self.renderer.swap_buffers();
        }
    }

    pub fn get_window(&self) -> &Rc<RefCell<WindowManager>> {
        &self.window
    }
}
