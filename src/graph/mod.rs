mod general;
mod undirected;

use crate::{marker::GraphType, vertex::Vertex};
use std::{collections::HashMap, hash::Hash, marker::PhantomData};

pub struct Graph<Key, Value, Type>
where
    Key: Hash + Eq + Clone,
    Type: GraphType,
{
    vertices: HashMap<Key, Vertex<Key, Value>>,

    _type: PhantomData<Type>,
}
