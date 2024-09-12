mod iter;
mod iter_mut;

use std::{collections::HashSet, hash::Hash};

use iter::DfsIterator;
use iter_mut::DfsIteratorMut;

use crate::{graph::Graph, marker::GraphType};

impl<Key, Value, Type> Graph<Key, Value, Type>
where
    Key: Hash + Eq + Clone,
    Type: GraphType,
{
    pub fn dfs<'a>(&'a self, from: &'a Key) -> DfsIterator<'a, Key, Value, Type> {
        DfsIterator {
            graph: self,
            stack: vec![from],
            visited: HashSet::new(),
        }
    }

    pub fn dfs_mut<'a>(&'a mut self, from: &'a Key) -> DfsIteratorMut<'a, Key, Value, Type> {
        DfsIteratorMut {
            graph: self,
            stack: vec![from],
            visited: HashSet::new(),
        }
    }
}
