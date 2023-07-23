use std::collections::HashSet;

use crate::position::{Pos, PosUnit};

pub struct Maze {
    walls: HashSet<Pos>,
    w: u32,
    h: u32,
    start: Pos,
    goal: Pos,
}

impl Maze {
    pub fn new(width: u32, height: u32, start: Pos, goal: Pos) -> Self {
        Maze {
            w: width,
            h: height,
            goal,
            start,
            walls: HashSet::new(),
        }
    }

    pub fn start(&self) -> &Pos {
        &self.start
    }

    pub fn goal(&self) -> &Pos {
        &self.goal
    }

    pub fn width(&self) -> u32 {
        self.w
    }

    pub fn height(&self) -> u32 {
        self.h
    }

    pub fn valid(&self, pos: &Pos) -> bool {
        pos.x < self.w as PosUnit && pos.y < self.h as PosUnit && pos.x >= 0 && pos.y >= 0
    }

    pub fn set(&mut self, pos: Pos, wall: bool) -> Result<(), String> {
        if self.valid(&pos) {
            if wall {
                self.walls.insert(pos);
            } else {
                self.walls.remove(&pos);
            }
            Ok(())
        } else {
            Err(format!("Invalid position provided: {}", pos))
        }
    }

    pub fn is_free(&self, pos: &Pos) -> bool {
        self.valid(pos) && !self.is_wall(pos)
    }

    pub fn is_wall(&self, pos: &Pos) -> bool {
        self.valid(pos) && self.walls.contains(pos)
    }

    pub fn walls(&self) -> Box<impl Iterator<Item = &Pos>> {
        Box::new(self.walls.iter())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_creation() {
        let maze = Maze::new(3, 4, Pos::new(0, 0), Pos::new(2, 3));
        assert_eq!(maze.width(), 3);
        assert_eq!(maze.height(), 4);
        assert_eq!(maze.start, Pos::new(0, 0));
        assert_eq!(maze.goal, Pos::new(2, 3));
        assert_eq!(maze.walls().count(), 0);
    }
}
