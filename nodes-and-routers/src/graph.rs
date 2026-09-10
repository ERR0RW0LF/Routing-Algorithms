

pub struct Graph {
    pub vertices: Vec<Vertex>,
    pub edges: Vec<Edge>,
}

pub struct Vertex {

}

pub struct Edge {
    pub u: usize,
    pub v: usize,
    pub weight: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum Distance {
    Finite(i32),
    Infinite,
}

#[derive(Debug, Clone, Copy)]
pub enum Predecessor {
    Vertex(usize),
    Unset,
}
