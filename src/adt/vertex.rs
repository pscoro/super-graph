pub enum VertexAttr {
    Labelled,
    Unlabelled,
    Weighted,
    Unweighted,
    Coloured,
    Uncoloured,
    Leaf,
    Root,
    Source,
    Sink,
    Isolated,
    Connected,
}

pub trait Vertex<V> {
    fn id(&self) -> usize;
    fn value(&self) -> &V;
    fn value_mut(&mut self) -> &mut V;
    fn attrs(&self) -> Vec<VertexAttr>;
}
