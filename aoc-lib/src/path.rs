use num_traits::{Bounded, Num};
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

// TODO test these - I don't think they're usable in this state, lifetimes are confused

/// Find a path using breadth first search
pub fn bfs<'a, N: Eq + Hash, ItN: Iterator<Item = &'a N>, Ctx>(
    context: &Ctx,
    start_node: &'a N,
    neighbours: fn(&Ctx, &N) -> ItN,
    is_end: fn(&Ctx, &N) -> bool,
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
            if is_end(context, link) {
                prev.insert(link, current);
                // Could unfold the path properly here, but we only care about length
                let mut at = link;
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
struct AStarNodeData<Node, Cost: Num> {
    f: Cost,
    g: Cost,
    came_from: Node,
}

/// Find shortest path using A*
pub fn a_star<
    Node: Eq + Hash + Copy + Clone,
    ItNC: IntoIterator<Item = (Node, Cost)>,
    Ctx,
    Cost: Num + Bounded + Copy + Ord,
>(
    context: &Ctx,
    start_node: &Node,
    neighbours: fn(&Ctx, &Node) -> ItNC,
    heuristic: fn(&Ctx, &Node) -> Cost,
    is_end: fn(&Ctx, &Node) -> bool,
) -> Option<Vec<Node>> {
    let mut open = HashSet::new();
    open.insert(*start_node);

    let mut nodes: HashMap<Node, AStarNodeData<Node, Cost>> = HashMap::new();
    nodes.insert(
        *start_node,
        AStarNodeData {
            f: heuristic(context, start_node),
            g: Cost::zero(),
            came_from: *start_node,
        },
    );

    while !open.is_empty() {
        // Select "best" node from open set. This is a bit of a mess.
        let current = open
            .iter()
            .map(|n| (n, nodes.get(n).map(|x| x.f).unwrap_or_else(|| Cost::max_value())))
            .min_by(|(_, s1), (_, s2)| s1.cmp(s2))
            .map(|(x, _)| *x)?;
        if is_end(context, &current) {
            // reconstruct path
            // includes start and end nodes
            let mut p = current;
            let mut result = Vec::new();
            result.push(p);
            loop {
                let v = nodes[&p].came_from;
                if v == p {
                    result.reverse();
                    return Some(result);
                } else {
                    // println!("{:?}", v);
                    result.push(v);
                    p = v;
                }
            }
        }
        open.remove(&current);
        // Check neighbours
        let node = nodes.get(&current);
        let current_g = node.map(|x| x.g).unwrap_or_else(|| Cost::max_value());
        for (n, c) in neighbours(context, &current) {
            let tentative_g = current_g + c;
            let neighbour = nodes.get(&n);
            if tentative_g < neighbour.map(|x| x.g).unwrap_or_else(|| Cost::max_value()) {
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
