use super::{GenericRefIter, Graph, IdEdge, IdGraph, IdVertex};
use std::collections::{HashMap, VecDeque};

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

    pub fn from(vertices: Vec<V>, edges: Vec<E>) -> Result<Self, ()> {
        let mut g = DirectedGraph::new();

        for v in vertices {
            g.insert_vertex(v);
        }

        for e in edges {
            g.insert_edge(e)?;
        }

        Ok(g)
    }
}

impl<'a, V, E> IdGraph<'a, V, E> for DirectedGraph<V, E>
where
    V: IdVertex + 'a,
    E: IdEdge + 'a,
{
    fn out_edges_id(&'a self, vertex: usize) -> Box<GenericRefIter<'a, Self::TEdge>> {
        match self.edges.get(&vertex) {
            Some(te) => Box::new(te.iter()),
            None => Box::new(std::iter::empty()),
        }
    }

    fn remove_vertex_id(&mut self, vertex: usize) -> Option<Self::TVertex> {
        self.vertices.remove(&vertex)
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
        self.out_edges_id(vertex.id())
    }

    fn insert_edge(&mut self, edge: Self::TEdge) -> Result<(), ()> {
        let (from, to) = (edge.from(), edge.to());
        if self.vertices.contains_key(&from) && self.vertices.contains_key(&to) {
            let te = self.edges.entry(edge.from()).or_insert_with(Vec::new);
            te.push(edge);
            self.edges_len += 1;
            Ok(())
        } else {
            Err(())
        }
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
        self.remove_vertex_id(vertex.id())
    }
}

pub fn topo_sort<V, E>(graph: &DirectedGraph<V, E>) -> Vec<usize>
where
    V: IdVertex,
    E: IdEdge,
{
    let mut q = VecDeque::new();

    let mut din = HashMap::new();

    let mut res = Vec::with_capacity(graph.len_vertex());

    graph.vertices().for_each(|x| {
        din.insert(x.id(), 0);
    });

    graph
        .edges()
        .for_each(|x| *din.get_mut(&x.to()).unwrap() += 1);

    din.iter()
        .filter(|&(_, &v)| v == 0)
        .for_each(|(&k, _)| q.push_back(k));

    while let Some(u) = q.pop_front() {
        res.push(u);
        graph.out_edges_id(u).for_each(|x| {
            let v = din.get_mut(&x.to()).unwrap();
            if *v > 0 {
                *v -= 1;
                if *v == 0 {
                    q.push_back(x.to());
                }
            }
        })
    }

    res
}

#[cfg(test)]
mod tests {
    use super::DirectedGraph;
    use super::Graph;
    use crate::graph::structs::{IdEdge as IdE, IdVertex as IdV};

    #[test]
    fn build() {
        let mut g =
            DirectedGraph::from(vec![IdV::new(0), IdV::new(1)], vec![IdE::new(0, 1)]).unwrap();

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
        g.insert_edge(IdE { from: 1, to: 0 }).unwrap();
        assert_eq!(1, g.len_edge());
        g.insert_vertex(IdV { id: 2 });
        assert_eq!(3, g.len_vertex());
        g.remove_vertex(&IdV { id: 1 });
        assert_eq!(2, g.len_vertex());
    }

    #[test]
    fn topo() {
        let g = DirectedGraph::from(
            (0..6).map(IdV::new).collect::<Vec<_>>(),
            vec![
                IdE::new(0, 1),
                IdE::new(1, 2),
                IdE::new(2, 3),
                IdE::new(4, 5),
                IdE::new(3, 4),
            ],
        )
        .unwrap();

        assert_eq!((0..6).collect::<Vec<_>>(), super::topo_sort(&g));
    }
}
