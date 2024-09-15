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

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    macro_rules! assert_option_edges {
        ($e_1:expr, $e_2:expr) => {
            assert_eq!($e_1.is_none(), $e_2.is_none());

            if let Some(e_1_unwrapped) = $e_1 {
                let e_2_unwrapped = unsafe { $e_2.unwrap_unchecked() };

                assert_eq!(
                    e_1_unwrapped.from(),
                    e_2_unwrapped.from(),
                    "edges sources are not equal"
                );
                assert_eq!(
                    e_1_unwrapped.to(),
                    e_2_unwrapped.to(),
                    "edges destination are not equal"
                );
                assert_eq!(
                    e_1_unwrapped.weight, e_2_unwrapped.weight,
                    "edges weights are not equal"
                );
            }
        };
    }

    #[test]
    fn insert_edge_test() {
        let input_data = vec![
            Vertex::new(1, "one"),
            Vertex {
                key: 2,
                value: "two",
                adj: vec![Edge::new_unweighted(2, 3)],
            },
            Vertex {
                key: 3,
                value: "three",
                adj: vec![Edge::new(3, 4, 69)],
            },
        ]
        .into_iter();

        let insertion_data = vec![
            Edge::new_unweighted(1, 2),
            Edge::new_unweighted(2, 4),
            Edge::new_unweighted(3, 4),
        ]
        .into_iter();

        let expected_results = vec![
            (
                Vertex {
                    key: 1,
                    value: "one",
                    adj: vec![Edge::new_unweighted(1, 2)],
                },
                None,
            ),
            (
                Vertex {
                    key: 2,
                    value: "two",
                    adj: vec![Edge::new_unweighted(2, 3), Edge::new_unweighted(2, 4)],
                },
                None,
            ),
            (
                Vertex {
                    key: 3,
                    value: "three",
                    adj: vec![Edge::new_unweighted(2, 3)],
                },
                Some(Edge::new(2, 3, 69)),
            ),
        ]
        .into_iter();

        let dataset = input_data
            .zip(insertion_data)
            .zip(expected_results)
            .map(|set| (set.0 .0, set.0 .1, set.1 .0, set.1 .1));

        for (mut input, insertion, expected, removed) in dataset {
            let result = input.insert_edge(insertion);
            assert_option_edges!(result, removed);

            assert_eq!(input.key, expected.key);
            assert_eq!(input.value, expected.value);
            for (input_edge, expected_edge) in input.adj.iter().zip(expected.adj.iter()) {
                assert_option_edges!(Some(input_edge), Some(expected_edge));
            }
        }
    }
}
