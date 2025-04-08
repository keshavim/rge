use super::window::WindowManager;

// may need to work on this trait
pub trait Layer {
    fn on_attach(&mut self) {}
    fn on_detach(&mut self) {}
    fn on_update(&mut self, _dt: f32) {}
    fn on_event(&mut self, _event: &glfw::WindowEvent) {}
    fn on_render(&mut self, _window: &mut WindowManager) {}
    fn get_id(&self) -> usize;
}

//may need to make the layers shared but rc refcell is a pain

///vec contains both layers and overlays which are seperated by the insert pos
pub struct LayerStack {
    layers: Vec<Box<dyn Layer>>,
    insert_pos: usize,
}

impl LayerStack {
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            insert_pos: 0,
        }
    }

    pub fn push_layer(&mut self, layer: Box<dyn Layer>) {
        self.layers.insert(self.insert_pos, layer);
        self.insert_pos += 1;
    }

    pub fn push_overlay(&mut self, overlay: Box<dyn Layer>) {
        self.layers.push(overlay);
    }

    pub fn pop_layer(&mut self, target: Box<dyn Layer>) -> Option<Box<dyn Layer>> {
        let end = self.insert_pos;
        if let Some(pos) = self.layers[..end]
            .iter()
            .position(|x| x.get_id() == target.get_id())
        {
            let mut old = self.layers.remove(pos);
            self.insert_pos -= 1;
            old.on_detach();
            return Some(old);
        }
        None
    }

    pub fn pop_overlay(&mut self, target: Box<dyn Layer>) -> Option<Box<dyn Layer>> {
        let start = self.insert_pos - 1;
        if let Some(pos) = self.layers[start..]
            .iter()
            .position(|x| x.get_id() == target.get_id())
            .map(|x| x + start)
        {
            let mut old = self.layers.remove(pos);
            old.on_detach();
            return Some(old);
        }
        None
    }

    //these are temorary, maybe
    pub fn update(&mut self) {
        for layer in self.layers.iter_mut().rev() {
            layer.on_update(0.016); // 60 FPS delta
        }
    }

    pub fn render(&mut self, window: &mut WindowManager) {
        for layer in &mut self.layers {
            layer.on_render(window);
        }
    }

    pub fn on_event(&mut self, event: glfw::WindowEvent) {
        for layer in self.layers.iter_mut().rev() {
            layer.on_event(&event);
        }
    }
}
