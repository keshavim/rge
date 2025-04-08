use super::{
    events::{Event, EventSystem, EventType},
    imgui::ImGuiLayer,
    layers::LayerStack,
    renderer::Renderer,
    window::WindowManager,
};

// engine/mod.rs
pub struct GameEngine {
    window: WindowManager,
    renderer: Renderer,
    event_system: EventSystem,
    layer_stack: LayerStack,
    is_running: bool,
}

impl GameEngine {
    pub fn new() -> Self {
        let mut window = WindowManager::new("Game Engine", 1280, 720);
        let renderer = Renderer::new(&mut window);
        let event_system = EventSystem::new();
        let mut layer_stack = LayerStack::new();

        layer_stack.push_overlay(Box::new(ImGuiLayer::new(&mut window, 1)));
        Self {
            window,
            renderer,
            event_system,
            layer_stack,
            is_running: true,
        }
    }

    pub fn run(&mut self) {
        while self.is_running {
            //events
            self.window.poll_events(|event| {
                //this is sooo shit
                let mut e = Event::from(&event);
                EventSystem::dispatch(&mut e, |e: Event| {
                    let et = EventType::WindowClose {};
                    if et == e.event_type {
                        self.is_running = false;
                        return true;
                    }
                    false
                });
                EventSystem::dispatch(&mut e, |e: Event| {
                    if let EventType::KeyPressed {
                        key: glfw::Key::Escape,
                        ..
                    } = e.event_type
                    {
                        self.is_running = false;
                        println!("Escape pressed!");
                        return true;
                    }
                    false
                });

                self.layer_stack.on_event(event);
            });

            //rendering
            self.layer_stack.update();
            self.renderer.render_frame(&mut self.window);
            self.layer_stack.render(&mut self.window);

            self.window.swap_buffers();
        }
    }
}
