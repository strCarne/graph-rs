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
    ) -> Result<Option<Edge<Key>>, &'static str> {
        if !self.contains(&from) || !self.contains(&to) {
            return Err("failure");
        }

        let v = self
            .get_vertex_mut(&from)
            .expect("source vertex must exist");

        Ok(v.insert_edge(to, weight))
    }

    /// Same as the insert_edge, but new edge's weight is zero
    pub fn insert_edge_unweighted(
        &mut self,
        from: Key,
        to: Key,
    ) -> Result<Option<Edge<Key>>, &'static str> {
        self.insert_edge(from, to, 0)
    }

    /// Removes an edge from the directed graph.
    /// So edge will be removed in one directions.
    /// Returns the removed edge if it existed.
    /// Returns None if the edge did not exist in the list.
    /// Returns Err if one of the vertices doesn't exist.
    pub fn remove_edge(&mut self, from: &Key, to: &Key) -> Result<Option<Edge<Key>>, &'static str> {
        if !self.contains(from) || !self.contains(to) {
            return Err("failure");
        }

        Ok(self
            .get_vertex_mut(from)
            .expect("source vertex must exist")
            .remove_edge(to))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn insert_edge_test() {
        let mut graph: Graph<i32, i32, Directed> = Graph::new();
        graph.insert(1, 1);
        graph.insert(2, 2);
        graph.insert(3, 3);

        assert_eq!(graph.insert_edge_unweighted(1, 2), Ok(None));
        assert_eq!(graph.insert_edge_unweighted(1, 3), Ok(None));
        assert_eq!(graph.insert_edge_unweighted(1, 4), Err("failure"));
        assert_eq!(graph.insert_edge_unweighted(2, 3), Ok(None));
        assert_eq!(
            graph.insert_edge_unweighted(1, 3),
            Ok(Some(Edge::new(1, 3, 0)))
        );
    }

    #[test]
    fn remove_edge_test() {
        let mut graph: Graph<i32, i32, Directed> = Graph::new();
        graph.insert(1, 1);
        graph.insert(2, 2);
        graph.insert(3, 3);
        graph.insert(4, 4);

        graph
            .insert_edge_unweighted(1, 2)
            .expect("both vertices must exist");
        graph
            .insert_edge_unweighted(1, 3)
            .expect("both vertices must exist");
        graph
            .insert_edge_unweighted(1, 4)
            .expect("both vertices must exist");
        graph
            .insert_edge_unweighted(2, 3)
            .expect("both vertices must exist");

        assert_eq!(graph.remove_edge(&2, &4), Ok(None));
        assert_eq!(graph.remove_edge(&1, &3), Ok(Some(Edge::new(1, 3, 0))));
        assert_eq!(graph.remove_edge(&1, &5), Err("failure"));
        assert_eq!(graph.remove_edge(&2, &3), Ok(Some(Edge::new(2, 3, 0))));
    }
}
