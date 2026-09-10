use nodes_and_routers::graph::{Distance, Graph, Predecessor};

use crate::{errors::BellmanFordError, helpers::find_cycle};

pub mod errors;

mod helpers;




pub fn bellman_ford(
    graph: Graph,
    source: usize,
) -> Result<(Vec<Distance>, Vec<Predecessor>), BellmanFordError> {
    let n = graph.vertices.len();

    if source >= n {
        return Err(BellmanFordError::SourceOutOfBounds {
            source,
            num_vertices: n,
        });
    }

    let mut distance: Vec<Distance> = (0..n).map(|_| Distance::Infinite).collect();
    let mut predecessor: Vec<Predecessor> = (0..n).map(|_| Predecessor::Unset).collect();

    distance[source] = Distance::Finite(0);

    // Step 2: relax |V| - 1 times.
    for _ in 0..n.saturating_sub(1) {
        for edge in &graph.edges {
            if let Distance::Finite(du) = &distance[edge.u] {
                let new_dist = *du + edge.weight;
                let better = match &distance[edge.v] {
                    Distance::Infinite => true,
                    Distance::Finite(dv) => new_dist < *dv,
                };
                if better {
                    distance[edge.v] = Distance::Finite(new_dist);
                    predecessor[edge.v] = Predecessor::Vertex(edge.u);
                }
            }
        }
    }

    // Step 3: detect negative cycles.
    for edge in &graph.edges {
        if let Distance::Finite(du) = &distance[edge.u] {
            let new_dist = *du + edge.weight;
            let still_relaxable = match &distance[edge.v] {
                Distance::Infinite => true,
                Distance::Finite(dv) => new_dist < *dv,
            };

            if still_relaxable {
                predecessor[edge.v] = Predecessor::Vertex(edge.u);

                let cycle = find_cycle(&predecessor, edge.u, edge.v)
                    .ok_or(BellmanFordError::NegativeCycle { cycle: Vec::new() })?;

                return Err(BellmanFordError::NegativeCycle { cycle });
            }
        }
    }

    Ok((distance, predecessor))
}