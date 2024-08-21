use std::collections::HashSet;

use crate::position::{Position, PositionUnit};

pub struct Maze {
    walls: HashSet<Position>,
    w: u32,
    h: u32,
    start: Position,
    goal: Position,
}

impl Maze {
    pub fn new(width: u32, height: u32, start: Position, goal: Position) -> Self {
        Maze {
            w: width,
            h: height,
            goal,
            start,
            walls: HashSet::new(),
        }
    }

    pub fn start(&self) -> Position {
        self.start
    }

    pub fn goal(&self) -> Position {
        self.goal
    }

    pub fn width(&self) -> u32 {
        self.w
    }

    pub fn height(&self) -> u32 {
        self.h
    }

    pub fn valid(&self, pos: Position) -> bool {
        pos.x < self.w as PositionUnit && pos.y < self.h as PositionUnit && pos.x >= 0 && pos.y >= 0
    }

    pub fn set(&mut self, pos: Position, wall: bool) -> Result<(), String> {
        if self.valid(pos) {
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

    pub fn is_free(&self, pos: Position) -> bool {
        self.valid(pos) && !self.is_wall(pos)
    }

    pub fn is_wall(&self, pos: Position) -> bool {
        self.valid(pos) && self.walls.contains(&pos)
    }

    pub fn walls(&self) -> impl Iterator<Item = Position> + '_ {
        self.walls.iter().copied()
    }
}
