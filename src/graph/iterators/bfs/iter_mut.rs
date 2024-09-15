use std::{
    collections::{HashSet, VecDeque},
    hash::Hash,
};

use crate::{graph::Graph, marker::GraphType, vertex::Vertex};

/// BfsIteratorMut is analog to BfsIterator, but mutable.
/// See BfsIterator for more information.
pub struct BfsIteratorMut<'a, Key, Value, Type>
where
    Key: Hash + Eq + Clone,
    Type: GraphType,
{
    pub graph: &'a mut Graph<Key, Value, Type>,
    pub queue: VecDeque<&'a Key>,
    pub visited: HashSet<&'a Key>,
}

impl<'a, Key, Value, Type> Iterator for BfsIteratorMut<'a, Key, Value, Type>
where
    Key: Hash + Eq + Clone,
    Type: GraphType,
{
    type Item = &'a mut Vertex<Key, Value>;
    fn next(&mut self) -> Option<Self::Item> {
        while !self.queue.is_empty() {
            let key = self.queue.pop_front().unwrap();

            unsafe {
                let vertex = if let Some(vertex) = self.graph.get_vertex_mut(key) {
                    vertex as *mut Vertex<Key, Value>
                } else {
                    return None;
                };
                self.visited.insert(&(*vertex).key());
                for edge in (*vertex).adjancency_list() {
                    let key = &edge.to();
                    if !self.visited.contains(key) {
                        self.queue.push_back(key);
                    }
                }
                return Some(&mut *vertex);
            }
        }

        None
    }
}
