mod iter;
mod iter_mut;

use std::{collections::HashSet, hash::Hash};

use iter::BfsIterator;
use iter_mut::BfsIteratorMut;

use crate::{graph::Graph, marker::GraphType};

impl<Key, Value, Type> Graph<Key, Value, Type>
where
    Key: Hash + Eq + Clone,
    Type: GraphType,
{
    pub fn bfs<'a>(&'a self, from: &'a Key) -> impl Iterator + 'a {
        BfsIterator {
            graph: self,
            stack: vec![from].into(),
            visited: HashSet::new(),
        }
    }

    pub fn bfs_mut<'a>(&'a mut self, from: &'a Key) -> impl Iterator + 'a {
        BfsIteratorMut {
            graph: self,
            stack: vec![from].into(),
            visited: HashSet::new(),
        }
    }
}
