use std::{
    collections::{HashSet, VecDeque},
    hash::Hash,
};

use crate::{graph::Graph, marker::GraphType, vertex::Vertex};

pub struct BfsIteratorMut<'a, Key, Value, Type>
where
    Key: Hash + Eq + Clone,
    Type: GraphType,
{
    pub graph: &'a mut Graph<Key, Value, Type>,
    pub stack: VecDeque<&'a Key>,
    pub visited: HashSet<&'a Key>,
}

impl<'a, Key, Value, Type> Iterator for BfsIteratorMut<'a, Key, Value, Type>
where
    Key: Hash + Eq + Clone,
    Type: GraphType,
{
    type Item = &'a mut Vertex<Key, Value>;
    fn next(&mut self) -> Option<Self::Item> {
        while !self.stack.is_empty() {
            let key = self.stack.pop_front().unwrap();

            unsafe {
                let vertex = self.graph.get_vertex_mut(key).unwrap() as *mut Vertex<Key, Value>;
                self.visited.insert(&(*vertex).key());
                for edge in (*vertex).adjancency_list() {
                    let key = &edge.to();
                    if !self.visited.contains(key) {
                        self.stack.push_back(key);
                    }
                }
                return Some(&mut *vertex);
            }
        }

        None
    }
}
