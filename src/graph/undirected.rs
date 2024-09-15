use std::hash::Hash;

use crate::{edge::Edge, graph::Graph, marker::Undirected};

impl<Key, Value> Graph<Key, Value, Undirected>
where
    Key: Hash + Eq + Clone,
{
    /// Inserts a new edge into the undirected graph.
    /// So edge will be created in both directions.
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

        let v_1 = self
            .get_vertex_mut(&from)
            .expect("source vertex must exist");

        let removed_edge_1 = v_1.insert_edge(to.clone(), weight);

        let v_2 = self
            .get_vertex_mut(&to)
            .expect("destination vertex must exist");
        let removed_edge_2 = v_2.insert_edge(from, weight);

        if removed_edge_1.is_none() && removed_edge_2.is_none() {
            Ok(None)
        } else if removed_edge_1.is_some() && removed_edge_2.is_some() {
            Ok(removed_edge_1)
        } else {
            panic!("undirected graph must contain edges in both directions or contain none")
        }
    }

    /// Same as the inser_edge, but new edge's weight is zero
    pub fn insert_edge_unweighted(&mut self, from: Key, to: Key) -> Result<Option<Edge<Key>>, ()> {
        self.insert_edge(from, to, 0)
    }

    /// Removes an edge from the undirected graph.
    /// So edge will be removed in both directions.
    /// Returns the removed edge or None if it didn't exist.
    /// Returns None if the edge did not exist in the list.
    /// Returns Err if one of the vertices doesn't exist.
    pub fn remove_edge(&mut self, from: &Key, to: &Key) -> Result<Option<Edge<Key>>, ()> {
        if !self.contains(from) || !self.contains(to) {
            return Err(());
        }

        self.get_vertex_mut(to)
            .expect("destination vertex must exist")
            .remove_edge(from);

        Ok(self
            .get_vertex_mut(from)
            .expect("source vertex must exist")
            .remove_edge(to))
    }
}
