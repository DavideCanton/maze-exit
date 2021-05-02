use crate::position::Pos;
use std::f64::consts::SQRT_2;

pub fn heur_diag(goal: Pos) -> impl Fn(&Pos) -> f64 {
    let goal = goal.clone();
    move |&node| {
        let diff = node - goal;
        let dx = diff.x.abs();
        let dy = diff.y.abs();

        let mut min = dx as f64;
        let mut max = dy as f64;
        if min > max {
            min = dy as f64;
            max = dx as f64;
        }
        return (min * (SQRT_2 - 1.0) + max) * 1.001;
    }
}
