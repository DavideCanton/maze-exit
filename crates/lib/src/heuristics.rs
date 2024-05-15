use std::f64::consts::SQRT_2;

use crate::position::Pos;

pub trait HeuristicFn {
    fn compute_heuristic(&self, node: &Pos) -> f64;
}

pub trait MazeHeuristic: HeuristicFn {
    fn set_goal(&mut self, goal: &Pos);
}

#[derive(Default)]
pub struct DiagonalHeuristic {
    goal: Option<Pos>,
}

impl MazeHeuristic for DiagonalHeuristic {
    fn set_goal(&mut self, goal: &Pos) {
        self.goal = Some(*goal);
    }
}

impl HeuristicFn for DiagonalHeuristic {
    fn compute_heuristic(&self, node: &Pos) -> f64 {
        let goal = self.goal.expect("No goal set on heuristic");
        let diff = *node - goal;
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
