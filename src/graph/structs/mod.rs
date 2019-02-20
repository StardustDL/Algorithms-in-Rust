pub struct IdVertex {
    pub id: usize,
}

impl super::IdVertex for IdVertex {
    fn id(&self) -> usize {
        self.id
    }
}

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
