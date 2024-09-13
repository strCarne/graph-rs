use std::hash::Hash;

use crate::{graph::Graph, marker::GraphType, vertex::Vertex};

pub mod bfs;
pub mod dfs;

impl<Key, Value, Type> Graph<Key, Value, Type>
where
    Key: Hash + Eq + Clone,
    Type: GraphType,
{
    pub fn vertecies(&self) -> impl Iterator<Item = &Vertex<Key, Value>> {
        self.vertices.iter().map(|v| v.1)
    }

    pub fn vertecies_mut(&mut self) -> impl Iterator<Item = &mut Vertex<Key, Value>> {
        self.vertices.iter_mut().map(|v| v.1)
    }
}
