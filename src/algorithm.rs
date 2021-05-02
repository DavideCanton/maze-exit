use std::cmp::{max, Ordering};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::Hash;

#[derive(Default, Debug)]
pub struct Info {
    pub max_length: usize,
    pub nodes: u32,
}

pub struct QueueNode<'a, N> {
    heuristic: &'a dyn Fn(&N) -> f64,
    pub node: N,
    pub depth: f64,
}

impl<'a, N> QueueNode<'a, N> {
    fn new(node: N, heuristic: &'a dyn Fn(&N) -> f64) -> Self {
        QueueNode::with_depth(node, heuristic, 0.0)
    }

    fn with_depth(node: N, heuristic: &'a dyn Fn(&N) -> f64, depth: f64) -> Self {
        QueueNode {
            heuristic,
            node,
            depth,
        }
    }
}

impl<'a, N> PartialEq for QueueNode<'a, N>
where
    N: Eq,
{
    fn eq(&self, other: &Self) -> bool {
        self.node.eq(&other.node)
    }
}

impl<'a, N> Eq for QueueNode<'a, N> where N: Eq {}

impl<'a, N> PartialOrd for QueueNode<'a, N>
where
    N: Eq,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a, N> Ord for QueueNode<'a, N>
where
    N: Eq,
{
    fn cmp(&self, other: &Self) -> Ordering {
        let closure = self.heuristic;
        let h1 = closure(&self.node) + self.depth as f64;
        let h2 = closure(&other.node) + other.depth as f64;
        let res = h1.partial_cmp(&h2);
        res.unwrap().reverse()
    }
}

#[derive(Debug)]
pub struct Child<N> {
    pub node: N,
    pub weight: f64,
}

impl<N> Child<N> {
    pub fn new(node: N, weight: f64) -> Self {
        Child { node, weight }
    }
}

pub fn a_star<N>(
    start: N,
    goal: impl Fn(&N) -> bool,
    heuristic: impl Fn(&N) -> f64,
    mut gen_children: impl FnMut(&N, Option<&N>) -> Vec<Child<N>>,
    mut callback: impl FnMut(&BinaryHeap<QueueNode<N>>),
) -> (Option<Vec<N>>, Info)
where
    N: Hash + Eq + Clone,
{
    let mut depth: HashMap<N, f64> = HashMap::new();
    let mut parents: HashMap<N, N> = HashMap::new();
    let mut queue: BinaryHeap<QueueNode<N>> = BinaryHeap::new();
    let mut visited: HashSet<N> = HashSet::new();
    let mut info = Info::default();

    depth.insert(start.clone(), 0.0);
    queue.push(QueueNode::new(start.clone(), &heuristic));

    while !queue.is_empty() {
        callback(&queue);

        info.nodes += 1;
        info.max_length = max(info.max_length, queue.len());

        let current = queue.pop().unwrap();
        let current_node = current.node;
        visited.insert(current_node.clone());

        if goal(&current_node) {
            let mut node = current_node;
            let mut path = vec![];
            while parents.contains_key(&node) {
                path.push(node.clone());
                node = parents.get(&node).expect("not found in parents").clone();
            }
            path.push(start);
            path.reverse();
            return (Some(path), info);
        }

        let parent = parents.get(&current_node);

        for generated in gen_children(&current_node, parent) {
            let successor = generated.node.clone();

            let weight = generated.weight;
            let successor_depth = *depth.get(&current_node).unwrap_or(&0.0) + weight;
            if visited.contains(&successor) {
                continue;
            }

            let ex_depth = *depth.get(&successor).unwrap_or(&f64::INFINITY);
            if successor_depth < ex_depth {
                parents.insert(successor.clone(), current_node.clone());
                depth.insert(successor.clone(), successor_depth);
                let new_node =
                    QueueNode::with_depth(successor.clone(), &heuristic, successor_depth);
                queue.push(new_node);
            }
        }
    }

    (None, info)
}
