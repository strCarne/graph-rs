use std::{
    collections::{HashSet, VecDeque},
    hash::Hash,
};

use crate::{graph::Graph, marker::GraphType, vertex::Vertex};

pub struct BfsIterator<'a, Key, Value, Type>
where
    Key: Hash + Eq + Clone,
    Type: GraphType,
{
    pub graph: &'a Graph<Key, Value, Type>,
    pub stack: VecDeque<&'a Key>,
    pub visited: HashSet<&'a Key>,
}

impl<'a, Key, Value, Type> Iterator for BfsIterator<'a, Key, Value, Type>
where
    Key: Hash + Eq + Clone,
    Type: GraphType,
{
    type Item = &'a Vertex<Key, Value>;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.stack.is_empty() {
            let key = self.stack.pop_front().unwrap();
            let vertex = self.graph.get_vertex(key).unwrap();

            if self.visited.contains(&vertex.key) {
                continue;
            }
            self.visited.insert(&vertex.key);

            for edge in vertex.adj.iter() {
                self.stack.push_back(&edge.to);
            }

            let result = Some(vertex);
            return result;
        }

        None
    }
}
