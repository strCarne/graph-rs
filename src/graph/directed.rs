use std::hash::Hash;

use crate::{edge::Edge, graph::Graph, marker::Directed};

impl<Key, Value> Graph<Key, Value, Directed>
where
    Key: Hash + Eq + Clone,
{
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

    pub fn insert_edge_unweighted(&mut self, from: Key, to: Key) -> Result<Option<Edge<Key>>, ()> {
        self.insert_edge(from, to, 0)
    }

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
