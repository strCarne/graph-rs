pub struct Edge<Key>
where
    Key: Eq,
{
    from: Key,
    to: Key,
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

    fn ne(&self, other: &Self) -> bool {
        self.from != other.from || self.to != other.to
    }
}
