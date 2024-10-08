use std::{collections::HashSet, hash::Hash};

use crate::{graph::Graph, marker::GraphType, vertex::Vertex};

/// DfsIteratorMut is analog to DfsIterator, but mutable.
/// See DfsIterator for more information.
pub struct DfsIteratorMut<'a, Key, Value, Type>
where
    Key: Hash + Eq + Clone,
    Type: GraphType,
{
    pub graph: &'a mut Graph<Key, Value, Type>,
    pub stack: Vec<&'a Key>,
    pub visited: HashSet<&'a Key>,
}

impl<'a, Key, Value, Type> Iterator for DfsIteratorMut<'a, Key, Value, Type>
where
    Key: Hash + Eq + Clone,
    Type: GraphType,
{
    type Item = &'a mut Vertex<Key, Value>;
    fn next(&mut self) -> Option<Self::Item> {
        let key = self.stack.pop().unwrap();

        unsafe {
            let vertex = if let Some(vertex) = self.graph.get_vertex_mut(key) {
                vertex as *mut Vertex<Key, Value>
            } else {
                return None;
            };
            self.visited.insert((*vertex).key());
            for edge in (*vertex).adjancency_list() {
                let key = edge.to();
                if !self.visited.contains(key) {
                    self.stack.push(key);
                }
            }
            Some(&mut *vertex)
        }
    }
}
