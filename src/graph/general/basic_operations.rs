use std::{collections::HashMap, hash::Hash, marker::PhantomData};

use crate::{graph::Graph, marker::GraphType, vertex::Vertex};

impl<Key, Value, Type> Graph<Key, Value, Type>
where
    Key: Hash + Eq + Clone,
    Type: GraphType,
{
    pub fn new() -> Self {
        Self {
            vertices: HashMap::new(),
            _type: PhantomData,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            vertices: HashMap::with_capacity(capacity),
            _type: PhantomData,
        }
    }

    pub fn insert(&mut self, key: Key, value: Value) -> Option<Value> {
        self.insert_vertex(Vertex::new(key, value)).map(|v| v.value)
    }

    pub fn get(&self, key: &Key) -> Option<&Value> {
        self.get_vertex(key).map(|v| &v.value)
    }

    pub fn get_mut(&mut self, key: &Key) -> Option<&mut Value> {
        self.get_vertex_mut(key).map(|v| &mut v.value)
    }

    pub fn remove(&mut self, key: &Key) -> Option<Value> {
        self.remove_vertex(key).map(|v| v.value)
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
    }

    pub fn len(&self) -> usize {
        self.vertices.len()
    }

    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }

    pub fn insert_vertex(&mut self, vertex: Vertex<Key, Value>) -> Option<Vertex<Key, Value>> {
        self.vertices.insert(vertex.key().clone(), vertex)
    }

    pub fn get_vertex(&self, key: &Key) -> Option<&Vertex<Key, Value>> {
        self.vertices.get(key)
    }

    pub fn get_vertex_mut<'a>(&'a mut self, key: &Key) -> Option<&'a mut Vertex<Key, Value>> {
        self.vertices.get_mut(key)
    }

    pub fn remove_vertex(&mut self, key: &Key) -> Option<Vertex<Key, Value>> {
        self.vertices.remove(key)
    }
}

impl<Key, Value, Type> Default for Graph<Key, Value, Type>
where
    Key: Hash + Eq + Clone,
    Type: GraphType,
{
    fn default() -> Self {
        Self::new()
    }
}
