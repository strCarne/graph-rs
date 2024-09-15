use std::hash::Hash;

use crate::{edge::Edge, graph::Graph, marker::Directed};

impl<Key, Value> Graph<Key, Value, Directed>
where
    Key: Hash + Eq + Clone,
{
    /// Inserts a new edge into the directed graph.
    /// So edge will be created in one directions.
    /// Returns the old edge if it existed in the list.
    /// Returns None if the edge did not exist in the list.
    /// Returns Err if one of the vertices does not exist.
    pub fn insert_edge(
        &mut self,
        from: Key,
        to: Key,
        weight: i64,
    ) -> Result<Option<Edge<Key>>, ()> {
        if !self.contains(&from) || !self.contains(&to) {
            return Err(());
        }

        let v = self
            .get_vertex_mut(&from)
            .expect("source vertex must exist");

        Ok(v.insert_edge(to, weight))
    }

    /// Same as the insert_edge, but new edge's weight is zero
    pub fn insert_edge_unweighted(&mut self, from: Key, to: Key) -> Result<Option<Edge<Key>>, ()> {
        self.insert_edge(from, to, 0)
    }

    /// Removes an edge from the directed graph.
    /// So edge will be removed in one directions.
    /// Returns the removed edge if it existed.
    /// Returns None if the edge did not exist in the list.
    /// Returns Err if one of the vertices doesn't exist.
    pub fn remove_edge(&mut self, from: &Key, to: &Key) -> Result<Option<Edge<Key>>, ()> {
        if !self.contains(from) || !self.contains(to) {
            return Err(());
        }

        Ok(self
            .get_vertex_mut(from)
            .expect("source vertex must exist")
            .remove_edge(to))
    }
}
