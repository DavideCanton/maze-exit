use std::f64::consts::SQRT_2;

use crate::{maze::Maze, position::Position};

pub trait MazeHeuristic {
    fn compute_heuristic(&self, node: Position) -> f64;
}

#[derive(Default)]
pub struct DiagonalHeuristic {
    goal: Position,
}

impl DiagonalHeuristic {
    pub fn new(maze: &Maze) -> Self {
        DiagonalHeuristic { goal: maze.goal() }
    }
}

impl MazeHeuristic for DiagonalHeuristic {
    fn compute_heuristic(&self, node: Position) -> f64 {
        let diff = (node - self.goal).abs();

        let min = diff.min_element() as f64;
        let max = diff.max_element() as f64;

        (min * (SQRT_2 - 1.0) + max) * 1.001
    }
}
