pub struct Edge<Key> {
    pub from: Key,
    pub to: Key,
    pub weight: i64,
}

impl<Key> Edge<Key> {
    pub fn new(from: Key, to: Key, weight: i64) -> Self {
        Self { from, to, weight }
    }

    pub fn new_unweighted(from: Key, to: Key) -> Self {
        Self::new(from, to, 0)
    }
}
