use super::{IdEdge, IdGraph, IdVertex, LengthEdge};
use crate::ds::DisjointSet;
use std::collections::HashMap;

pub fn minimum_spanning_tree<'a, V, E>(graph: &'a IdGraph<'a, V, E>) -> Option<isize>
where
    V: IdVertex,
    E: IdEdge + LengthEdge,
{
    if graph.len_vertex() == 0 {
        return Some(0);
    }

    let mut map = HashMap::new();
    graph.vertices().enumerate().for_each(|(k, v)| {
        map.insert(v.id(), k);
    });

    let mut sorted: Vec<_> = graph.edges().collect();
    sorted.sort_by(|&x, &y| x.length().cmp(&y.length()));
    let mut ds = DisjointSet::new(map.len(), true);
    let mut res = 0;

    for edge in sorted {
        let (from, to) = (map[&edge.from()], map[&edge.to()]);
        if ds.in_same(from, to) {
            continue;
        }
        ds.unite(from, to);
        res += edge.length();
        if ds.is_one() {
            break;
        }
    }

    if ds.is_one() {
        Some(res)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::super::DirectedGraph;
    use super::minimum_spanning_tree;
    use crate::graph::structs::{IdVertex as IdV, LengthIdEdge as LIdE};

    #[test]
    fn in_graph() {
        let g = DirectedGraph::from(
            vec![IdV::new(0), IdV::new(1)],
            vec![LIdE::new(0, 1, 2), LIdE::new(0, 1, 1)],
        )
        .unwrap();
        assert_eq!(Some(1), minimum_spanning_tree(&g))
    }
}
