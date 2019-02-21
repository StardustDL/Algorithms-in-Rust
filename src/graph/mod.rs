pub mod structs;

pub mod directed_graph;
pub use directed_graph::DirectedGraph;

mod mst;
pub use mst::*;

pub trait Vertex {}

impl<T> Vertex for T {}

pub trait Edge {}

impl<T> Edge for T {}

pub type GenericRefIter<'a, T> = dyn Iterator<Item = &'a T> + 'a;

pub trait Graph<'a> {
    type TVertex: Vertex + 'a;
    type TEdge: Edge + 'a;

    /// Gets the number of vertices
    fn len_vertex(&self) -> usize;

    /// Gets the number of edges
    fn len_edge(&self) -> usize;

    /// Gets vertices
    fn vertices(&'a self) -> Box<GenericRefIter<'a, Self::TVertex>>;

    /// Gets edges
    fn edges(&'a self) -> Box<GenericRefIter<'a, Self::TEdge>>;

    /// Gets out edges from `vertex`.
    fn out_edges(&'a self, vertex: &Self::TVertex) -> Box<GenericRefIter<'a, Self::TEdge>>;

    /// Inserts an edge into the graph.
    fn insert_edge(&mut self, edge: Self::TEdge);

    /// Insert vertex
    /// If the graph did not have this vertex present, None is returned.
    /// If the graph did have this vertex present, the value is updated, and the old value is returned.
    fn insert_vertex(&mut self, vertex: Self::TVertex) -> Option<Self::TVertex>;

    /// Remove edge
    fn remove_edge(&mut self, edge: &Self::TEdge) -> Option<Self::TEdge>
    where
        Self::TEdge: PartialEq;

    /// Remove vertex
    fn remove_vertex(&mut self, vertex: &Self::TVertex) -> Option<Self::TVertex>
    where
        Self::TEdge: PartialEq;
}

pub trait IdVertex: Vertex {
    fn id(&self) -> usize;
}

pub trait IdEdge: Edge {
    fn from(&self) -> usize;
    fn to(&self) -> usize;
}

pub trait LengthEdge: Edge {
    fn length(&self) -> isize;
}
