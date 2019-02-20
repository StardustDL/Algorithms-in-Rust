use super::{Graph, IdEdge, IdVertex};
use std::collections::HashMap;

#[derive(Default)]
pub struct DirectedGraph<V, E>
where
    V: IdVertex,
    E: IdEdge,
{
    vertices: HashMap<usize, V>,
    edges: HashMap<usize, Vec<E>>,
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
        }
    }

    pub fn from(vertices: Vec<V>, edges: Vec<E>) -> Self {
        let mut v = HashMap::new();
        let mut e = HashMap::new();
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
    type VertexIter = VertexIter<'a, Self::TVertex>;
    type EdgeIter = EdgeIter<'a, Self::TEdge>;

    fn len_vertex(&self) -> usize {
        self.vertices.len()
    }

    fn len_edge(&self) -> usize {
        self.edges.len()
    }

    fn vertices(&'a self) -> Self::VertexIter {
        self.vertices.values()
    }

    fn edges(&'a self) -> Self::EdgeIter {
        EdgeIter {
            inner: self.edges.values(),
            cur: None,
        }
    }
}

pub type VertexIter<'a, T> = std::collections::hash_map::Values<'a, usize, T>;

pub struct EdgeIter<'a, T> {
    inner: std::collections::hash_map::Values<'a, usize, Vec<T>>,
    cur: Option<std::slice::Iter<'a, T>>,
}

impl<'a, T> Iterator for EdgeIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cit) = &mut self.cur {
            if let Some(x) = cit.next() {
                return Some(x);
            }
        }
        for tit in &mut self.inner {
            self.cur = Some(tit.iter());
            if let Some(cit) = &mut self.cur {
                if let Some(x) = cit.next() {
                    return Some(x);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::DirectedGraph;
    use super::Graph;
    use crate::graph::structs::{IdEdge as IdE, IdVertex as IdV};

    #[test]
    fn build() {
        let g = DirectedGraph::from(
            vec![IdV { id: 0 }, IdV { id: 1 }],
            vec![IdE { from: 0, to: 1 }],
        );

        assert_eq!(2, g.len_vertex());
        assert_eq!(1, g.len_edge());

        let vs: Vec<&IdV> = g.vertices().collect();
        assert_eq!(vs[0].id, 0);
        assert_eq!(vs[1].id, 1);

        let es: Vec<&IdE> = g.edges().collect();
        assert_eq!(es[0].from, 0);
        assert_eq!(es[0].to, 1);
    }
}
