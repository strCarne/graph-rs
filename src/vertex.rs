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

}
