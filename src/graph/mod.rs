mod directed;
mod iterators;
mod undirected;

use iterators::{
    bfs::{iter::BfsIterator, iter_mut::BfsIteratorMut},
    dfs::{iter::DfsIterator, iter_mut::DfsIteratorMut},
};

use crate::{
    edge::Edge,
    marker::{Directed, GraphType},
    vertex::Vertex,
};
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    marker::PhantomData,
};

pub struct Graph<Key, Value, Type = Directed>
where
    Key: Hash + Eq + Clone,
    Type: GraphType,
{
    vertices: HashMap<Key, Vertex<Key, Value>>,

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

    pub fn insert(&mut self, key: Key, value: Value) -> Option<Value> {
        self.insert_vertex(Vertex::new(key, value)).map(|v| v.value)
    }

    pub fn get(&self, key: &Key) -> Option<&Value> {
        self.get_vertex(key).map(|v| &v.value)
    }

    pub fn get_mut(&mut self, key: &Key) -> Option<&mut Value> {
        self.get_vertex_mut(key).map(|v| &mut v.value)
    }

    pub fn remove(&mut self, key: &Key) -> Option<Value> {
        self.remove_vertex(key).map(|v| v.value)
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
    }

    pub fn len(&self) -> usize {
        self.vertices.len()
    }

    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }

    pub fn insert_vertex(&mut self, vertex: Vertex<Key, Value>) -> Option<Vertex<Key, Value>> {
        self.vertices.insert(vertex.key().clone(), vertex)
    }

    pub fn get_vertex(&self, key: &Key) -> Option<&Vertex<Key, Value>> {
        self.vertices.get(key)
    }

    pub fn get_vertex_mut<'a>(&'a mut self, key: &Key) -> Option<&'a mut Vertex<Key, Value>> {
        self.vertices.get_mut(key)
    }

    pub fn remove_vertex(&mut self, key: &Key) -> Option<Vertex<Key, Value>> {
        self.vertices.remove(key)
    }

    pub fn get_edge(&mut self, from: &Key, to: &Key) -> Option<&Edge<Key>> {
        self.get_vertex(from)?.get_edge(to)
    }

    pub fn vertecies(&self) -> impl Iterator<Item = &Vertex<Key, Value>> {
        self.vertices.iter().map(|v| v.1)
    }

    pub fn vertecies_mut(&mut self) -> impl Iterator<Item = &mut Vertex<Key, Value>> {
        self.vertices.iter_mut().map(|v| v.1)
    }

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

    pub fn bfs<'a>(&'a self, from: &'a Key) -> BfsIterator<'a, Key, Value, Type> {
        BfsIterator {
            graph: self,
            stack: vec![from].into(),
            visited: HashSet::new(),
        }
    }

    pub fn bfs_mut<'a>(&'a mut self, from: &'a Key) -> BfsIteratorMut<'a, Key, Value, Type> {
        BfsIteratorMut {
            graph: self,
            stack: vec![from].into(),
            visited: HashSet::new(),
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
