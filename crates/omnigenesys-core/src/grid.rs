use indexmap::IndexMap;
use crate::coords::GridSize;
use crate::layer::Layer;

pub struct Grid {
    pub size: GridSize,
    pub seed: u64,
    layers: IndexMap<String, Layer>,
}

impl Grid{
    pub fn new(size: GridSize, seed: u64) -> Self {
        Grid { 
            size, 
            seed, 
            layers: IndexMap::new() 
        }
    }

    pub fn add_layer(&mut self, name: &str) -> bool {
        if self.layers.contains_key(name) {
            false
        } else {
            let layer = Layer::new(name.to_string(), self.size);
            self.layers.insert(name.to_string(), layer);
            true
        }
    }

    pub fn layer(&self, name: &str) -> Option<&Layer> {
        self.layers.get(name)
    }

    pub fn layer_mut(&mut self, name: &str) -> Option<&mut Layer> {
        self.layers.get_mut(name)
    }
}