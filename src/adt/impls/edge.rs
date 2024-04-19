use crate::adt::{Edge, EdgeAttr};

pub struct BasicEdge<E> {
    id: usize,
    value: E,
    attrs: Vec<EdgeAttr>,
}

impl<E> BasicEdge<E> {
    pub fn new() -> Self {
        BasicEdge<E> {}
    }
}

impl<E> Edge<E> for BasicEdge<E> {
    fn id(&self) -> usize {
        self.id
    }

    fn value(&self) -> &E {
        &self.value
    }

    fn value_mut(&mut self) -> &mut E {
        &mut self.value
    }

    fn attrs(&self) -> Vec<EdgeAttr> {
        self.attrs.clone()
    }
}
