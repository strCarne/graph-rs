/// GraphType trait is used to constarint the type of graph and acts as a marker.
pub trait GraphType {
    fn type_name() -> &'static str;
}

pub struct Directed;
impl GraphType for Directed {
    fn type_name() -> &'static str {
        "Directed"
    }
}

pub struct Undirected;
impl GraphType for Undirected {
    fn type_name() -> &'static str {
        "Undirected"
    }
}
