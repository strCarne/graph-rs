use std::{collections::HashSet, hash::Hash};

use crate::{graph::Graph, marker::GraphType, vertex::Vertex};

/// DfsIterator is a Depth-First Search iterator.
/// See https://en.wikipedia.org/wiki/Depth-first_search
pub struct DfsIterator<'a, Key, Value, Type>
where
    Key: Hash + Eq + Clone,
    Type: GraphType,
{
    /// A reference to the graph on which the iterator is operating.
    pub graph: &'a Graph<Key, Value, Type>,

    /// Stack is used to store the vertices that need to be visited.
    pub stack: Vec<&'a Key>,

    /// Visited is used to store the vertices that have already been visited.
    pub visited: HashSet<&'a Key>,
}

impl<'a, Key, Value, Type> Iterator for DfsIterator<'a, Key, Value, Type>
where
    Key: Hash + Eq + Clone,
    Type: GraphType,
{
    type Item = &'a Vertex<Key, Value>;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.stack.is_empty() {
            let key = self.stack.pop().unwrap();
            let vertex = self.graph.get_vertex(key).unwrap();

            if self.visited.contains(&vertex.key()) {
                continue;
            }
            self.visited.insert(&vertex.key());

            for edge in vertex.adjancency_list() {
                self.stack.push(&edge.to());
            }

            let result = Some(vertex);
            return result;
        }

        None
    }
}
