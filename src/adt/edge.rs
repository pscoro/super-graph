pub enum EdgeAttr {
    Directed,
    Undirected,
    Labelled,
    Unlabelled,
    Weighted,
    Unweighted,
    Coloured,
    Uncoloured,
}

pub trait Edge<E> {
    fn id(&self) -> usize;
    fn value(&self) -> &E;
    fn value_mut(&mut self) -> &mut E;
    fn attrs(&self) -> Vec<EdgeAttr>;
}

