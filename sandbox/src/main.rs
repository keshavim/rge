use rgengine::application::Application;
use rgengine::layer::Layer;
use rgengine::{engine_run, rge_critical, rge_error, rge_info};

#[derive(Default)]
struct Example_layer {
    id: usize,
}

impl Layer for Example_layer {
    fn on_update(&self) {
        rge_info!("example layer: update");
    }
    fn on_event(&self, e: &dyn rgengine::events::Event) {
        rge_error!("{}", e.to_string());
    }
    fn id(&self) -> u32 {
        todo!()
    }
    fn on_attach(&self) {
        todo!()
    }
    fn on_detach(&self) {
        todo!()
    }
}

engine_run!();
