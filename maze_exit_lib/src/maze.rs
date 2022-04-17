use std::collections::HashSet;

use crate::position::Pos;

pub struct Maze {
    walls: HashSet<Pos>,
    w: u32,
    h: u32,
    pub start: Pos,
    pub goal: Pos,
}

pub enum CellStatus {
    Wall,
    Empty,
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

    pub fn width(&self) -> u32 {
        self.w
    }

    pub fn height(&self) -> u32 {
        self.h
    }

    pub fn get(&self, pos: &Pos) -> Option<CellStatus> {
        if self.contains(pos) {
            let is_wall = self.walls.contains(pos);
            Some(if is_wall {
                CellStatus::Wall
            } else {
                CellStatus::Empty
            })
        } else {
            None
        }
    }

    pub fn contains(&self, pos: &Pos) -> bool {
        pos.x < self.w as i32 && pos.y < self.h as i32 && pos.x >= 0 && pos.y >= 0
    }

    pub fn set(&mut self, pos: Pos, status: CellStatus) -> bool {
        if self.contains(&pos) {
            match status {
                CellStatus::Empty => self.walls.remove(&pos),
                CellStatus::Wall => self.walls.insert(pos),
            };
            true
        } else {
            false
        }
    }

    pub fn is_free(&self, result: &Pos) -> bool {
        matches!(self.get(result), Some(CellStatus::Empty))
    }

    pub fn is_wall(&self, result: &Pos) -> bool {
        matches!(self.get(result), Some(CellStatus::Wall))
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
