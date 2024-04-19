use crate::adt::{Edge, Vertex};
use crate::utils::{Error, DefaultResult};

pub trait Graph<V: Vertex<T>, E: Edge<U>> {}

pub trait Subgraph<V: Vertex, E: Edge> where Self: Graph<V, E> {
    fn parent(&self) -> &dyn Graph<V, E>;
}

pub trait Iterable<V: Vertex, E: Edge> where Self: Graph<V, E> {
    fn vertices(&self) -> Vec<&V>;
    fn edges(&self) -> Vec<&E>;
}

pub trait Mutable<V: Vertex, E: Edge> where Self: Graph<V, E> {
    fn add_vertex(&mut self, vertex: V) -> DefaultResult;
    fn add_edge(&mut self, edge: E) -> DefaultResult;
    fn remove_vertex(&mut self, vertex: &V) -> DefaultResult;
    fn remove_edge(&mut self, edge: &E) -> DefaultResult;
}

pub trait Directed<V: Vertex, E: Edge> where Self: Graph<V, E> {
    fn in_edges(&self, vertex: &V) -> Vec<&E>;
    fn out_edges(&self, vertex: &V) -> Vec<&E>;
}

pub trait Weighted<V: Vertex, E: Edge> where Self: Graph<V, E> {
    fn weight(&self, edge: &E) -> f64;
}

pub trait Coloured<V: Vertex, E: Edge> where Self: Graph<V, E> {
    fn colour(&self, edge: &E) -> String;
}

pub trait LabelledEdges<V: Vertex, E: Edge> where Self: Graph<V, E> {
    fn label(&self, edge: &E) -> String;
}

pub trait LabelledVertices<V: Vertex, E: Edge> where Self: Graph<V, E> {
    fn label(&self, vertex: &V) -> String;
}

pub trait Interval<V: Vertex, E: Edge> where Self: Graph<V, E> {
    fn interval(&self, vertex: &V) -> (f64, f64);
}

pub trait IsComplete<V: Vertex, E: Edge> where Self: Graph<V, E> {
    fn is_complete(&self) -> bool;
}

pub trait Adjacency<V: Vertex<T>, E: Edge<U>> where Self: Graph<V, E> {
    fn adjacency(&self, vertex: &V) -> Vec<&V>;
}
