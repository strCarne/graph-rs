use std::{collections::HashSet, hash::Hash};

use crate::{graph::Graph, marker::GraphType, vertex::Vertex};

pub struct DfsIterator<'a, Key, Value, Type>
where
    Key: Hash + Eq + Clone,
    Type: GraphType,
{
    pub graph: &'a Graph<Key, Value, Type>,
    pub stack: Vec<&'a Key>,
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

            if self.visited.contains(&vertex.key) {
                continue;
            }
            self.visited.insert(&vertex.key);

            for edge in vertex.adj.iter() {
                self.stack.push(&edge.to);
            }

            let result = Some(vertex);
            return result;
        }

        None
    }
}