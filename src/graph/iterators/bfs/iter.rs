use std::{
    collections::{HashSet, VecDeque},
    hash::Hash,
};

use crate::{graph::Graph, marker::GraphType, vertex::Vertex};

/// BfsIterator is a Breadth-First Search iterator.
/// See <https://en.wikipedia.org/wiki/Breadth-first_search>.
pub struct BfsIterator<'a, Key, Value, Type>
where
    Key: Hash + Eq + Clone,
    Type: GraphType,
{
    /// A reference to the graph on which the iterator is operating.
    pub graph: &'a Graph<Key, Value, Type>,

    /// Queue is used to store the vertices that need to be visited.
    pub queue: VecDeque<&'a Key>,

    /// Visited is used to store the vertices that have already been visited.
    pub visited: HashSet<&'a Key>,
}

impl<'a, Key, Value, Type> Iterator for BfsIterator<'a, Key, Value, Type>
where
    Key: Hash + Eq + Clone,
    Type: GraphType,
{
    type Item = &'a Vertex<Key, Value>;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.queue.is_empty() {
            let key = self.queue.pop_front().unwrap();

            let vertex = self.graph.get_vertex(key)?;

            if self.visited.contains(&vertex.key()) {
                continue;
            }
            self.visited.insert(vertex.key());

            for edge in vertex.adjancency_list() {
                self.queue.push_back(edge.to());
            }

            let result = Some(vertex);
            return result;
        }

        None
    }
}
