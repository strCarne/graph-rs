use std::hash::Hash;

use crate::{edge::Edge, graph::Graph, marker::Directed};

impl<Key, Value> Graph<Key, Value, Directed>
where
    Key: Hash + Eq + Clone,
{
    pub fn insert_edge(&mut self, from: Key, to: Key, weight: i64) -> Option<Edge<Key>> {
        let v = self.get_vertex_mut(&from)?;
        v.insert_edge(Edge::new(from.clone(), to.clone(), weight))
    }

    pub fn insert_edge_unweighted(&mut self, from: Key, to: Key) -> Option<Edge<Key>> {
        self.insert_edge(from, to, 0)
    }

    pub fn remove_edge(&mut self, from: &Key, to: &Key) -> Option<Edge<Key>> {
        self.get_vertex_mut(from)?.remove_edge(to)
    }
}
