use super::{IdEdge, LengthEdge};
use crate::ds::DisjointSet;
use std::collections::HashMap;

pub fn minimum_spanning_tree<T: IdEdge + LengthEdge>(edges: &[&T]) -> Option<isize> {
    if edges.is_empty() {
        return Some(0);
    }

    let mut map = HashMap::new();
    for edge in edges {
        let len = map.len();
        map.entry(edge.from()).or_insert(len);
        let len = map.len();
        map.entry(edge.to()).or_insert(len);
    }

    let mut sorted: Vec<_> = edges.iter().collect();
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
    use super::super::Graph;
    use super::minimum_spanning_tree;
    use crate::graph::structs::{IdVertex as IdV, LengthIdEdge as LIdE};

    #[test]
    fn in_graph() {
        let g = DirectedGraph::from(
            vec![IdV::new(0), IdV::new(1)],
            vec![LIdE::new(0, 1, 2), LIdE::new(0, 1, 1)],
        )
        .unwrap();
        let edges: Vec<_> = g.edges().collect();
        assert_eq!(Some(1), minimum_spanning_tree(&edges))
    }

    #[test]
    fn raw() {
        let edges = vec![LIdE::new(0, 1, 2), LIdE::new(0, 1, 1)];
        let edges: Vec<_> = edges.iter().collect();
        assert_eq!(Some(1), minimum_spanning_tree(&edges));
    }
}
