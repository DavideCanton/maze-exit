use std::f64::consts::SQRT_2;

use itertools::Itertools;

use crate::algorithm::Child;
use crate::maze::Maze;
use crate::position::Pos;

pub type Path = Vec<Pos>;
pub type PathRef<'a> = &'a [Pos];

pub trait ChildrenGenerator {
    fn generate_children(&self, current: &Pos, parent: Option<&Pos>) -> Vec<Child>;
}

pub trait MazeChildrenGenerator: ChildrenGenerator {
    fn reconstruct_path(&self, path: PathRef) -> (Path, f64);
}

pub struct JpsGenerator<'a> {
    maze: &'a Maze,
}

impl<'a> JpsGenerator<'a> {
    pub fn new(maze: &'a Maze) -> Self {
        JpsGenerator { maze }
    }

    fn natural_neighbors(&self, current: &Pos) -> Vec<Child> {
        let pos = *current;
        vec![
            (pos.up(), 1.0),
            (pos.down(), 1.0),
            (pos.left(), 1.0),
            (pos.right(), 1.0),
            (pos.up_left(), SQRT_2),
            (pos.up_right(), SQRT_2),
            (pos.down_left(), SQRT_2),
            (pos.down_right(), SQRT_2),
        ]
        .into_iter()
        .filter(|(n, _)| self.maze.is_free(n))
        .map(|(n, c)| Child::new(n, c))
        .collect()
    }
    fn prune_neighbors(&self, current: &Pos, parent: &Pos, vec: &mut Vec<Child>) {
        let mv = (*current - *parent).sign();
        if mv.is_diagonal() {
            self.prune_diagonal(vec, current, mv);
        } else {
            self.prune_straight(vec, current, mv);
        }
    }

    fn do_jump(&self, current: &Pos, vec: Vec<Child>) -> Vec<Child> {
        vec.into_iter()
            .filter_map(|p| {
                self.jump_rec(current, &(p.node - *current), self.maze.goal())
                    .map(|j| Child::new(j, (j - *current).norm()))
            })
            .collect()
    }

    fn prune_diagonal(&self, vec: &mut Vec<Child>, current: &Pos, mv: Pos) {
        let mut pruned_list = vec![*current + mv];
        pruned_list.extend(mv.components().iter().map(|&p| *current + p));
        pruned_list.extend_from_slice(&self.compute_forced_diagonal(&(*current - mv), &mv));
        vec.retain(|p| pruned_list.contains(&p.node));
    }

    fn prune_straight(&self, vec: &mut Vec<Child>, current: &Pos, mv: Pos) {
        let mut pruned_list = vec![*current + mv];
        pruned_list.extend_from_slice(&self.compute_forced_straight(current, &mv));
        vec.retain(|p| pruned_list.contains(&p.node))
    }

    fn compute_forced_straight(&self, current: &Pos, mv: &Pos) -> Vec<Pos> {
        mv.orthogonal()
            .iter()
            .map(|&d| *current + d)
            .filter(|&n| self.maze.is_wall(&n))
            .map(|n| n + *mv)
            .collect()
    }

    fn compute_forced_diagonal(&self, current: &Pos, mv: &Pos) -> Vec<Pos> {
        mv.components()
            .iter()
            .filter_map(|&c| {
                let n = *current + c;
                if self.maze.is_wall(&n) {
                    Some(n + c)
                } else {
                    None
                }
            })
            .collect()
    }

    fn jump_rec(&self, current: &Pos, direction: &Pos, goal: &Pos) -> Option<Pos> {
        let next = *current + *direction;
        if !self.maze.is_free(&next) {
            return None;
        }
        if next == *goal {
            return Some(next);
        }

        let forced = if direction.is_diagonal() {
            let cant_move = direction
                .components()
                .iter()
                .all(|dirs| !self.maze.is_free(&(current + dirs)));
            if cant_move {
                return None;
            }
            self.compute_forced_diagonal(current, direction)
        } else {
            self.compute_forced_straight(&next, direction)
        };

        if forced.iter().any(|f| self.maze.is_free(f)) {
            return Some(next);
        }

        if direction.is_diagonal() {
            for dirt in &direction.components() {
                if self.jump_rec(&next, dirt, goal).is_some() {
                    return Some(next);
                }
            }
        }

        self.jump_rec(&next, direction, goal)
    }
}

impl ChildrenGenerator for JpsGenerator<'_> {
    fn generate_children(&self, current: &Pos, parent: Option<&Pos>) -> Vec<Child> {
        let mut natural_neighbors = self.natural_neighbors(current);

        match parent {
            None => natural_neighbors,
            Some(parent) => {
                self.prune_neighbors(current, parent, &mut natural_neighbors);
                self.do_jump(current, natural_neighbors)
            }
        }
    }
}

impl MazeChildrenGenerator for JpsGenerator<'_> {
    fn reconstruct_path(&self, path: PathRef) -> (Path, f64) {
        if path.is_empty() {
            return (Vec::new(), 0.0);
        }

        let mut result = vec![*path.first().unwrap()];
        let mut cost = 0.0;

        for (cur, next) in path.iter().tuple_windows() {
            let dir = (*next - *cur).sign();
            let cost_unit = if dir.is_diagonal() { SQRT_2 } else { 1.0 };
            let mut cur = *cur;
            while cur != *next {
                cur = cur + dir;
                result.push(cur);
                cost += cost_unit;
            }
        }

        (result, cost)
    }
}
