/// Edge is a directed connection between two vertices in a graph that has a weight.
#[derive(Debug)]
pub struct Edge<Key>
where
    Key: Eq,
{
    /// Identifies the source vertex in the graph
    from: Key,

    /// Identifies the destination vertex in the graph
    to: Key,

    /// Identifies the weight of the edge
    pub weight: i64,
}

impl<Key> Edge<Key>
where
    Key: Eq,
{
    pub fn new(from: Key, to: Key, weight: i64) -> Self {
        Self { from, to, weight }
    }

    pub fn new_unweighted(from: Key, to: Key) -> Self {
        Self::new(from, to, 0)
    }

    pub fn from(&self) -> &Key {
        &self.from
    }

    pub fn to(&self) -> &Key {
        &self.to
    }
}

impl<Key> PartialEq for Edge<Key>
where
    Key: Eq,
{
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from && self.to == other.to
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn edge_eq_test() {
        assert!(
            Edge::new(1, 2, 69) == Edge::new(1, 2, 69),
            "edges must be absolutely equal in this case"
        );
        assert!(
            Edge::new(1, 2, 69) == Edge::new(1, 2, 96),
            "edge weight does not affect equality"
        );
        assert!(
            !(Edge::new(1, 2, 69) == Edge::new(3, 4, 69)),
            "edge must not be equal because its direction is not the same"
        );
        assert!(
            !(Edge::new(1, 2, 69) == Edge::new(3, 2, 96)),
            "edge must not be equal because its source is not the same"
        );
        assert!(
            !(Edge::new(1, 2, 69) == Edge::new(1, 3, 96)),
            "edge must not be equal because its destination is not the same"
        );
    }

    #[test]
    fn edge_ne_test() {
        assert!(
            !(Edge::new(1, 2, 69) != Edge::new(1, 2, 69)),
            "edges must be absolutely equal in this case"
        );
        assert!(
            !(Edge::new(1, 2, 69) != Edge::new(1, 2, 96)),
            "edge weight does not affect equality"
        );
        assert!(
            Edge::new(1, 2, 69) != Edge::new(3, 4, 69),
            "edge must not be equal because its direction is not the same"
        );
        assert!(
            Edge::new(1, 2, 69) != Edge::new(3, 2, 96),
            "edge must not be equal because its source is not the same"
        );
        assert!(
            Edge::new(1, 2, 69) != Edge::new(1, 3, 96),
            "edge must not be equal because its destination is not the same"
        );
    }
}
