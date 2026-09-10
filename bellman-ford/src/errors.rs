use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BellmanFordError {
    /// The source index was >= the number of vertices.
    SourceOutOfBounds { source: usize, num_vertices: usize },

    /// The graph contains a negative-weight cycle.
    /// `cycle` lists the vertex indices tat form the cycle, in order.
    NegativeCycle { cycle: Vec<usize>},
}

impl fmt::Display for BellmanFordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BellmanFordError::SourceOutOfBounds { source, num_vertices } => write!(
                f,
                "source vertex {source} is out of bounds (graph has {num_vertices} vertices)"
            ),
            BellmanFordError::NegativeCycle { cycle } => {
                write!(f, "graph contains a negative-weight cycle: ")?;
                for (i, v) in cycle.iter().enumerate() {
                    if i > 0 {
                        write!(f, " -> ")?;
                    }
                    write!(f, "{v}")?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for BellmanFordError {}