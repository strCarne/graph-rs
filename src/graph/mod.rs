use std::{collections::HashMap, hash::Hash, marker::PhantomData};

use crate::{marker::GraphType, vertex::Vertex};

pub struct Graph<Key, Value, Type>
where
    Key: Hash + Eq + Clone,
    Type: GraphType,
{
    pub vertices: HashMap<Key, Vertex<Key, Value>>,

    _type: PhantomData<Type>,
}

impl<Key, Value, Type> Graph<Key, Value, Type>
where
    Key: Hash + Eq + Clone,
    Type: GraphType,
{
    pub fn new() -> Self {
        Self {
            vertices: HashMap::new(),
            _type: PhantomData,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            vertices: HashMap::with_capacity(capacity),
            _type: PhantomData,
        }
    }
}

impl<Key, Value, Type> Default for Graph<Key, Value, Type>
where
    Key: Hash + Eq + Clone,
    Type: GraphType,
{
    fn default() -> Self {
        Self::new()
    }
}
