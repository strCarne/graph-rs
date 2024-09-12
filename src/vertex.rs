use crate::edge::Edge;

pub struct Vertex<Key, Value>
where
    Key: Eq,
{
    pub key: Key,
    pub value: Value,
    pub adj: Vec<Edge<Key>>,
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
}
