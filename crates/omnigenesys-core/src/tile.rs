use std::collections::HashMap;

pub type TileId = u16;
pub const EMPTY: TileId = 0;

pub struct TileRegistry{
    to_id: HashMap<String, TileId>,
    to_name: Vec<String>,
    next_id: TileId,
}

impl TileRegistry{
    pub fn new() -> Self {
        let mut registry = TileRegistry {
            to_id: HashMap::new(),
            to_name: Vec::new(),
            next_id: 0,
        };
        registry.get_or_insert("empty");
        registry
    }

    pub fn name(&self, id: TileId) -> &str {
        &self.to_name[id as usize]
    }

    pub fn get_or_insert(&mut self, name: &str) -> TileId {
        if let Some(&id) = self.to_id.get(name) {
            return id;
        }
        let id = self.next_id;
        self.to_id.insert(name.to_string(), id);
        self.to_name.push(name.to_string());
        self.next_id += 1;
        id
    }
}