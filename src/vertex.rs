use crate::edge::Edge;

/// Vertex is a container that represents a node in a graph.
/// It contains a key and a value, and a list of edges that connect to it.
pub struct Vertex<Key, Value>
where
    Key: Eq,
{
    /// Key is used to identify the vertex in the graph.
    key: Key,

    /// Value is used to store additional information about the vertex.
    pub value: Value,

    /// Adjancency list is a list of edges that connect to the vertex.
    adj: Vec<Edge<Key>>,
}

impl<Key, Value> Vertex<Key, Value>
where
    Key: Eq,
{
    pub fn new(key: Key, value: Value) -> Self {
        Vertex {
            key,
            value,
            adj: Vec::new(),
        }
    }

    /// Inserts a new edge into the vertex's adjacency list.
    /// Returns the old edge if it existed in the list.
    /// Returns None if the edge did not exist in the list.
    pub fn insert_edge(&mut self, new_edge: Edge<Key>) -> Option<Edge<Key>> {
        let result = 'a: {
            for (i, edge) in self.adj.iter().enumerate() {
                if edge == &new_edge {
                    let edge = self.adj.swap_remove(i);
                    break 'a Some(edge);
                }
            }
            None
        };

        self.adj.push(new_edge);

        result
    }

    pub fn key(&self) -> &Key {
        &self.key
    }

    pub fn adjancency_list(&self) -> &Vec<Edge<Key>> {
        &self.adj
    }

    /// Returns the edge if it exists in the adjacency list, otherwise returns None.
    pub fn get_edge(&self, to: &Key) -> Option<&Edge<Key>> {
        for edge in &self.adj {
            if *edge.to() == *to {
                return Some(edge);
            }
        }
        None
    }

    /// Removes the edge if it exists in the adjacency list, otherwise returns None.
    pub fn remove_edge(&mut self, to: &Key) -> Option<Edge<Key>> {
        for (i, edge) in self.adj.iter().enumerate() {
            if *edge.to() == *to {
                return Some(self.adj.swap_remove(i));
            }
        }
        None
    }
}
