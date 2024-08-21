use std::{
    cmp::{max, Ordering},
    collections::{BinaryHeap, HashMap, HashSet},
};

use typed_arena::Arena;

use crate::{
    channel::ChannelSender, generator::ChildrenGenerator, heuristics::MazeHeuristic,
    position::Position,
};

#[derive(Default, Debug)]
pub struct Info {
    pub max_length: usize,
    pub nodes: u32,
}

#[derive(Debug)]
pub struct QueueNode {
    pub heuristic: f64,
    pub node: Position,
    pub depth: f64,
}

impl QueueNode {
    fn new(node: Position, heuristic: f64) -> Self {
        QueueNode::with_depth(node, heuristic, 0.0)
    }

    fn with_depth(node: Position, heuristic: f64, depth: f64) -> Self {
        QueueNode {
            heuristic,
            node,
            depth,
        }
    }
}

impl PartialEq for QueueNode {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for QueueNode {}

impl PartialOrd for QueueNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QueueNode {
    fn cmp(&self, other: &Self) -> Ordering {
        let h1 = self.heuristic + self.depth;
        let h2 = other.heuristic + other.depth;
        let res = h1.partial_cmp(&h2);
        res.unwrap().reverse()
    }
}

#[derive(Debug, PartialEq)]
pub struct Child {
    pub node: Position,
    pub weight: f64,
}

impl Child {
    pub fn new(node: Position, weight: f64) -> Self {
        Child { node, weight }
    }
}

pub enum Message {
    Enqueued(Position, f64),
    End(Vec<Position>),
}

pub fn a_star<G: ChildrenGenerator, C: ChannelSender<Message>>(
    start: Position,
    goal: Position,
    heuristic: &dyn MazeHeuristic,
    gen: &G,
    channel: C,
) -> (Option<Vec<Position>>, Info) {
    let node_arena = Arena::new();

    let mut depth = HashMap::new();
    let mut parents: HashMap<Position, Position> = HashMap::new();
    let mut queue: BinaryHeap<&QueueNode> = BinaryHeap::new();
    let mut visited = HashSet::new();
    let mut info = Info::default();

    depth.insert(start, 0.0);

    let start_node = QueueNode::new(start, heuristic.compute_heuristic(start));
    queue.push(node_arena.alloc(start_node));

    while let Some(current) = queue.pop() {
        info.nodes += 1;
        info.max_length = max(info.max_length, queue.len());

        let current_node = current.node;
        visited.insert(current_node);

        if current_node == goal {
            let mut node = current_node;
            let mut path = Vec::new();
            while parents.contains_key(&node) {
                path.push(node);
                node = *parents.get(&node).expect("not found in parents");
            }
            path.push(start);
            path.reverse();
            return (Some(path), info);
        }

        for generated in gen.generate_children(current_node, parents.get(&current_node).copied()) {
            let Child {
                node: successor,
                weight,
            } = generated;

            if visited.contains(&successor) {
                continue;
            }

            let successor_depth = depth.get(&current_node).unwrap_or(&0.0) + weight;

            let ex_depth = *depth.get(&successor).unwrap_or(&f64::INFINITY);
            if successor_depth < ex_depth {
                parents.insert(successor, current_node);
                depth.insert(successor, successor_depth);
                let new_node = QueueNode::with_depth(
                    successor,
                    heuristic.compute_heuristic(successor),
                    successor_depth,
                );

                if channel
                    .send(Message::Enqueued(successor, successor_depth))
                    .is_err()
                {
                    return (None, info);
                }

                queue.push(node_arena.alloc(new_node));
            }
        }
    }

    (None, info)
}
