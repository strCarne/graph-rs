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
