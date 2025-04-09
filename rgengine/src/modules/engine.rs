use crate::events::*;

use super::{imgui::ImGuiLayer, layers::LayerStack, renderer::Renderer, window::WindowManager};

// engine/mod.rs
pub struct GameEngine {
    window: WindowManager,
    renderer: Renderer,
    layer_stack: LayerStack,
    is_running: bool,
}

impl GameEngine {
    pub fn new() -> Self {
        let mut window = WindowManager::new("Game Engine", 1280, 720);
        let renderer = Renderer::new(&mut window);
        let mut layer_stack = LayerStack::new();

        layer_stack.push_overlay(Box::new(ImGuiLayer::new(&mut window, 1)));

        Self {
            window,
            renderer,
            layer_stack,
            is_running: true,
        }
    }

    pub fn run(&mut self) {
        while self.is_running {
            //events
            let event = self.window.handle_events();

            if let Some(e) = event {
                if e.get_type() == EventType::WindowClose {
                    self.is_running = false;
                }
                self.layer_stack.on_event(&e);
            }

            //rendering
            self.layer_stack.update();
            self.renderer.render_frame(&mut self.window);
            self.layer_stack.render(&mut self.window);

            self.window.swap_buffers();
        }
    }
}
