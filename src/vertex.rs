use crate::edge::Edge;

pub struct Vertex<Key, Value>
where
    Key: Eq,
{
    key: Key,
    pub value: Value,
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

    pub fn key(&self) ->  &Key {
        &self.key
    }

    pub fn adjancency_list(&self) -> &Vec<Edge<Key>> {
        &self.adj
    }

    pub fn get_edge(&self, to: &Key) -> Option<&Edge<Key>> {
        for edge in &self.adj {
            if edge.to == *to {
                return Some(edge);
            }
        }
        None
    }

    pub fn remove_edge(&mut self, to: &Key) -> Option<Edge<Key>> {
        for (i, edge) in self.adj.iter().enumerate() {
            if edge.to == *to {
                return Some(self.adj.swap_remove(i));
            }
        }
        None
    }
}
