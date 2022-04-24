use std::collections::HashSet;

use crate::maze::{CellStatus, Maze};
use crate::position::Pos;

#[derive(Default)]
pub struct MazeBuilder {
    walls: HashSet<Pos>,
    start: Option<Pos>,
    goal: Option<Pos>,
    errors: Vec<String>,
    width: Option<u32>,
    height: Option<u32>,
}

macro_rules! set_or_error {
    ($name: ident, $arg: ty) => {
        pub fn $name(mut self, arg: $arg) -> Self {
            match self.$name {
                Some(_) => {
                    self.errors
                        .push(format!("{} already set at {}", stringify!($name), arg))
                }
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

    pub fn build(mut self) -> Result<Maze, Vec<String>> {
        self.check_options();

        if self.errors.is_empty() {
            let mut maze = Maze::new(
                self.width.unwrap(),
                self.height.unwrap(),
                self.start.unwrap(),
                self.goal.unwrap(),
            );

            for wall in self.walls {
                maze.set(wall, CellStatus::Wall);
            }

            Ok(maze)
        } else {
            Err(self.errors)
        }
    }

    fn check_options(&mut self) {
        if self.width.is_none() {
            self.errors.push("width not set".to_owned());
        }
        if self.height.is_none() {
            self.errors.push("height not set".to_owned());
        }
        if self.start.is_none() {
            self.errors.push("start not set".to_owned());
        }
        if self.goal.is_none() {
            self.errors.push("goal not set".to_owned());
        }
    }
}
