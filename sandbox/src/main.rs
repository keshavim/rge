use rgengine::{
    engine::GameEngine,
    layers::{Layer, LayerStack},
    rge_info, rge_warn,
};

#[derive(Debug, Default)]
struct ExampleLayer {
    pub id: usize,
}

impl Layer for ExampleLayer {
    fn get_id(&self) -> usize {
        self.id
    }
    fn on_update(&mut self, dt: f32) {
        rge_info!("{:?}", self.id);
    }
}

fn main() {
    let _ = rgengine::log::init();

    let mut engine = GameEngine::new();
    //
    engine.run();
}
