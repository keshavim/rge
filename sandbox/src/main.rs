use std::cell::{Ref, RefCell};
use std::default;
use std::rc::Rc;

use rgengine::application::Application;
use rgengine::imgui::ImGuiLayer;
use rgengine::layer::{Layer, SharedLayer};
use rgengine::{engine_run, rge_critical, rge_error, rge_info, rge_trace};

#[derive(Default)]
struct Example_layer {
    id: usize,
}
impl Example_layer {
    fn new() -> Self {
        Self { id: 1 }
    }
}

impl Layer for Example_layer {
    fn update(&mut self) {}
    fn event(&mut self, e: &dyn rgengine::events::Event) {
        rge_error!("{}", e.to_string());
    }
    fn id(&self) -> u32 {
        todo!()
    }
    fn attach(&mut self) {
        rge_trace!("attach");
    }
    fn detach(&mut self) {
        todo!()
    }
}

fn main() {
    let _ = rgengine::log::init();
    rgengine::rge_info!("init done");
    let mut app = Application::new();
    let shared_layer: SharedLayer = Box::new(Example_layer::new());
    app.push_layer(shared_layer);
    //let shared_layer: SharedLayer = Box::new(ImGuiLayer::new(app.window.get_native_window()));
    //app.push_layer(shared_layer);
    app.run();
}
