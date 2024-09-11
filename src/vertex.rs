use crate::edge::Edge;

pub struct Vertex<Key, Value> {
    pub key: Key,
    pub value: Value,
    pub adj: Vec<Edge<Key>>,
}

impl<Key, Value> Vertex<Key, Value> {
    pub fn new(key: Key, value: Value) -> Self {
        Vertex {
            key,
            value,
            adj: Vec::new(),
        }
    }
}
