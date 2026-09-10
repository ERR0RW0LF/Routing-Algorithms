use nodes_and_routers::graph::Predecessor;

pub(crate) fn find_cycle(
    predecessor: &[Predecessor],
    start_u: usize,
    start_v: usize,
) -> Option<Vec<usize>> {
    let n = predecessor.len();
    let mut visited = vec![false; n];
    visited[start_v] = true;

    // Walk backward from start_u until we hit an already-visited node.
    let mut x = start_u;
    while !visited[x] {
        visited[x] = true;
        x = predecessor_vertex(predecessor, x)?;
    }

    // `x` is on the cycle. Reconstruct it.
    let mut cycle = vec![x];
    let mut y = predecessor_vertex(predecessor, x)?;
    while y != x {
        cycle.push(y);
        y = predecessor_vertex(predecessor, y)?;
    }
    cycle.push(x); // close the cycle for nicer printing
    cycle.reverse();
    Some(cycle)
}

fn predecessor_vertex(predecessor: &[Predecessor], i: usize) -> Option<usize> {
    match &predecessor[i] {
        Predecessor::Vertex(p) => Some(*p),
        Predecessor::Unset => None,
    }
}