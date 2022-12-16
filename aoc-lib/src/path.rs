use num_traits::{Bounded, Num};
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

/// Find a path using breadth first search
pub fn bfs<'a, N: Eq + Hash, ItN: Iterator<Item = &'a N>, Ctx>(
    start_node: &'a N,
    end_node: &'a N,
    context: &Ctx,
    neighbours: fn(&Ctx, &N) -> ItN,
) -> Option<Vec<&'a N>> {
    // Empty queue - "open". Add the start node.
    let mut open = VecDeque::new();
    open.push_back(start_node);
    // Visited nodes, track where they were visited from
    let mut prev: HashMap<&N, &N> = HashMap::new();
    prev.insert(start_node, start_node);
    // Dequeue from open
    while !open.is_empty() {
        let current = open.pop_front().expect("Queue cannot be empty");
        for link in neighbours(context, current) {
            // If we're connected to the end then that'll do
            if link == end_node {
                prev.insert(link, current);
                // Could unfold the path properly here, but we only care about length
                let mut at = end_node;
                let mut result = vec![at];
                loop {
                    let a = prev[at];
                    if a == at {
                        return Some(result);
                    }
                    result.push(a);
                    at = a;
                }
            }
            // And connected node to open, remember prev
            prev.entry(link).or_insert_with(|| {
                open.push_back(link);
                current
            });
        }
    }
    None
}

/// Track A* node data
struct AStarNodeData<'a, N, C: Num> {
    f: C,
    g: C,
    came_from: &'a N,
}

/// Find shortest path using A*
/// TODO multiple ends - success by function
pub fn a_star<
    'a,
    N: Eq + Hash,
    ItNC: Iterator<Item = (&'a N, C)>,
    Ctx,
    C: Num + Bounded + Copy + Ord,
>(
    start_node: &'a N,
    end_node: &'a N,
    context: &Ctx,
    neighbours: fn(&Ctx, &N) -> ItNC,
    heuristic: fn(&Ctx, &N) -> C,
) -> Option<Vec<&'a N>> {
    let mut open = HashSet::new();
    open.insert(start_node);

    let mut nodes: HashMap<&N, AStarNodeData<'a, N, C>> = HashMap::new();
    nodes.insert(
        start_node,
        AStarNodeData {
            f: heuristic(context, start_node),
            g: C::zero(),
            came_from: start_node,
        },
    );

    while !open.is_empty() {
        // Select "best" node from open set. This is a bit of a mess.
        let current = open
            .iter()
            .map(|n| {
                (
                    n,
                    nodes.get(n).map(|x| x.f).unwrap_or_else(|| C::max_value()),
                )
            })
            .min_by(|(_, s1), (_, s2)| s1.cmp(s2))
            .map(|(x, _)| *x)?;
        if current == end_node {
            // reconstruct path
            let mut p = current;
            let mut result = Vec::new();
            loop {
                if let Some(v) = nodes.get(&p).map(|x| x.came_from) {
                    if v == p {
                        return Some(result);
                    } else {
                        // println!("{:?}", v);
                        result.push(v);
                        p = v;
                    }
                }
            }
        }
        open.remove(&current);
        // Check neighbours
        let node = nodes.get(&current);
        let current_g = node.map(|x| x.g).unwrap_or_else(|| C::max_value());
        for (n, c) in neighbours(context, current) {
            let tentative_g = current_g + c;
            let neighbour = nodes.get(&n);
            if tentative_g < neighbour.map(|x| x.g).unwrap_or_else(|| C::max_value()) {
                nodes.insert(
                    n,
                    AStarNodeData {
                        f: tentative_g + heuristic(context, &n),
                        g: tentative_g,
                        came_from: current,
                    },
                );
                open.insert(n);
            }
        }
    }
    None
}
