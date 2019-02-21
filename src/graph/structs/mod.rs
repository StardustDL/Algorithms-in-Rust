#[derive(PartialEq, Eq)]
pub struct IdVertex {
    pub id: usize,
}

impl super::IdVertex for IdVertex {
    fn id(&self) -> usize {
        self.id
    }
}

impl IdVertex {
    pub fn new(id: usize) -> Self {
        IdVertex { id }
    }
}

#[derive(PartialEq, Eq)]
pub struct IdEdge {
    pub from: usize,
    pub to: usize,
}

impl super::IdEdge for IdEdge {
    fn from(&self) -> usize {
        self.from
    }

    fn to(&self) -> usize {
        self.to
    }
}

impl IdEdge {
    pub fn new(from: usize, to: usize) -> Self {
        IdEdge { from, to }
    }
}

#[derive(PartialEq, Eq)]
pub struct LengthIdEdge {
    pub inner: IdEdge,
    pub length: isize,
}

impl super::IdEdge for LengthIdEdge {
    fn from(&self) -> usize {
        self.inner.from
    }

    fn to(&self) -> usize {
        self.inner.to
    }
}

impl super::LengthEdge for LengthIdEdge {
    fn length(&self) -> isize {
        self.length
    }
}

impl LengthIdEdge {
    pub fn new(from: usize, to: usize, length: isize) -> Self {
        LengthIdEdge {
            inner: IdEdge::new(from, to),
            length,
        }
    }
}
