pub struct DirectedAcyclicGraph<V: Vertex<T>, E: Edge<U>> {
    pub adj_matrix: AdjMatrix<V, E>,
}
