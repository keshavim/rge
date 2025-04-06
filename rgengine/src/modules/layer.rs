use super::events::Event;

pub trait Layer: Sync + Send {
    fn on_attach(&self);
    fn on_detach(&self);
    fn on_update(&self);
    fn on_event(&self, e: &dyn Event);
    fn id(&self) -> u32;
}

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

    pub fn pop_layer(&mut self, layer: Box<dyn Layer>) {
        let target_id = layer.id();
        let original_len = self.layers.len();

        self.layers.retain(|l| l.id() != target_id);

        // Adjust insert_pos if we actually removed an element
        if self.layers.len() < original_len && self.insert_pos > 0 {
            self.insert_pos -= 1;
        }
    }

    pub fn popover_layer(&mut self, overlay: Box<dyn Layer>) {
        let target_id = overlay.id();
        self.layers.retain(|l| l.id() != target_id);
    }
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &Box<dyn Layer>> + '_ {
        self.layers.iter()
    }
}
