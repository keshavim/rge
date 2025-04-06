use super::events::Event;

use std::cell::RefCell;
use std::rc::Rc;

pub trait Layer {
    fn attach(&mut self);
    fn detach(&mut self);
    fn update(&mut self);
    fn event(&mut self, event: &dyn Event);
    fn id(&self) -> u32;
}

pub type SharedLayer = Box<dyn Layer>;

// Layer stack implementation
pub struct LayerStack {
    layers: Vec<SharedLayer>,
    insert_index: usize, // Tracks the boundary between layers and overlays
}

impl LayerStack {
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            insert_index: 0,
        }
    }

    pub fn push_layer(&mut self, layer: SharedLayer) {
        self.layers.insert(self.insert_index, layer);
        self.insert_index += 1;
        self.layers[self.insert_index - 1].attach();
    }
    // Push an overlay (always goes on top)
    pub fn push_overlay(&mut self, overlay: SharedLayer) {
        self.layers.push(overlay);
        self.layers.last_mut().unwrap().attach();
    }
    // Pop a regular layer
    // Pop a regular layer
    pub fn pop_layer(&mut self, layer: SharedLayer) -> Option<SharedLayer> {
        // Get raw pointer of the target layer
        let target_ptr: *const dyn Layer = &*layer;

        // Search in regular layers (before insert_index)
        if let Some(pos) = self.layers[..self.insert_index].iter().position(|layer| {
            let layer_ptr: *const dyn Layer = &**layer;
            std::ptr::eq(target_ptr, layer_ptr)
        }) {
            // Layer found, remove it
            let removed_layer = self.layers.remove(pos);
            //removed_layer.detach();
            self.insert_index -= 1;
            Some(removed_layer)
        } else {
            None
        }
    }

    // Pop an overlay
    pub fn pop_overlay(&mut self, overlay: SharedLayer) -> Option<SharedLayer> {
        // Get raw pointer of the target overlay
        let target_ptr: *const dyn Layer = &*overlay;

        // Search in overlays section (after insert_index)
        if let Some(pos) = self.layers[self.insert_index..]
            .iter()
            .position(|overlay| {
                let overlay_ptr: *const dyn Layer = &**overlay;
                std::ptr::eq(target_ptr, overlay_ptr)
            })
            .map(|i| i + self.insert_index)
        // Convert to absolute index
        {
            // Overlay found, remove it
            let removed_overlay = self.layers.remove(pos);
            //removed_overlay.detach();
            Some(removed_overlay)
        } else {
            None
        }
    }
    // Iterate over all layers (both regular and overlays)
    pub fn iter(&self) -> impl Iterator<Item = &SharedLayer> {
        self.layers.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut SharedLayer> {
        self.layers.iter_mut()
    }
    // Iterate over layers in reverse (top to bottom)
    pub fn iter_rev(&self) -> impl Iterator<Item = &SharedLayer> {
        self.layers.iter().rev()
    }
}
