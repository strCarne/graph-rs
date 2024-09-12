use std::hash::Hash;

use crate::{edge::Edge, graph::Graph, marker::Undirected};

impl<Key, Value> Graph<Key, Value, Undirected>
where
    Key: Hash + Eq + Clone,
{
    pub fn insert_edge(&mut self, from: Key, to: Key, weight: i64) -> Option<Edge<Key>> {
        let v_1 = self.get_vertex_mut(&from)?;
        let removed_edge_1 = v_1.insert_edge(Edge::new(from.clone(), to.clone(), weight));

        let v_2 = self.get_vertex_mut(&to)?;
        let removed_edge_2 = v_2.insert_edge(Edge::new(to, from, weight));

        if removed_edge_1.is_none() && removed_edge_2.is_none() {
            None
        } else if removed_edge_1.is_some() && removed_edge_2.is_some() {
            removed_edge_1
        } else {
            panic!("undirected graph must contain edges in both directions or contain none")
        }
    }

    pub fn insert_edge_unweighted(&mut self, from: Key, to: Key) -> Option<Edge<Key>> {
        self.insert_edge(from, to, 0)
    }

    pub fn remove_edge(&mut self, from: &Key, to: &Key) -> Option<Edge<Key>> {
        self.get_vertex_mut(to)?.remove_edge(from);
        self.get_vertex_mut(from)?.remove_edge(to)
    }
}
