use std::vec::Vec;

use super::event::Event;

pub trait Layer {
    fn on_attach(&self) {}
    fn on_detach(&self) {}
    fn on_update(&self);
    fn on_event(&self, event: &mut dyn Event);
    // fn on_event<E: Event>(event: &mut E);
}


pub struct LayerStack {
    layers: Vec::<Box<dyn Layer>>,
    layer_insert_index: usize
}

impl LayerStack {

    pub fn new() -> Self {
        LayerStack {
            layers: vec![],
            layer_insert_index: 0
        }
    }

    pub fn push_layer(&mut self, layer: Box<dyn Layer>) {
        // let cap: usize = self.layers.capacity();
        if self.layers.capacity() <= self.layer_insert_index {
            self.layers.reserve(1);
        }
        self.layers.insert(self.layer_insert_index, layer);
        self.layer_insert_index += 1;
    }
    pub fn pop_layer(&mut self) -> Box<dyn Layer>  {todo!()}
    pub fn push_overlay(&mut self, overlay: Box<dyn Layer>) {
        self.layers.push(overlay);
    }
    pub fn pop_overlay(&mut self) -> Box<dyn Layer> {todo!()}

    pub fn run_layers(&self) {

    }

}

pub struct LayerStackIter<'a> {
    stack: &'a Vec<Box<dyn Layer>>,
    index: usize
}

impl<'a> Iterator for LayerStackIter<'a> {
    type Item = &'a dyn Layer;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > 0 {
          self.index -= 1;
          Some(self.stack[self.index].as_ref())
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a LayerStack {
    type Item = &'a dyn Layer;

    type IntoIter = LayerStackIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        LayerStackIter {
            stack: &self.layers,
            index: self.layers.len()
        }
    }
}