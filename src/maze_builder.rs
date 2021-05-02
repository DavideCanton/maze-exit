use std::collections::HashSet;

use crate::maze::{CellStatus, Maze};
use crate::position::Pos;

#[derive(Default)]
pub struct MazeBuilder {
    walls: HashSet<Pos>,
    start: Option<Pos>,
    goal: Option<Pos>,
    error: bool,
    width: Option<u32>,
    height: Option<u32>,
}

macro_rules! set_or_error {
    ($name: ident, $arg: ty) => {
        pub fn $name(mut self, arg: $arg) -> Self {
            match self.$name {
                Some(_) => self.error = true,
                None => self.$name = Some(arg),
            };
            self
        }
    };
}

impl MazeBuilder {
    pub fn new() -> Self {
        MazeBuilder::default()
    }

    set_or_error!(width, u32);
    set_or_error!(height, u32);
    set_or_error!(start, Pos);
    set_or_error!(goal, Pos);

    pub fn add_wall(mut self, pos: Pos) -> Self {
        self.walls.insert(pos);
        self
    }

    pub fn build(self) -> Option<Maze> {
        if self.error
            || self.width.is_none()
            || self.height.is_none()
            || self.start.is_none()
            || self.height.is_none()
        {
            None
        } else {
            let mut maze = Maze::new(
                self.width.unwrap(),
                self.height.unwrap(),
                self.start.unwrap(),
                self.goal.unwrap(),
            );

            for wall in self.walls {
                maze.set(wall, CellStatus::Wall);
            }

            Some(maze)
        }
    }
}
