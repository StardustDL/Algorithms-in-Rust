pub mod structs;

pub mod directed_graph;
pub use directed_graph::EdgeIter;

pub trait Vertex {}

impl<T> Vertex for T {}

pub trait Edge {}

impl<T> Edge for T {}

pub trait Graph<'a> {
    type TVertex: Vertex + 'a;
    type TEdge: Edge + 'a;
    type VertexIter: Iterator<Item = &'a Self::TVertex>;
    type EdgeIter: Iterator<Item = &'a Self::TEdge>;

    /// Gets the number of vertices
    fn len_vertex(&self) -> usize;

    /// Gets the number of edges
    fn len_edge(&self) -> usize;

    /// Gets vertices
    fn vertices(&'a self) -> Self::VertexIter;

    /// Gets edges
    fn edges(&'a self) -> Self::EdgeIter;
}

pub trait IdVertex: Vertex {
    fn id(&self) -> usize;
}

pub trait IdEdge: Edge {
    fn from(&self) -> usize;
    fn to(&self) -> usize;
}
