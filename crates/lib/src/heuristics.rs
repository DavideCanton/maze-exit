use std::f64::consts::SQRT_2;

use crate::{maze::Maze, position::Pos};

pub trait MazeHeuristic {
    fn compute_heuristic(&self, node: Pos) -> f64;
}

#[derive(Default)]
pub struct DiagonalHeuristic {
    goal: Pos,
}

impl DiagonalHeuristic {
    pub fn new(maze: &Maze) -> Self {
        DiagonalHeuristic { goal: maze.goal() }
    }
}

impl MazeHeuristic for DiagonalHeuristic {
    fn compute_heuristic(&self, node: Pos) -> f64 {
        let diff = node - self.goal;
        let dx = diff.x.abs();
        let dy = diff.y.abs();

        let mut min = dx as f64;
        let mut max = dy as f64;
        if min > max {
            min = dy as f64;
            max = dx as f64;
        }

        (min * (SQRT_2 - 1.0) + max) * 1.001
    }
}
