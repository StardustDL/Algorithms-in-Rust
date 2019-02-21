use super::{GenericRefIter, Graph, IdEdge, IdVertex};
use std::collections::HashMap;

#[derive(Default)]
pub struct DirectedGraph<V, E>
where
    V: IdVertex,
    E: IdEdge,
{
    vertices: HashMap<usize, V>,
    edges: HashMap<usize, Vec<E>>,
    edges_len: usize,
}

impl<'a, V, E> DirectedGraph<V, E>
where
    V: IdVertex + 'a,
    E: IdEdge + 'a,
{
    pub fn new() -> Self {
        DirectedGraph {
            vertices: HashMap::new(),
            edges: HashMap::new(),
            edges_len: 0,
        }
    }

    pub fn from(vertices: Vec<V>, edges: Vec<E>) -> Self {
        let mut v = HashMap::new();
        let mut e = HashMap::new();
        let es_len = edges.len();
        for iv in vertices {
            v.insert(iv.id(), iv);
        }
        for ie in edges {
            let te = e.entry(ie.from()).or_insert_with(Vec::new);
            te.push(ie);
        }
        DirectedGraph {
            vertices: v,
            edges: e,
            edges_len: es_len,
        }
    }
}

impl<'a, V, E> Graph<'a> for DirectedGraph<V, E>
where
    V: IdVertex + 'a,
    E: IdEdge + 'a,
{
    type TVertex = V;
    type TEdge = E;

    fn len_vertex(&self) -> usize {
        self.vertices.len()
    }

    fn len_edge(&self) -> usize {
        self.edges_len
    }

    fn vertices(&'a self) -> Box<GenericRefIter<'a, Self::TVertex>> {
        Box::new(self.vertices.values())
    }

    fn edges(&'a self) -> Box<GenericRefIter<'a, Self::TEdge>> {
        Box::new(self.edges.values().flatten())
    }

    fn out_edges(&'a self, vertex: &Self::TVertex) -> Box<GenericRefIter<'a, Self::TEdge>> {
        match self.edges.get(&vertex.id()) {
            Some(te) => Box::new(te.iter()),
            None => Box::new(std::iter::empty()),
        }
    }

    fn insert_edge(&mut self, edge: Self::TEdge) {
        let te = self.edges.entry(edge.from()).or_insert_with(Vec::new);
        te.push(edge);

        self.edges_len += 1;
    }

    fn insert_vertex(&mut self, vertex: Self::TVertex) -> Option<Self::TVertex> {
        self.vertices.insert(vertex.id(), vertex)
    }

    fn remove_edge(&mut self, edge: &Self::TEdge) -> Option<Self::TEdge>
    where
        Self::TEdge: PartialEq,
    {
        let mut ten = match self.edges.entry(edge.from()) {
            std::collections::hash_map::Entry::Occupied(e) => e,
            std::collections::hash_map::Entry::Vacant(_) => return None,
        };
        let te = ten.get_mut();
        let pos = match te.iter().position(|e| e == edge) {
            Some(val) => val,
            None => return None,
        };
        let res = te.swap_remove(pos);

        self.edges_len -= 1;

        if te.is_empty() {
            ten.remove();
        }
        Some(res)
    }

    /// Remove vertex
    fn remove_vertex(&mut self, vertex: &Self::TVertex) -> Option<Self::TVertex>
    where
        Self::TEdge: PartialEq,
    {
        self.vertices.remove(&vertex.id())
    }
}

#[cfg(test)]
mod tests {
    use super::DirectedGraph;
    use super::Graph;
    use crate::graph::structs::{IdEdge as IdE, IdVertex as IdV};

    #[test]
    fn build() {
        let mut g = DirectedGraph::from(vec![IdV::new(0), IdV::new(1)], vec![IdE::new(0, 1)]);

        assert_eq!(2, g.len_vertex());
        assert_eq!(1, g.len_edge());

        let mut vs: Vec<usize> = g.vertices().map(|v| v.id).collect();
        vs.sort();
        assert_eq!(vs[0], 0);
        assert_eq!(vs[1], 1);

        let es: Vec<&IdE> = g.edges().collect();
        assert_eq!(es[0].from, 0);
        assert_eq!(es[0].to, 1);

        g.remove_edge(&IdE { from: 0, to: 1 });
        assert_eq!(0, g.len_edge());
        g.insert_edge(IdE { from: 1, to: 0 });
        assert_eq!(1, g.len_edge());
        g.insert_vertex(IdV { id: 2 });
        assert_eq!(3, g.len_vertex());
        g.remove_vertex(&IdV { id: 1 });
        assert_eq!(2, g.len_vertex());
    }
}
