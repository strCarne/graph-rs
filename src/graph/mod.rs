pub mod iterators;

mod directed;
mod undirected;

use iterators::{
    bfs::{iter::BfsIterator, iter_mut::BfsIteratorMut},
    dfs::{iter::DfsIterator, iter_mut::DfsIteratorMut},
};

use crate::{
    edge::Edge,
    marker::{Directed, GraphType},
    tgf::{TgfConvertible, TrivialGraphFormat},
    vertex::Vertex,
};

use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    hash::Hash,
    marker::PhantomData,
    str::FromStr,
};

/// Graph is a directed or undirected graph container.
pub struct Graph<Key, Value, Type = Directed>
where
    Key: Hash + Eq + Clone,
    Type: GraphType,
{
    /// Vertices is a unordered map of vertices in the graph.
    /// Unoredered collection was choosed to allow non-standard
    /// keys like 'A' and so on.
    vertices: HashMap<Key, Vertex<Key, Value>>,

    /// Type is used to constarint the type of graph.
    /// This field is nothing after compilng and used only
    /// for constraint usage.
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

    /// Insert a new vertex into the graph.
    /// Returns the old value if the key already exists.
    /// Returns None if the key does not exist.
    pub fn insert(&mut self, key: Key, value: Value) -> Option<Value> {
        self.insert_vertex(Vertex::new(key, value)).map(|v| v.value)
    }

    /// Get a value from the graph by its key.
    /// Returns None if the key does not exist.
    /// Returns Some(&Value) if the key exists.
    pub fn get(&self, key: &Key) -> Option<&Value> {
        self.get_vertex(key).map(|v| &v.value)
    }

    /// Same as Graph::get, but returns a mutable reference.
    pub fn get_mut(&mut self, key: &Key) -> Option<&mut Value> {
        self.get_vertex_mut(key).map(|v| &mut v.value)
    }

    /// Removes a vertex from the graph.
    /// Returns the old value if the key exists.
    /// Returns None if the key does not exist.
    pub fn remove(&mut self, key: &Key) -> Option<Value> {
        self.remove_vertex(key).map(|v| v.value)
    }

    /// Clears the graph.
    pub fn clear(&mut self) {
        self.vertices.clear();
    }

    /// Returns the number of vertices in the graph.
    pub fn len(&self) -> usize {
        self.vertices.len()
    }

    /// Returns true if the graph is empty.
    /// Returns false if the graph is not empty.
    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }

    /// Inserts a new vertex into the graph.
    /// Returns the old vertex if the key already exists.
    /// Returns None if the key does not exist.
    pub fn insert_vertex(&mut self, vertex: Vertex<Key, Value>) -> Option<Vertex<Key, Value>> {
        self.vertices.insert(vertex.key().clone(), vertex)
    }

    /// Returns the vertex if it exists in the graph, otherwise returns None.
    pub fn get_vertex(&self, key: &Key) -> Option<&Vertex<Key, Value>> {
        self.vertices.get(key)
    }

    /// Same as Graph::get_vertex, but returns a mutable reference in case the key exists.
    pub fn get_vertex_mut<'a>(&'a mut self, key: &Key) -> Option<&'a mut Vertex<Key, Value>> {
        self.vertices.get_mut(key)
    }

    /// Removes the vertex if it exists in the graph and returns
    /// it, otherwise returns None.
    /// Also removes all edges connected to the vertex.
    pub fn remove_vertex(&mut self, key: &Key) -> Option<Vertex<Key, Value>> {
        let result = self.vertices.remove(key);

        // Make sure to delete all dangling edges
        for vertex in self.vertecies_mut() {
            let mut remove_queue = vec![];
            for (i, edge) in vertex.adjancency_list().iter().enumerate() {
                if edge.to() == key {
                    remove_queue.push(i);
                }
            }
            while let Some(i) = remove_queue.pop() {
                vertex.remove_edge_exact(i);
            }
        }

        result
    }

    /// Returns true if the vertex exists in the graph, otherwise returns false.
    pub fn contains(&self, key: &Key) -> bool {
        self.vertices.contains_key(key)
    }

    /// Returns the edge if it exists in the graph, otherwise returns None.
    pub fn get_edge(&mut self, from: &Key, to: &Key) -> Option<&Edge<Key>> {
        self.get_vertex(from)?.get_edge(to)
    }

    /// Returns an iterator over the vertices in the graph.
    pub fn vertecies(&self) -> impl Iterator<Item = &Vertex<Key, Value>> {
        self.vertices.iter().map(|v| v.1)
    }

    /// Same as Graph::vertecies, but returns a mutable reference.
    pub fn vertecies_mut(&mut self) -> impl Iterator<Item = &mut Vertex<Key, Value>> {
        self.vertices.iter_mut().map(|v| v.1)
    }

    /// Returns an Depth-First Search iterator over the edges in the graph
    /// starting from the vertex, whick key is equal to from.
    pub fn dfs<'a>(&'a self, from: &'a Key) -> DfsIterator<'a, Key, Value, Type> {
        DfsIterator {
            graph: self,
            stack: vec![from],
            visited: HashSet::new(),
        }
    }

    /// Same as Graph::dfs, but returns a mutable reference.
    pub fn dfs_mut<'a>(&'a mut self, from: &'a Key) -> DfsIteratorMut<'a, Key, Value, Type> {
        DfsIteratorMut {
            graph: self,
            stack: vec![from],
            visited: HashSet::new(),
        }
    }

    /// Returns a Breadth-First Search iterator over the edges in the graph
    /// starting from the vertex, whick key is equal to from.
    pub fn bfs<'a>(&'a self, from: &'a Key) -> BfsIterator<'a, Key, Value, Type> {
        BfsIterator {
            graph: self,
            queue: vec![from].into(),
            visited: HashSet::new(),
        }
    }

    /// Same as Graph::bfs, but returns a mutable reference.
    pub fn bfs_mut<'a>(&'a mut self, from: &'a Key) -> BfsIteratorMut<'a, Key, Value, Type> {
        BfsIteratorMut {
            graph: self,
            queue: vec![from].into(),
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

impl<Key, Value> TgfConvertible for Graph<Key, Value, Directed>
where
    Key: Hash + Eq + Clone + Display + FromStr,
    Value: Display + FromStr,
{
    /// Deserializes TrivialGraphFormat into Graph
    fn from_tgf(tgf: TrivialGraphFormat) -> Result<Self, String> {
        let mut graph: Self = Graph::new();

        enum ParsingState {
            Vertecies,
            Edges,
        }
        let mut state = ParsingState::Vertecies;

        let raw: String = tgf.into();
        for line in raw.lines() {
            match state {
                ParsingState::Vertecies => {
                    if line.starts_with('#') {
                        state = ParsingState::Edges;
                    } else {
                        let mut tokens = line.split_whitespace();

                        let first = if let Some(f) = tokens.next() {
                            f
                        } else {
                            continue;
                        };

                        let key = if let Ok(key) = first.parse() {
                            key
                        } else {
                            return Err(String::from("couldn't parse vertex"));
                        };

                        let value = {
                            let tmp_buf: Vec<&str> = tokens.collect();
                            if let Ok(value) = tmp_buf.join(" ").parse() {
                                value
                            } else {
                                return Err(String::from("couldn't parse vertex value"));
                            }
                        };

                        graph.insert(key, value);
                    }
                }

                ParsingState::Edges => {
                    if line.is_empty() {
                        break;
                    }

                    let mut tokens = line.split_whitespace();
                    let from: Key = if let Ok(key) = tokens.next().unwrap().parse() {
                        key
                    } else {
                        return Err(String::from("couldn't parse vertex"));
                    };
                    let to: Key = if let Ok(key) = tokens.next().unwrap().parse() {
                        key
                    } else {
                        return Err(String::from("couldn't parse vertex"));
                    };
                    graph
                        .insert_edge_unweighted(from, to)
                        .expect("graph must contain src and dst vertecies");
                }
            }
        }

        Ok(graph)
    }

    /// Serializes Graph into TrivialGraphFormat
    fn to_tgf(&self) -> TrivialGraphFormat {
        let mut buffer = String::new();

        for vertex in self.vertecies() {
            buffer += &format!("{} {}\n", vertex.key(), vertex.value);
        }

        buffer += "#\n";

        for vertex in self.vertecies() {
            for edge in vertex.adjancency_list() {
                buffer += &format!("{} {}\n", edge.from(), edge.to());
            }
        }

        buffer.into()
    }
}

impl<Key, Value, Type> ToString for Graph<Key, Value, Type>
where
    Key: Hash + Eq + Clone + ToString,
    Value: ToString,
    Type: GraphType,
{
    fn to_string(&self) -> String {
        if self.is_empty() {
            return String::from("[]");
        }

        let mut main_buf = String::new();
        main_buf.push('[');

        for vertex in self.vertecies() {
            let mut sub_buf = String::new();
            for edge in vertex.adjancency_list() {
                sub_buf += &format!("{}, ", edge.to().to_string());
            }
            let sub_buf = sub_buf.trim_end_matches(", ");

            main_buf += &format!(
                r#"
  {{
    "key": {},
    "value": "{}",
    "adjacent_vertecies_keys": [{}],
  }},"#,
                vertex.key().to_string(),
                vertex.value.to_string(),
                sub_buf
            );
        }

        let main_buf = main_buf.trim_end_matches(',');

        String::from(main_buf) + "\n]"
    }
}

#[cfg(test)]
mod tests;
