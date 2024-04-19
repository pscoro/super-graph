pub struct AdjMatrix<K: Hash + Eq, V: EdgeType> {
    pub matrix: HashMap<K, HashMap<K, V>>,
}
